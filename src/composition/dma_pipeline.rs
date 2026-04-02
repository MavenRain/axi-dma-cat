//! DMA pipeline composition.
//!
//! The DMA engine composes:
//! - An AXI-Lite slave interface (register access) for configuration
//! - A DMA transfer engine (read from source, write to destination)
//!
//! Categorically: `Controller ; (RegisterInterface ⊗ DataInterface)`.
//! The controller FSM sequences access to both interfaces.

use crate::axi_lite::sim::golden::RegisterFile;
use crate::axi_lite::transaction::{AxiLiteReadRequest, AxiLiteWriteRequest};
use crate::dma::register::{
    DmaControl, TransferLength,
    REG_CONTROL, REG_DEST, REG_LENGTH, REG_SOURCE,
};
use crate::dma::sim::golden::{DmaSimResult, MemoryModel, dma_transfer_golden};
use crate::dma::transaction::DmaTransferDescriptor;
use crate::error::Error;
use crate::primitives::address::AxiAddress;

/// Result of a full DMA pipeline operation.
#[derive(Debug, Clone)]
pub struct DmaPipelineResult {
    reg_file: RegisterFile,
    memory: MemoryModel,
    transfer_result: DmaSimResult,
}

impl DmaPipelineResult {
    /// The register file after the operation.
    #[must_use]
    pub fn reg_file(&self) -> &RegisterFile {
        &self.reg_file
    }

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

/// Execute a full DMA pipeline: configure via AXI-Lite writes, then
/// execute the transfer.
///
/// Steps:
/// 1. Write source address to register file
/// 2. Write destination address to register file
/// 3. Write transfer length to register file
/// 4. Write control register with start bit
/// 5. Execute the DMA transfer using the golden model
///
/// # Errors
///
/// Returns an error if any AXI-Lite operation fails.
pub fn dma_pipeline(
    reg_file: RegisterFile,
    memory: MemoryModel,
    source: AxiAddress,
    dest: AxiAddress,
    length: TransferLength,
) -> Result<DmaPipelineResult, Error> {
    // Step 1-3: Configure registers via AXI-Lite writes
    let (rf, _) = reg_file.write(
        AxiLiteWriteRequest::write32(AxiAddress::new(REG_SOURCE), source.value()),
    );
    let (rf, _) = rf.write(
        AxiLiteWriteRequest::write32(AxiAddress::new(REG_DEST), dest.value()),
    );
    let (rf, _) = rf.write(
        AxiLiteWriteRequest::write32(AxiAddress::new(REG_LENGTH), length.bytes()),
    );

    // Step 4: Start the transfer
    let (rf, _) = rf.write(
        AxiLiteWriteRequest::write32(
            AxiAddress::new(REG_CONTROL),
            DmaControl::with_start().value(),
        ),
    );

    // Step 5: Read back the configuration and execute transfer
    let (_rf_after_read, rd_src) = rf.read(
        AxiLiteReadRequest::read(AxiAddress::new(REG_SOURCE)),
    );
    let (_, rd_dst) = rf.read(
        AxiLiteReadRequest::read(AxiAddress::new(REG_DEST)),
    );
    let (_, rd_len) = rf.read(
        AxiLiteReadRequest::read(AxiAddress::new(REG_LENGTH)),
    );

    let src_addr = AxiAddress::new(u32::try_from(rd_src.data()).unwrap_or(0));
    let dst_addr = AxiAddress::new(u32::try_from(rd_dst.data()).unwrap_or(0));
    let xfer_len = TransferLength::new(u32::try_from(rd_len.data()).unwrap_or(0));

    let xfer = DmaTransferDescriptor::new(src_addr, dst_addr, xfer_len);
    let (new_memory, transfer_result) = dma_transfer_golden(memory, &xfer);

    Ok(DmaPipelineResult {
        reg_file: rf,
        memory: new_memory,
        transfer_result,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_pipeline_transfers_data() -> Result<(), Error> {
        let rf = RegisterFile::new();
        let mem = MemoryModel::new()
            .write_block(AxiAddress::new(0x1000), &[0xCA, 0xFE, 0xBA, 0xBE]);

        let result = dma_pipeline(
            rf,
            mem,
            AxiAddress::new(0x1000),
            AxiAddress::new(0x2000),
            TransferLength::new(4),
        )?;

        assert!(result.transfer_result().status().done());
        assert_eq!(
            result.memory().read_block(AxiAddress::new(0x2000), 4),
            vec![0xCA, 0xFE, 0xBA, 0xBE],
        );
        Ok(())
    }

    #[test]
    fn pipeline_zero_length_is_noop() -> Result<(), Error> {
        let rf = RegisterFile::new();
        let mem = MemoryModel::new();

        let result = dma_pipeline(
            rf,
            mem,
            AxiAddress::new(0x1000),
            AxiAddress::new(0x2000),
            TransferLength::new(0),
        )?;

        assert!(result.transfer_result().status().done());
        assert_eq!(result.transfer_result().bytes_transferred(), 0);
        Ok(())
    }
}
