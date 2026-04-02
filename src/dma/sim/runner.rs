//! `Io`-wrapped DMA simulation.

use comp_cat_rs::effect::io::Io;

use crate::dma::sim::golden::{DmaSimResult, MemoryModel, dma_transfer_golden};
use crate::dma::transaction::DmaTransferDescriptor;
use crate::error::Error;

/// Configuration for a DMA behavioral simulation.
#[derive(Debug, Clone)]
pub struct DmaSimConfig {
    memory: MemoryModel,
    descriptor: DmaTransferDescriptor,
}

impl DmaSimConfig {
    /// Create a new DMA simulation config.
    #[must_use]
    pub fn new(memory: MemoryModel, descriptor: DmaTransferDescriptor) -> Self {
        Self { memory, descriptor }
    }
}

/// Result of a DMA simulation including the final memory state.
#[derive(Debug, Clone)]
pub struct DmaSimFullResult {
    memory: MemoryModel,
    transfer_result: DmaSimResult,
}

impl DmaSimFullResult {
    /// The memory after the transfer.
    #[must_use]
    pub fn memory(&self) -> &MemoryModel {
        &self.memory
    }

    /// The transfer result.
    #[must_use]
    pub fn transfer_result(&self) -> &DmaSimResult {
        &self.transfer_result
    }
}

/// Build an `Io` that simulates a DMA transfer.
///
/// Nothing executes until [`Io::run`](comp_cat_rs::effect::io::Io::run).
#[must_use]
pub fn simulate_dma(config: DmaSimConfig) -> Io<Error, DmaSimFullResult> {
    Io::suspend(move || {
        let (memory, transfer_result) =
            dma_transfer_golden(config.memory, &config.descriptor);
        Ok(DmaSimFullResult {
            memory,
            transfer_result,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dma::register::TransferLength;
    use crate::primitives::address::AxiAddress;

    #[test]
    fn dma_sim_via_io() -> Result<(), Error> {
        let mem = MemoryModel::new()
            .write_block(AxiAddress::new(0x1000), &[0xCA, 0xFE]);
        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x1000),
            AxiAddress::new(0x2000),
            TransferLength::new(2),
        );
        let config = DmaSimConfig::new(mem, desc);
        let result = simulate_dma(config).run()?;

        assert!(result.transfer_result().status().done());
        assert_eq!(
            result.memory().read_block(AxiAddress::new(0x2000), 2),
            vec![0xCA, 0xFE],
        );
        Ok(())
    }
}
