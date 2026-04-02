//! Property-based tests for AXI-Lite protocol correctness.

use proptest::prelude::*;

use axi_dma_cat::axi_lite::sim::golden::RegisterFile;
use axi_dma_cat::axi_lite::transaction::{AxiLiteReadRequest, AxiLiteWriteRequest};
use axi_dma_cat::primitives::address::AxiAddress;
use axi_dma_cat::primitives::response::AxiResponse;

fn arb_aligned_offset() -> impl Strategy<Value = u32> {
    // 16 registers at 4-byte-aligned offsets: 0x00, 0x04, ..., 0x3C
    (0_u32..16).prop_map(|i| i * 4)
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Write-read round-trip: data written to address X reads back.
    #[test]
    fn write_read_round_trip(
        offset in arb_aligned_offset(),
        data in any::<u32>(),
    ) {
        let rf = RegisterFile::new();
        let (rf, wr) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(offset), data));
        prop_assert!(wr.resp().is_okay());
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(offset)));
        prop_assert!(rd.resp().is_okay());
        prop_assert_eq!(rd.data(), u64::from(data));
    }

    /// Response code round-trip: from_bits(to_bits(r)) == r.
    #[test]
    fn response_code_round_trip(code in prop::sample::select(vec![0_u8, 2, 3])) {
        let resp = AxiResponse::from_bits(code);
        prop_assert!(resp.is_ok());
        let resp = resp.ok().unwrap_or(AxiResponse::Okay);
        prop_assert_eq!(AxiResponse::from_bits(resp.to_bits()).ok(), Some(resp));
    }

    /// Two writes to different addresses don't interfere.
    #[test]
    fn writes_to_different_addresses_are_independent(
        offset_a in arb_aligned_offset(),
        offset_b in arb_aligned_offset(),
        data_a in any::<u32>(),
        data_b in any::<u32>(),
    ) {
        prop_assume!(offset_a != offset_b);
        let rf = RegisterFile::new();
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(offset_a), data_a));
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(offset_b), data_b));
        let (rf, rd_a) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(offset_a)));
        let (_rf, rd_b) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(offset_b)));
        prop_assert_eq!(rd_a.data(), u64::from(data_a));
        prop_assert_eq!(rd_b.data(), u64::from(data_b));
    }

    /// Unwritten address reads zero.
    #[test]
    fn unwritten_reads_zero(offset in arb_aligned_offset()) {
        let rf = RegisterFile::new();
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(offset)));
        prop_assert!(rd.resp().is_okay());
        prop_assert_eq!(rd.data(), 0);
    }
}
