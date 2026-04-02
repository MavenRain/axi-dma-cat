//! AXI-Lite channel signal bundles (pure, no HDL).
//!
//! Each of the five AXI-Lite channels is represented as a simple
//! struct carrying the channel's payload fields.  These are used
//! by the golden model and transaction types; the HDL layer has
//! its own signal-level representations.

use crate::primitives::address::AxiAddress;
use crate::primitives::protection::AxiProt;
use crate::primitives::response::AxiResponse;
use crate::primitives::strobe::WriteStrobe;

/// Write address channel payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct WriteAddressPayload {
    addr: AxiAddress,
    prot: AxiProt,
}

impl WriteAddressPayload {
    /// Create a new write address payload.
    pub fn new(addr: AxiAddress, prot: AxiProt) -> Self {
        Self { addr, prot }
    }

    /// The address.
    pub fn addr(&self) -> AxiAddress {
        self.addr
    }

    /// The protection type.
    pub fn prot(&self) -> AxiProt {
        self.prot
    }
}

/// Write data channel payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct WriteDataPayload {
    data: u64,
    strobe: WriteStrobe,
}

impl WriteDataPayload {
    /// Create a new write data payload.
    pub fn new(data: u64, strobe: WriteStrobe) -> Self {
        Self { data, strobe }
    }

    /// The data value (up to 64 bits).
    #[must_use]
    pub fn data(&self) -> u64 {
        self.data
    }

    /// The write strobe.
    pub fn strobe(&self) -> WriteStrobe {
        self.strobe
    }
}

/// Write response channel payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct WriteResponsePayload {
    resp: AxiResponse,
}

impl WriteResponsePayload {
    /// Create a new write response payload.
    pub fn new(resp: AxiResponse) -> Self {
        Self { resp }
    }

    /// The response code.
    #[must_use]
    pub fn resp(&self) -> AxiResponse {
        self.resp
    }
}

/// Read address channel payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct ReadAddressPayload {
    addr: AxiAddress,
    prot: AxiProt,
}

impl ReadAddressPayload {
    /// Create a new read address payload.
    pub fn new(addr: AxiAddress, prot: AxiProt) -> Self {
        Self { addr, prot }
    }

    /// The address.
    pub fn addr(&self) -> AxiAddress {
        self.addr
    }

    /// The protection type.
    pub fn prot(&self) -> AxiProt {
        self.prot
    }
}

/// Read data channel payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct ReadDataPayload {
    data: u64,
    resp: AxiResponse,
}

impl ReadDataPayload {
    /// Create a new read data payload.
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
