//! Tensor product for AXI-Lite channel independence.
//!
//! Per the AMBA spec, the AW and W channels of a write transaction
//! are independent: WVALID need not wait for AWREADY, and vice versa.
//! This independence is the monoidal tensor product `AW ⊗ W`.
//!
//! The full write pipeline is `(AW ⊗ W) ; B`:
//! the address and data channels execute independently (tensor),
//! then the response channel follows sequentially (composition).
//!
//! The read pipeline is purely sequential: `AR ; R`.

use crate::axi_lite::transaction::{
    AxiLiteReadRequest, AxiLiteReadResult,
    AxiLiteWriteRequest, AxiLiteWriteResult,
};
use crate::axi_lite::sim::golden::RegisterFile;
use crate::error::Error;

/// Tensor product of two independent AXI-Lite write transactions.
///
/// Models two writes to independent addresses executing in parallel.
/// In hardware, only one transaction is active at a time on a single
/// bus, but categorically they are independent (order irrelevant).
///
/// # Errors
///
/// Returns an error if either write targets an invalid address.
pub fn axi_write_tensor(
    rf: RegisterFile,
    a: AxiLiteWriteRequest,
    b: AxiLiteWriteRequest,
) -> Result<(RegisterFile, AxiLiteWriteResult, AxiLiteWriteResult), Error> {
    let (rf, result_a) = rf.write(a);
    let (rf, result_b) = rf.write(b);
    Ok((rf, result_a, result_b))
}

/// Sequential composition: write then read at the same address.
///
/// Models the `(Write ; Read)` pipeline: the write must complete
/// before the read can observe the new value.
///
/// # Errors
///
/// Returns an error if the address is invalid.
pub fn axi_write_then_read(
    rf: RegisterFile,
    write: AxiLiteWriteRequest,
    read: AxiLiteReadRequest,
) -> Result<(RegisterFile, AxiLiteWriteResult, AxiLiteReadResult), Error> {
    let (rf, wr) = rf.write(write);
    let (rf, rd) = rf.read(read);
    Ok((rf, wr, rd))
}

/// Braiding (swap): reorder two independent transactions.
///
/// Symmetric monoidal braiding: `braid(A, B) = (B, A)`.
#[must_use]
pub fn axi_braid<A, B>(pair: (A, B)) -> (B, A) {
    (pair.1, pair.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::address::AxiAddress;

    #[test]
    fn tensor_writes_are_independent() -> Result<(), Error> {
        let rf = RegisterFile::new();
        let wr_a = AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0xAA);
        let wr_b = AxiLiteWriteRequest::write32(AxiAddress::new(0x04), 0xBB);

        let (rf, res_a, res_b) = axi_write_tensor(rf, wr_a, wr_b)?;
        assert!(res_a.resp().is_okay());
        assert!(res_b.resp().is_okay());
        assert_eq!(rf.get(0x00), 0xAA);
        assert_eq!(rf.get(0x04), 0xBB);
        Ok(())
    }

    #[test]
    fn write_then_read_observes_new_value() -> Result<(), Error> {
        let rf = RegisterFile::new();
        let write = AxiLiteWriteRequest::write32(AxiAddress::new(0x10), 0xDEAD);
        let read = AxiLiteReadRequest::read(AxiAddress::new(0x10));

        let (_rf, wr, rd) = axi_write_then_read(rf, write, read)?;
        assert!(wr.resp().is_okay());
        assert!(rd.resp().is_okay());
        assert_eq!(rd.data(), 0xDEAD);
        Ok(())
    }

    #[test]
    fn braid_is_involution() {
        let pair = (1_u32, 2_u32);
        assert_eq!(axi_braid(axi_braid(pair)), pair);
    }
}
