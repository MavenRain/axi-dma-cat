//! Property-based tests for DMA transfer correctness.

use proptest::prelude::*;

use axi_dma_cat::dma::register::TransferLength;
use axi_dma_cat::dma::sim::golden::{MemoryModel, dma_transfer_golden};
use axi_dma_cat::dma::transaction::DmaTransferDescriptor;
use axi_dma_cat::primitives::address::AxiAddress;

fn arb_aligned_address() -> impl Strategy<Value = AxiAddress> {
    (0_u32..0x1000).prop_map(|a| AxiAddress::new(a * 4))
}

fn arb_data_buffer() -> impl Strategy<Value = Vec<u8>> {
    prop::collection::vec(any::<u8>(), 1..64)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Data integrity: destination matches source after transfer.
    #[test]
    fn data_integrity(
        src_base in arb_aligned_address(),
        data in arb_data_buffer(),
    ) {
        // Place source at src_base, destination at src_base + 0x10000
        let dest_base = AxiAddress::new(src_base.value().wrapping_add(0x10000));
        let length = TransferLength::new(u32::try_from(data.len()).unwrap_or(0));

        let mem = MemoryModel::new().write_block(src_base, &data);
        let desc = DmaTransferDescriptor::new(src_base, dest_base, length);
        let (mem, result) = dma_transfer_golden(mem, &desc);

        prop_assert!(result.status().done());
        prop_assert_eq!(
            mem.read_block(dest_base, length.bytes()),
            data,
        );
    }

    /// Transfer length: exactly N bytes transferred.
    #[test]
    fn transfer_length_exact(
        data in arb_data_buffer(),
    ) {
        let length = TransferLength::new(u32::try_from(data.len()).unwrap_or(0));
        let mem = MemoryModel::new().write_block(AxiAddress::new(0x0), &data);
        let desc = DmaTransferDescriptor::new(
            AxiAddress::new(0x0),
            AxiAddress::new(0x10000),
            length,
        );
        let (_mem, result) = dma_transfer_golden(mem, &desc);
        prop_assert_eq!(result.bytes_transferred(), length.bytes());
    }

    /// Zero-length transfer completes immediately.
    #[test]
    fn zero_length_is_immediate(
        src in arb_aligned_address(),
        dst in arb_aligned_address(),
    ) {
        let mem = MemoryModel::new();
        let desc = DmaTransferDescriptor::new(src, dst, TransferLength::new(0));
        let (_mem, result) = dma_transfer_golden(mem, &desc);
        prop_assert!(result.status().done());
        prop_assert_eq!(result.bytes_transferred(), 0);
    }

    /// Source equals destination: in-place copy preserves data.
    #[test]
    fn in_place_copy_preserves(
        data in arb_data_buffer(),
    ) {
        let addr = AxiAddress::new(0x5000);
        let length = TransferLength::new(u32::try_from(data.len()).unwrap_or(0));
        let mem = MemoryModel::new().write_block(addr, &data);
        let desc = DmaTransferDescriptor::new(addr, addr, length);
        let (mem, result) = dma_transfer_golden(mem, &desc);
        prop_assert!(result.status().done());
        prop_assert_eq!(mem.read_block(addr, length.bytes()), data);
    }
}
