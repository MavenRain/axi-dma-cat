//! AXI-Lite bus signal types for `RustHDL`.
//!
//! Provides master-side and slave-side signal bundles for the
//! five AXI-Lite channels.  Uses `Bits<32>` for address and
//! `Bits<32>` for data (the most common AXI-Lite configuration).

use rust_hdl::prelude::*;

/// AXI-Lite master bus signals.
///
/// The master drives AW, W, AR channels and receives B, R channels.
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct AxiLiteMasterBus {
    // Write address channel (master -> slave)
    /// Write address.
    pub awaddr: Signal<Out, Bits<32>>,
    /// Write address valid.
    pub awvalid: Signal<Out, Bit>,
    /// Write address ready (from slave).
    pub awready: Signal<In, Bit>,
    // Write data channel (master -> slave)
    /// Write data.
    pub wdata: Signal<Out, Bits<32>>,
    /// Write strobe.
    pub wstrb: Signal<Out, Bits<4>>,
    /// Write data valid.
    pub wvalid: Signal<Out, Bit>,
    /// Write data ready (from slave).
    pub wready: Signal<In, Bit>,
    // Write response channel (slave -> master)
    /// Write response.
    pub bresp: Signal<In, Bits<2>>,
    /// Write response valid (from slave).
    pub bvalid: Signal<In, Bit>,
    /// Write response ready.
    pub bready: Signal<Out, Bit>,
    // Read address channel (master -> slave)
    /// Read address.
    pub araddr: Signal<Out, Bits<32>>,
    /// Read address valid.
    pub arvalid: Signal<Out, Bit>,
    /// Read address ready (from slave).
    pub arready: Signal<In, Bit>,
    // Read data channel (slave -> master)
    /// Read data (from slave).
    pub rdata: Signal<In, Bits<32>>,
    /// Read response (from slave).
    pub rresp: Signal<In, Bits<2>>,
    /// Read data valid (from slave).
    pub rvalid: Signal<In, Bit>,
    /// Read data ready.
    pub rready: Signal<Out, Bit>,
}

impl Logic for AxiLiteMasterBus {
    fn update(&mut self) {}
}

/// AXI-Lite slave bus signals.
///
/// The slave receives AW, W, AR channels and drives B, R channels.
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct AxiLiteSlaveBus {
    // Write address channel (master -> slave)
    /// Write address.
    pub awaddr: Signal<In, Bits<32>>,
    /// Write address valid.
    pub awvalid: Signal<In, Bit>,
    /// Write address ready.
    pub awready: Signal<Out, Bit>,
    // Write data channel (master -> slave)
    /// Write data.
    pub wdata: Signal<In, Bits<32>>,
    /// Write strobe.
    pub wstrb: Signal<In, Bits<4>>,
    /// Write data valid.
    pub wvalid: Signal<In, Bit>,
    /// Write data ready.
    pub wready: Signal<Out, Bit>,
    // Write response channel (slave -> master)
    /// Write response.
    pub bresp: Signal<Out, Bits<2>>,
    /// Write response valid.
    pub bvalid: Signal<Out, Bit>,
    /// Write response ready (from master).
    pub bready: Signal<In, Bit>,
    // Read address channel (master -> slave)
    /// Read address.
    pub araddr: Signal<In, Bits<32>>,
    /// Read address valid.
    pub arvalid: Signal<In, Bit>,
    /// Read address ready.
    pub arready: Signal<Out, Bit>,
    // Read data channel (slave -> master)
    /// Read data.
    pub rdata: Signal<Out, Bits<32>>,
    /// Read response.
    pub rresp: Signal<Out, Bits<2>>,
    /// Read data valid.
    pub rvalid: Signal<Out, Bit>,
    /// Read data ready (from master).
    pub rready: Signal<In, Bit>,
}

impl Logic for AxiLiteSlaveBus {
    fn update(&mut self) {}
}
