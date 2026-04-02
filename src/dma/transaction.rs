//! DMA transfer descriptor.

use crate::dma::register::TransferLength;
use crate::primitives::address::AxiAddress;

/// A DMA transfer descriptor: source, destination, and length.
///
/// # Examples
///
/// ```
/// use axi_dma_cat::dma::transaction::DmaTransferDescriptor;
/// use axi_dma_cat::dma::register::TransferLength;
/// use axi_dma_cat::primitives::address::AxiAddress;
///
/// let desc = DmaTransferDescriptor::new(
///     AxiAddress::new(0x1000_0000),
///     AxiAddress::new(0x2000_0000),
///     TransferLength::new(1024),
/// );
/// assert_eq!(desc.source().value(), 0x1000_0000);
/// assert_eq!(desc.length().bytes(), 1024);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct DmaTransferDescriptor {
    source: AxiAddress,
    destination: AxiAddress,
    length: TransferLength,
}

impl DmaTransferDescriptor {
    /// Create a new transfer descriptor.
    pub fn new(source: AxiAddress, destination: AxiAddress, length: TransferLength) -> Self {
        Self {
            source,
            destination,
            length,
        }
    }

    /// The source address.
    pub fn source(&self) -> AxiAddress {
        self.source
    }

    /// The destination address.
    pub fn destination(&self) -> AxiAddress {
        self.destination
    }

    /// The transfer length.
    pub fn length(&self) -> TransferLength {
        self.length
    }

    /// Whether this is a zero-length (no-op) transfer.
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.length.is_zero()
    }
}
