//! AXI-Lite master `RustHDL` module.
//!
//! Performs single write and read transactions via the AXI-Lite
//! handshaking protocol.

use rust_hdl::prelude::*;

fn bits4_to_u64(b: Bits<4>) -> u64 {
    (0..4_usize).fold(0_u64, |acc, i| acc | (u64::from(b.get_bit(i)) << i))
}

/// AXI-Lite master controller.
///
/// To write: set `wr_addr`, `wr_data`, assert `wr_start`.
/// To read: set `rd_addr`, assert `rd_start`.
/// When `done` is pulsed, the operation is complete.
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct AxiLiteMaster {
    /// System clock.
    pub clock: Signal<In, Clock>,
    // Write interface
    /// Write address input.
    pub wr_addr: Signal<In, Bits<32>>,
    /// Write data input.
    pub wr_data: Signal<In, Bits<32>>,
    /// Assert to start a write.
    pub wr_start: Signal<In, Bit>,
    // Read interface
    /// Read address input.
    pub rd_addr: Signal<In, Bits<32>>,
    /// Assert to start a read.
    pub rd_start: Signal<In, Bit>,
    /// Read data output (valid when `done` and last op was read).
    pub rd_data: Signal<Out, Bits<32>>,
    // Status
    /// Pulsed when operation completes.
    pub done: Signal<Out, Bit>,
    /// Whether the master is busy.
    pub busy: Signal<Out, Bit>,
    // AXI-Lite bus signals (directly exposed)
    /// Write address.
    pub awaddr: Signal<Out, Bits<32>>,
    /// Write address valid.
    pub awvalid: Signal<Out, Bit>,
    /// Write address ready.
    pub awready: Signal<In, Bit>,
    /// Write data.
    pub wdata: Signal<Out, Bits<32>>,
    /// Write strobe.
    pub wstrb: Signal<Out, Bits<4>>,
    /// Write valid.
    pub wvalid: Signal<Out, Bit>,
    /// Write ready.
    pub wready: Signal<In, Bit>,
    /// Write response.
    pub bresp: Signal<In, Bits<2>>,
    /// Write response valid.
    pub bvalid: Signal<In, Bit>,
    /// Write response ready.
    pub bready: Signal<Out, Bit>,
    /// Read address.
    pub araddr: Signal<Out, Bits<32>>,
    /// Read address valid.
    pub arvalid: Signal<Out, Bit>,
    /// Read address ready.
    pub arready: Signal<In, Bit>,
    /// Read data.
    pub rdata: Signal<In, Bits<32>>,
    /// Read response.
    pub rresp: Signal<In, Bits<2>>,
    /// Read valid.
    pub rvalid: Signal<In, Bit>,
    /// Read ready.
    pub rready: Signal<Out, Bit>,
    // Internal state
    state: DFF<Bits<4>>,
    addr_reg: DFF<Bits<32>>,
    data_reg: DFF<Bits<32>>,
}

const ST_IDLE: u64 = 0;
const ST_WR_ADDR: u64 = 1;
const ST_WR_DATA: u64 = 2;
const ST_WR_RESP: u64 = 3;
const ST_RD_ADDR: u64 = 4;
const ST_RD_DATA: u64 = 5;
const ST_DONE: u64 = 6;

impl Logic for AxiLiteMaster {
    fn update(&mut self) {
        self.state.clock.next = self.clock.val();
        self.addr_reg.clock.next = self.clock.val();
        self.data_reg.clock.next = self.clock.val();

        // Defaults: hold state, deassert everything
        self.state.d.next = self.state.q.val();
        self.addr_reg.d.next = self.addr_reg.q.val();
        self.data_reg.d.next = self.data_reg.q.val();
        self.done.next = false;
        self.awvalid.next = false;
        self.awaddr.next = bits(0_u64);
        self.wvalid.next = false;
        self.wdata.next = bits(0_u64);
        self.wstrb.next = bits(0_u64);
        self.bready.next = false;
        self.arvalid.next = false;
        self.araddr.next = bits(0_u64);
        self.rready.next = false;
        self.rd_data.next = self.data_reg.q.val();

        let state_val = bits4_to_u64(self.state.q.val());
        self.busy.next = state_val != ST_IDLE;

        match state_val {
            ST_IDLE => {
                if self.wr_start.val() {
                    self.addr_reg.d.next = self.wr_addr.val();
                    self.data_reg.d.next = self.wr_data.val();
                    self.state.d.next = bits(ST_WR_ADDR);
                } else if self.rd_start.val() {
                    self.addr_reg.d.next = self.rd_addr.val();
                    self.state.d.next = bits(ST_RD_ADDR);
                }
            }
            ST_WR_ADDR => {
                self.awvalid.next = true;
                self.awaddr.next = self.addr_reg.q.val();
                if self.awready.val() {
                    self.state.d.next = bits(ST_WR_DATA);
                }
            }
            ST_WR_DATA => {
                self.wvalid.next = true;
                self.wdata.next = self.data_reg.q.val();
                self.wstrb.next = bits(0x0F_u64); // full 32-bit write
                if self.wready.val() {
                    self.state.d.next = bits(ST_WR_RESP);
                }
            }
            ST_WR_RESP => {
                self.bready.next = true;
                if self.bvalid.val() {
                    self.state.d.next = bits(ST_DONE);
                }
            }
            ST_RD_ADDR => {
                self.arvalid.next = true;
                self.araddr.next = self.addr_reg.q.val();
                if self.arready.val() {
                    self.state.d.next = bits(ST_RD_DATA);
                }
            }
            ST_RD_DATA => {
                self.rready.next = true;
                if self.rvalid.val() {
                    self.data_reg.d.next = self.rdata.val();
                    self.state.d.next = bits(ST_DONE);
                }
            }
            ST_DONE => {
                self.done.next = true;
                self.state.d.next = bits(ST_IDLE);
            }
            _ => {
                self.state.d.next = bits(ST_IDLE);
            }
        }
    }
}
