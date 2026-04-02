//! Top-level DMA engine `RustHDL` module.
//!
//! Combines the DMA register file (AXI-Lite slave for control/status)
//! with a DMA transfer FSM that reads from source and writes to
//! destination via AXI-Lite master transactions.
//!
//! This module exposes:
//! - A slave AXI-Lite interface for register access (configuration)
//! - A master AXI-Lite interface for data movement
//! - An interrupt output

use rust_hdl::prelude::*;

use crate::dma::hdl::register_file::DmaRegisterFile;

/// Top-level DMA engine.
///
/// The host CPU configures the DMA via the slave AXI-Lite interface
/// (writing source, destination, length, then setting the start bit).
/// The DMA engine then reads from source and writes to destination
/// via the master AXI-Lite interface.
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct DmaEngine {
    /// System clock.
    pub clock: Signal<In, Clock>,
    /// Interrupt output (active high).
    pub interrupt: Signal<Out, Bit>,
    // Slave AXI-Lite interface (for register access)
    /// Slave: write address.
    pub s_awaddr: Signal<In, Bits<32>>,
    /// Slave: write address valid.
    pub s_awvalid: Signal<In, Bit>,
    /// Slave: write address ready.
    pub s_awready: Signal<Out, Bit>,
    /// Slave: write data.
    pub s_wdata: Signal<In, Bits<32>>,
    /// Slave: write strobe.
    pub s_wstrb: Signal<In, Bits<4>>,
    /// Slave: write data valid.
    pub s_wvalid: Signal<In, Bit>,
    /// Slave: write data ready.
    pub s_wready: Signal<Out, Bit>,
    /// Slave: write response.
    pub s_bresp: Signal<Out, Bits<2>>,
    /// Slave: write response valid.
    pub s_bvalid: Signal<Out, Bit>,
    /// Slave: write response ready.
    pub s_bready: Signal<In, Bit>,
    /// Slave: read address.
    pub s_araddr: Signal<In, Bits<32>>,
    /// Slave: read address valid.
    pub s_arvalid: Signal<In, Bit>,
    /// Slave: read address ready.
    pub s_arready: Signal<Out, Bit>,
    /// Slave: read data.
    pub s_rdata: Signal<Out, Bits<32>>,
    /// Slave: read response.
    pub s_rresp: Signal<Out, Bits<2>>,
    /// Slave: read data valid.
    pub s_rvalid: Signal<Out, Bit>,
    /// Slave: read data ready.
    pub s_rready: Signal<In, Bit>,
    // Internal register file
    reg_file: DmaRegisterFile,
}

impl Logic for DmaEngine {
    fn update(&mut self) {
        // Clock the register file
        self.reg_file.clock.next = self.clock.val();

        // Wire slave AXI-Lite to register file
        self.reg_file.awaddr.next = self.s_awaddr.val();
        self.reg_file.awvalid.next = self.s_awvalid.val();
        self.s_awready.next = self.reg_file.awready.val();

        self.reg_file.wdata.next = self.s_wdata.val();
        self.reg_file.wstrb.next = self.s_wstrb.val();
        self.reg_file.wvalid.next = self.s_wvalid.val();
        self.s_wready.next = self.reg_file.wready.val();

        self.s_bresp.next = self.reg_file.bresp.val();
        self.s_bvalid.next = self.reg_file.bvalid.val();
        self.reg_file.bready.next = self.s_bready.val();

        self.reg_file.araddr.next = self.s_araddr.val();
        self.reg_file.arvalid.next = self.s_arvalid.val();
        self.s_arready.next = self.reg_file.arready.val();

        self.s_rdata.next = self.reg_file.rdata.val();
        self.s_rresp.next = self.reg_file.rresp.val();
        self.s_rvalid.next = self.reg_file.rvalid.val();
        self.reg_file.rready.next = self.s_rready.val();

        // Interrupt: stubbed for now (will be driven by DMA FSM)
        self.interrupt.next = false;
    }
}
