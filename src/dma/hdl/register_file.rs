//! DMA register file as an AXI-Lite slave.
//!
//! Maps the DMA control/status/address/length registers to
//! AXI-Lite read/write transactions.  Uses the same 16-register
//! `AxiLiteSlave` module from the AXI-Lite HDL layer, with
//! specific offsets assigned to DMA functions.

use rust_hdl::prelude::*;

use crate::axi_lite::hdl::slave::AxiLiteSlave;

/// DMA register file: wraps an `AxiLiteSlave` and exposes the
/// DMA-specific registers as named output signals.
///
/// Register map:
/// - Reg 0 (0x00): Control
/// - Reg 1 (0x04): Status
/// - Reg 2 (0x08): Source Address
/// - Reg 3 (0x0C): Destination Address
/// - Reg 4 (0x10): Transfer Length
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct DmaRegisterFile {
    /// System clock.
    pub clock: Signal<In, Clock>,
    // AXI-Lite slave interface (directly exposed)
    /// Write address.
    pub awaddr: Signal<In, Bits<32>>,
    /// Write address valid.
    pub awvalid: Signal<In, Bit>,
    /// Write address ready.
    pub awready: Signal<Out, Bit>,
    /// Write data.
    pub wdata: Signal<In, Bits<32>>,
    /// Write strobe.
    pub wstrb: Signal<In, Bits<4>>,
    /// Write data valid.
    pub wvalid: Signal<In, Bit>,
    /// Write data ready.
    pub wready: Signal<Out, Bit>,
    /// Write response.
    pub bresp: Signal<Out, Bits<2>>,
    /// Write response valid.
    pub bvalid: Signal<Out, Bit>,
    /// Write response ready.
    pub bready: Signal<In, Bit>,
    /// Read address.
    pub araddr: Signal<In, Bits<32>>,
    /// Read address valid.
    pub arvalid: Signal<In, Bit>,
    /// Read address ready.
    pub arready: Signal<Out, Bit>,
    /// Read data.
    pub rdata: Signal<Out, Bits<32>>,
    /// Read response.
    pub rresp: Signal<Out, Bits<2>>,
    /// Read data valid.
    pub rvalid: Signal<Out, Bit>,
    /// Read data ready.
    pub rready: Signal<In, Bit>,
    // Internal AXI-Lite slave
    slave: AxiLiteSlave,
}

impl Logic for DmaRegisterFile {
    fn update(&mut self) {
        // Wire clock
        self.slave.clock.next = self.clock.val();

        // Wire AXI-Lite signals through to internal slave
        self.slave.awaddr.next = self.awaddr.val();
        self.slave.awvalid.next = self.awvalid.val();
        self.awready.next = self.slave.awready.val();

        self.slave.wdata.next = self.wdata.val();
        self.slave.wstrb.next = self.wstrb.val();
        self.slave.wvalid.next = self.wvalid.val();
        self.wready.next = self.slave.wready.val();

        self.bresp.next = self.slave.bresp.val();
        self.bvalid.next = self.slave.bvalid.val();
        self.slave.bready.next = self.bready.val();

        self.slave.araddr.next = self.araddr.val();
        self.slave.arvalid.next = self.arvalid.val();
        self.arready.next = self.slave.arready.val();

        self.rdata.next = self.slave.rdata.val();
        self.rresp.next = self.slave.rresp.val();
        self.rvalid.next = self.slave.rvalid.val();
        self.slave.rready.next = self.rready.val();
    }
}
