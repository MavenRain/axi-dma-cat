//! Pure AXI-Lite golden model.
//!
//! Models a simple register file: writes store data at an address,
//! reads retrieve it.  No side effects, no HDL.

use std::collections::BTreeMap;

use crate::primitives::address::AxiAddress;
use crate::primitives::response::AxiResponse;
use crate::axi_lite::transaction::{
    AxiLiteReadRequest, AxiLiteReadResult,
    AxiLiteWriteRequest, AxiLiteWriteResult,
};

/// A pure register file model.
///
/// Maps aligned 32-bit addresses to 64-bit values.
/// Reads from unwritten addresses return zero with OKAY.
/// All writes succeed with OKAY.
///
/// # Examples
///
/// ```
/// use axi_dma_cat::axi_lite::sim::golden::RegisterFile;
/// use axi_dma_cat::axi_lite::transaction::{AxiLiteWriteRequest, AxiLiteReadRequest};
/// use axi_dma_cat::primitives::address::AxiAddress;
///
/// let rf = RegisterFile::new();
/// let (rf, wr) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x100), 0xDEAD_BEEF));
/// let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x100)));
/// assert_eq!(rd.data(), 0xDEAD_BEEF);
/// ```
#[derive(Debug, Clone)]
pub struct RegisterFile {
    regs: BTreeMap<u32, u64>,
    base: AxiAddress,
    size: u32,
}

impl RegisterFile {
    /// Create a new register file with default 4 KB address range at base 0.
    #[must_use]
    pub fn new() -> Self {
        Self {
            regs: BTreeMap::new(),
            base: AxiAddress::ZERO,
            size: 4096,
        }
    }

    /// Create a register file with a specific base address and size.
    #[must_use]
    pub fn with_range(base: AxiAddress, size: u32) -> Self {
        Self {
            regs: BTreeMap::new(),
            base,
            size,
        }
    }

    /// Process a write request, returning the updated register file
    /// and the write result.
    ///
    /// Takes ownership and returns a new `RegisterFile` (immutable style).
    /// Process a write request, returning the updated register file
    /// and the write result.
    pub fn write(self, req: AxiLiteWriteRequest) -> (Self, AxiLiteWriteResult) {
        let addr = req.addr().value();
        let base = self.base.value();

        if addr < base || addr >= base.saturating_add(self.size) {
            (self, AxiLiteWriteResult::new(AxiResponse::DecErr))
        } else {
            let offset = addr.wrapping_sub(base);
            let regs = {
                let mut r = self.regs;
                r.insert(offset, req.data());
                r
            };
            (
                Self { regs, base: self.base, size: self.size },
                AxiLiteWriteResult::new(AxiResponse::Okay),
            )
        }
    }

    /// Process a read request, returning the (unchanged) register file
    /// and the read result.
    /// Process a read request, returning the register file and read result.
    pub fn read(&self, req: AxiLiteReadRequest) -> (Self, AxiLiteReadResult) {
        let addr = req.addr().value();
        let base = self.base.value();

        if addr < base || addr >= base.saturating_add(self.size) {
            (self.clone(), AxiLiteReadResult::new(0, AxiResponse::DecErr))
        } else {
            let offset = addr.wrapping_sub(base);
            let data = self.regs.get(&offset).copied().unwrap_or(0);
            (self.clone(), AxiLiteReadResult::new(data, AxiResponse::Okay))
        }
    }

    /// Get the value at a given address offset (relative to base).
    #[must_use]
    pub fn get(&self, offset: u32) -> u64 {
        self.regs.get(&offset).copied().unwrap_or(0)
    }
}

impl Default for RegisterFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn write_then_read_returns_same_data() {
        let rf = RegisterFile::new();
        let (rf, wr) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x10), 0xCAFE));
        assert!(wr.resp().is_okay());
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x10)));
        assert!(rd.resp().is_okay());
        assert_eq!(rd.data(), 0xCAFE);
    }

    #[test]
    fn unwritten_address_reads_zero() {
        let rf = RegisterFile::new();
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x20)));
        assert!(rd.resp().is_okay());
        assert_eq!(rd.data(), 0);
    }

    #[test]
    fn out_of_range_write_returns_decerr() {
        let rf = RegisterFile::with_range(AxiAddress::new(0x1000), 256);
        let (_rf, wr) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0xFF));
        assert_eq!(wr.resp(), AxiResponse::DecErr);
    }

    #[test]
    fn out_of_range_read_returns_decerr() {
        let rf = RegisterFile::with_range(AxiAddress::new(0x1000), 256);
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x2000)));
        assert_eq!(rd.resp(), AxiResponse::DecErr);
    }

    #[test]
    fn overwrite_replaces_value() {
        let rf = RegisterFile::new();
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0x111));
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0x222));
        let (_rf, rd) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x00)));
        assert_eq!(rd.data(), 0x222);
    }

    #[test]
    fn multiple_addresses_independent() {
        let rf = RegisterFile::new();
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0xAA));
        let (rf, _) = rf.write(AxiLiteWriteRequest::write32(AxiAddress::new(0x04), 0xBB));
        let (rf, r0) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x00)));
        let (_rf, r4) = rf.read(AxiLiteReadRequest::read(AxiAddress::new(0x04)));
        assert_eq!(r0.data(), 0xAA);
        assert_eq!(r4.data(), 0xBB);
    }
}
