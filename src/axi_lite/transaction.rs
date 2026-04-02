//! AXI-Lite transaction and result types.

use crate::primitives::address::AxiAddress;
use crate::primitives::protection::AxiProt;
use crate::primitives::response::AxiResponse;
use crate::primitives::strobe::WriteStrobe;

/// An AXI-Lite write transaction request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct AxiLiteWriteRequest {
    addr: AxiAddress,
    data: u64,
    strobe: WriteStrobe,
    prot: AxiProt,
}

impl AxiLiteWriteRequest {
    /// Create a new write request.
    pub fn new(addr: AxiAddress, data: u64, strobe: WriteStrobe, prot: AxiProt) -> Self {
        Self { addr, data, strobe, prot }
    }

    /// Convenience: 32-bit write with full strobe.
    pub fn write32(addr: AxiAddress, data: u32) -> Self {
        Self {
            addr,
            data: u64::from(data),
            strobe: WriteStrobe::ALL_32,
            prot: AxiProt::DEFAULT,
        }
    }

    /// The target address.
    pub fn addr(&self) -> AxiAddress {
        self.addr
    }

    /// The data value.
    #[must_use]
    pub fn data(&self) -> u64 {
        self.data
    }

    /// The write strobe.
    pub fn strobe(&self) -> WriteStrobe {
        self.strobe
    }

    /// The protection type.
    pub fn prot(&self) -> AxiProt {
        self.prot
    }
}

/// An AXI-Lite write transaction result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct AxiLiteWriteResult {
    resp: AxiResponse,
}

impl AxiLiteWriteResult {
    /// Create a new write result.
    pub fn new(resp: AxiResponse) -> Self {
        Self { resp }
    }

    /// The response code.
    #[must_use]
    pub fn resp(&self) -> AxiResponse {
        self.resp
    }
}

/// An AXI-Lite read transaction request.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct AxiLiteReadRequest {
    addr: AxiAddress,
    prot: AxiProt,
}

impl AxiLiteReadRequest {
    /// Create a new read request.
    pub fn new(addr: AxiAddress, prot: AxiProt) -> Self {
        Self { addr, prot }
    }

    /// Convenience: read with default protection.
    pub fn read(addr: AxiAddress) -> Self {
        Self { addr, prot: AxiProt::DEFAULT }
    }

    /// The target address.
    pub fn addr(&self) -> AxiAddress {
        self.addr
    }

    /// The protection type.
    pub fn prot(&self) -> AxiProt {
        self.prot
    }
}

/// An AXI-Lite read transaction result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct AxiLiteReadResult {
    data: u64,
    resp: AxiResponse,
}

impl AxiLiteReadResult {
    /// Create a new read result.
    pub fn new(data: u64, resp: AxiResponse) -> Self {
        Self { data, resp }
    }

    /// The data value.
    #[must_use]
    pub fn data(&self) -> u64 {
        self.data
    }

    /// The response code.
    #[must_use]
    pub fn resp(&self) -> AxiResponse {
        self.resp
    }
}
