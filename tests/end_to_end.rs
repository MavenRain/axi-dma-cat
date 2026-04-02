//! End-to-end integration tests: full DMA pipeline via AXI-Lite.

use axi_dma_cat::axi_lite::sim::golden::RegisterFile;
use axi_dma_cat::axi_lite::sim::runner::{AxiLiteOp, AxiLiteOpResult, simulate_axi_lite};
use axi_dma_cat::axi_lite::transaction::{AxiLiteReadRequest, AxiLiteWriteRequest};
use axi_dma_cat::composition::dma_pipeline::dma_pipeline;
use axi_dma_cat::dma::register::TransferLength;
use axi_dma_cat::dma::sim::golden::MemoryModel;
use axi_dma_cat::dma::sim::runner::{DmaSimConfig, simulate_dma};
use axi_dma_cat::dma::transaction::DmaTransferDescriptor;
use axi_dma_cat::error::Error;
use axi_dma_cat::primitives::address::AxiAddress;

#[test]
fn axi_lite_write_read_sequence_via_io() -> Result<(), Error> {
    let ops = vec![
        AxiLiteOp::Write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0x1111)),
        AxiLiteOp::Write(AxiLiteWriteRequest::write32(AxiAddress::new(0x04), 0x2222)),
        AxiLiteOp::Read(AxiLiteReadRequest::read(AxiAddress::new(0x00))),
        AxiLiteOp::Read(AxiLiteReadRequest::read(AxiAddress::new(0x04))),
    ];
    let results = simulate_axi_lite(ops).run()?;

    let rd0 = results.get(2).and_then(|r| match r {
        AxiLiteOpResult::Read(rd) => Some(rd.data()),
        AxiLiteOpResult::Write(_) => None,
    });
    let rd4 = results.get(3).and_then(|r| match r {
        AxiLiteOpResult::Read(rd) => Some(rd.data()),
        AxiLiteOpResult::Write(_) => None,
    });
    assert_eq!(rd0, Some(0x1111));
    assert_eq!(rd4, Some(0x2222));
    Ok(())
}

#[test]
fn dma_transfer_via_io() -> Result<(), Error> {
    let mem = MemoryModel::new()
        .write_block(AxiAddress::new(0x8000), &[0xDE, 0xAD, 0xBE, 0xEF]);
    let desc = DmaTransferDescriptor::new(
        AxiAddress::new(0x8000),
        AxiAddress::new(0x9000),
        TransferLength::new(4),
    );
    let config = DmaSimConfig::new(mem, desc);
    let result = simulate_dma(config).run()?;

    assert!(result.transfer_result().status().done());
    assert_eq!(
        result.memory().read_block(AxiAddress::new(0x9000), 4),
        vec![0xDE, 0xAD, 0xBE, 0xEF],
    );
    Ok(())
}

#[test]
fn full_dma_pipeline_configure_and_transfer() -> Result<(), Error> {
    let rf = RegisterFile::new();
    let mem = MemoryModel::new()
        .write_block(AxiAddress::new(0x4000), &[1, 2, 3, 4, 5, 6, 7, 8]);

    let result = dma_pipeline(
        rf,
        mem,
        AxiAddress::new(0x4000),
        AxiAddress::new(0x5000),
        TransferLength::new(8),
    )?;

    assert!(result.transfer_result().status().done());
    assert_eq!(
        result.memory().read_block(AxiAddress::new(0x5000), 8),
        vec![1, 2, 3, 4, 5, 6, 7, 8],
    );
    Ok(())
}
