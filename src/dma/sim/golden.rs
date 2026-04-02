//! Pure DMA golden model.
//!
//! Models DMA transfers as simple memory copies.  The "memory"
//! is a flat `BTreeMap` from address to byte.

use std::collections::BTreeMap;

use crate::dma::register::DmaStatus;
use crate::dma::transaction::DmaTransferDescriptor;
use crate::primitives::address::AxiAddress;

/// Result of a DMA transfer simulation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DmaSimResult {
    status: DmaStatus,
    bytes_transferred: u32,
}

impl DmaSimResult {
    /// The final DMA status.
    pub fn status(&self) -> DmaStatus {
        self.status
    }

    /// Number of bytes transferred.
    #[must_use]
    pub fn bytes_transferred(&self) -> u32 {
        self.bytes_transferred
    }
}

/// A simple memory model (byte-addressable).
#[derive(Debug, Clone)]
pub struct MemoryModel {
    data: BTreeMap<u32, u8>,
}

impl MemoryModel {
    /// Create an empty memory.
    #[must_use]
    pub fn new() -> Self {
        Self {
            data: BTreeMap::new(),
        }
    }

    /// Write a block of bytes starting at `base`.
    #[must_use]
    pub fn write_block(self, base: AxiAddress, bytes: &[u8]) -> Self {
        let data = bytes.iter().enumerate().fold(self.data, |mut m, (i, &b)| {
            let addr = base.value().wrapping_add(u32::try_from(i).unwrap_or(0));
            m.insert(addr, b);
            m
        });
        Self { data }
    }

    /// Read a block of bytes starting at `base`.
    #[must_use]
    pub fn read_block(&self, base: AxiAddress, length: u32) -> Vec<u8> {
        (0..length)
            .map(|i| {
                let addr = base.value().wrapping_add(i);
                self.data.get(&addr).copied().unwrap_or(0)
            })
            .collect()
    }

    /// Read a single byte.
    #[must_use]
    pub fn read_byte(&self, addr: AxiAddress) -> u8 {
        self.data.get(&addr.value()).copied().unwrap_or(0)
    }
}

impl Default for MemoryModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Execute a DMA transfer on the memory model (pure, no side effects).
///
/// Copies `length` bytes from `source` to `destination`.
/// Returns the updated memory and the transfer result.
///
/// # Examples
///
/// ```
/// use axi_dma_cat::dma::sim::golden::{MemoryModel, dma_transfer_golden};
/// use axi_dma_cat::dma::transaction::DmaTransferDescriptor;
/// use axi_dma_cat::dma::register::TransferLength;
/// use axi_dma_cat::primitives::address::AxiAddress;
///
/// let mem = MemoryModel::new()
///     .write_block(AxiAddress::new(0x1000), &[0xDE, 0xAD, 0xBE, 0xEF]);
///
/// let desc = DmaTransferDescriptor::new(
///     AxiAddress::new(0x1000),
///     AxiAddress::new(0x2000),
///     TransferLength::new(4),
/// );
///
/// let (mem, result) = dma_transfer_golden(mem, &desc);
/// assert!(result.status().done());
/// assert_eq!(mem.read_block(AxiAddress::new(0x2000), 4), vec![0xDE, 0xAD, 0xBE, 0xEF]);
/// ```
#[must_use]
pub fn dma_transfer_golden(
    memory: MemoryModel,
    desc: &DmaTransferDescriptor,
) -> (MemoryModel, DmaSimResult) {
    let length = desc.length().bytes();

    if desc.is_noop() {
        (
            memory,
            DmaSimResult {
                status: DmaStatus::done_status(),
                bytes_transferred: 0,
            },
        )
    } else {
        // Read source bytes
        let source_data = memory.read_block(desc.source(), length);

        // Write to destination
        let new_memory = memory.write_block(desc.destination(), &source_data);

        (
            new_memory,
            DmaSimResult {
                status: DmaStatus::done_status(),
                bytes_transferred: length,
            },
        )
    }
}

/// Execute a sequence of DMA transfers.
#[must_use]
pub fn dma_transfer_chain(
    memory: MemoryModel,
    descriptors: &[DmaTransferDescriptor],
) -> (MemoryModel, Vec<DmaSimResult>) {
    descriptors.iter().fold(
        (memory, Vec::with_capacity(descriptors.len())),
        |(mem, results), desc| {
            let (new_mem, result) = dma_transfer_golden(mem, desc);
            let new_results = {
                let mut r = results;
                r.push(result);
                r
            };
            (new_mem, new_results)
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dma::register::TransferLength;

    #[test]
    fn basic_transfer() {
        let mem = MemoryModel::new()
            .write_block(AxiAddress::new(0x100), &[1, 2, 3, 4]);

        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x100),
            AxiAddress::new(0x200),
            TransferLength::new(4),
        );

        let (mem, result) = dma_transfer_golden(mem, &desc);
        assert!(result.status().done());
        assert_eq!(result.bytes_transferred(), 4);
        assert_eq!(mem.read_block(AxiAddress::new(0x200), 4), vec![1, 2, 3, 4]);
    }

    #[test]
    fn zero_length_transfer_is_done() {
        let mem = MemoryModel::new();
        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x100),
            AxiAddress::new(0x200),
            TransferLength::new(0),
        );

        let (_mem, result) = dma_transfer_golden(mem, &desc);
        assert!(result.status().done());
        assert_eq!(result.bytes_transferred(), 0);
    }

    #[test]
    fn uninitialized_source_reads_zero() {
        let mem = MemoryModel::new();
        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x100),
            AxiAddress::new(0x200),
            TransferLength::new(4),
        );

        let (mem, _) = dma_transfer_golden(mem, &desc);
        assert_eq!(mem.read_block(AxiAddress::new(0x200), 4), vec![0, 0, 0, 0]);
    }

    #[test]
    fn in_place_copy_works() {
        let mem = MemoryModel::new()
            .write_block(AxiAddress::new(0x100), &[0xAA, 0xBB]);

        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x100),
            AxiAddress::new(0x100),
            TransferLength::new(2),
        );

        let (mem, result) = dma_transfer_golden(mem, &desc);
        assert!(result.status().done());
        assert_eq!(mem.read_block(AxiAddress::new(0x100), 2), vec![0xAA, 0xBB]);
    }

    #[test]
    fn chain_of_transfers() {
        let mem = MemoryModel::new()
            .write_block(AxiAddress::new(0x000), &[0x11, 0x22]);

        let descs = [
            DmaTransferDescriptor::new(
                AxiAddress::new(0x000),
                AxiAddress::new(0x100),
                TransferLength::new(2),
            ),
            DmaTransferDescriptor::new(
                AxiAddress::new(0x100),
                AxiAddress::new(0x200),
                TransferLength::new(2),
            ),
        ];

        let (mem, results) = dma_transfer_chain(mem, &descs);
        assert!(results.iter().all(|r| r.status().done()));
        assert_eq!(mem.read_block(AxiAddress::new(0x200), 2), vec![0x11, 0x22]);
    }
}
