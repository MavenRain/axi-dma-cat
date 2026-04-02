//! AXI-Lite slave register file `RustHDL` module.
//!
//! A simple 16-register slave that responds to AXI-Lite read
//! and write transactions.  Registers are 32 bits wide at
//! 4-byte-aligned offsets 0x00 through 0x3C.

use rust_hdl::prelude::*;

fn bits4_to_u64(b: Bits<4>) -> u64 {
    (0..4_usize).fold(0_u64, |acc, i| acc | (u64::from(b.get_bit(i)) << i))
}

fn bits32_to_u64(b: Bits<32>) -> u64 {
    (0..32_usize).fold(0_u64, |acc, i| acc | (u64::from(b.get_bit(i)) << i))
}

/// AXI-Lite slave with a 16-entry register file.
///
/// Supports simultaneous write and read handling via independent
/// FSMs for the write (AW+W+B) and read (AR+R) channels.
#[derive(Clone, Debug, Default, LogicBlock)]
pub struct AxiLiteSlave {
    /// System clock.
    pub clock: Signal<In, Clock>,
    // Write address channel
    /// Write address.
    pub awaddr: Signal<In, Bits<32>>,
    /// Write address valid.
    pub awvalid: Signal<In, Bit>,
    /// Write address ready.
    pub awready: Signal<Out, Bit>,
    // Write data channel
    /// Write data.
    pub wdata: Signal<In, Bits<32>>,
    /// Write strobe.
    pub wstrb: Signal<In, Bits<4>>,
    /// Write data valid.
    pub wvalid: Signal<In, Bit>,
    /// Write data ready.
    pub wready: Signal<Out, Bit>,
    // Write response channel
    /// Write response.
    pub bresp: Signal<Out, Bits<2>>,
    /// Write response valid.
    pub bvalid: Signal<Out, Bit>,
    /// Write response ready.
    pub bready: Signal<In, Bit>,
    // Read address channel
    /// Read address.
    pub araddr: Signal<In, Bits<32>>,
    /// Read address valid.
    pub arvalid: Signal<In, Bit>,
    /// Read address ready.
    pub arready: Signal<Out, Bit>,
    // Read data channel
    /// Read data.
    pub rdata: Signal<Out, Bits<32>>,
    /// Read response.
    pub rresp: Signal<Out, Bits<2>>,
    /// Read data valid.
    pub rvalid: Signal<Out, Bit>,
    /// Read data ready.
    pub rready: Signal<In, Bit>,
    // Internal state
    wr_state: DFF<Bits<4>>,
    rd_state: DFF<Bits<4>>,
    wr_addr_reg: DFF<Bits<32>>,
    wr_data_reg: DFF<Bits<32>>,
    rd_addr_reg: DFF<Bits<32>>,
    // 16 registers
    reg0: DFF<Bits<32>>,
    reg1: DFF<Bits<32>>,
    reg2: DFF<Bits<32>>,
    reg3: DFF<Bits<32>>,
    reg4: DFF<Bits<32>>,
    reg5: DFF<Bits<32>>,
    reg6: DFF<Bits<32>>,
    reg7: DFF<Bits<32>>,
    reg8: DFF<Bits<32>>,
    reg9: DFF<Bits<32>>,
    reg10: DFF<Bits<32>>,
    reg11: DFF<Bits<32>>,
    reg12: DFF<Bits<32>>,
    reg13: DFF<Bits<32>>,
    reg14: DFF<Bits<32>>,
    reg15: DFF<Bits<32>>,
}

// Write FSM states
const WR_IDLE: u64 = 0;
const WR_GOT_ADDR: u64 = 1;
const WR_GOT_DATA: u64 = 2;
const WR_RESP: u64 = 3;

// Read FSM states
const RD_IDLE: u64 = 0;
const RD_GOT_ADDR: u64 = 1;
const RD_RESP: u64 = 2;

impl AxiLiteSlave {
    /// Read a register by index (0..15).
    fn read_reg(&self, idx: u64) -> Bits<32> {
        match idx {
            0 => self.reg0.q.val(),
            1 => self.reg1.q.val(),
            2 => self.reg2.q.val(),
            3 => self.reg3.q.val(),
            4 => self.reg4.q.val(),
            5 => self.reg5.q.val(),
            6 => self.reg6.q.val(),
            7 => self.reg7.q.val(),
            8 => self.reg8.q.val(),
            9 => self.reg9.q.val(),
            10 => self.reg10.q.val(),
            11 => self.reg11.q.val(),
            12 => self.reg12.q.val(),
            13 => self.reg13.q.val(),
            14 => self.reg14.q.val(),
            15 => self.reg15.q.val(),
            _ => bits(0_u64),
        }
    }

    /// Write a register by index.  Caller must also hold the d.next
    /// for all other registers (default hold is set in `update`).
    fn write_reg(&mut self, idx: u64, data: Bits<32>) {
        match idx {
            0 => self.reg0.d.next = data,
            1 => self.reg1.d.next = data,
            2 => self.reg2.d.next = data,
            3 => self.reg3.d.next = data,
            4 => self.reg4.d.next = data,
            5 => self.reg5.d.next = data,
            6 => self.reg6.d.next = data,
            7 => self.reg7.d.next = data,
            8 => self.reg8.d.next = data,
            9 => self.reg9.d.next = data,
            10 => self.reg10.d.next = data,
            11 => self.reg11.d.next = data,
            12 => self.reg12.d.next = data,
            13 => self.reg13.d.next = data,
            14 => self.reg14.d.next = data,
            15 => self.reg15.d.next = data,
            _ => {}
        }
    }
}

impl Logic for AxiLiteSlave {
    #[allow(clippy::too_many_lines)]
    fn update(&mut self) {
        // Clock all registers
        self.wr_state.clock.next = self.clock.val();
        self.rd_state.clock.next = self.clock.val();
        self.wr_addr_reg.clock.next = self.clock.val();
        self.wr_data_reg.clock.next = self.clock.val();
        self.rd_addr_reg.clock.next = self.clock.val();
        self.reg0.clock.next = self.clock.val();
        self.reg1.clock.next = self.clock.val();
        self.reg2.clock.next = self.clock.val();
        self.reg3.clock.next = self.clock.val();
        self.reg4.clock.next = self.clock.val();
        self.reg5.clock.next = self.clock.val();
        self.reg6.clock.next = self.clock.val();
        self.reg7.clock.next = self.clock.val();
        self.reg8.clock.next = self.clock.val();
        self.reg9.clock.next = self.clock.val();
        self.reg10.clock.next = self.clock.val();
        self.reg11.clock.next = self.clock.val();
        self.reg12.clock.next = self.clock.val();
        self.reg13.clock.next = self.clock.val();
        self.reg14.clock.next = self.clock.val();
        self.reg15.clock.next = self.clock.val();

        // Default: hold all registers
        self.wr_state.d.next = self.wr_state.q.val();
        self.rd_state.d.next = self.rd_state.q.val();
        self.wr_addr_reg.d.next = self.wr_addr_reg.q.val();
        self.wr_data_reg.d.next = self.wr_data_reg.q.val();
        self.rd_addr_reg.d.next = self.rd_addr_reg.q.val();
        self.reg0.d.next = self.reg0.q.val();
        self.reg1.d.next = self.reg1.q.val();
        self.reg2.d.next = self.reg2.q.val();
        self.reg3.d.next = self.reg3.q.val();
        self.reg4.d.next = self.reg4.q.val();
        self.reg5.d.next = self.reg5.q.val();
        self.reg6.d.next = self.reg6.q.val();
        self.reg7.d.next = self.reg7.q.val();
        self.reg8.d.next = self.reg8.q.val();
        self.reg9.d.next = self.reg9.q.val();
        self.reg10.d.next = self.reg10.q.val();
        self.reg11.d.next = self.reg11.q.val();
        self.reg12.d.next = self.reg12.q.val();
        self.reg13.d.next = self.reg13.q.val();
        self.reg14.d.next = self.reg14.q.val();
        self.reg15.d.next = self.reg15.q.val();

        // Default outputs
        self.awready.next = false;
        self.wready.next = false;
        self.bvalid.next = false;
        self.bresp.next = bits(0_u64); // OKAY
        self.arready.next = false;
        self.rvalid.next = false;
        self.rdata.next = bits(0_u64);
        self.rresp.next = bits(0_u64); // OKAY

        // --- Write FSM ---
        let wr_state_val = bits4_to_u64(self.wr_state.q.val());

        match wr_state_val {
            WR_IDLE => {
                self.awready.next = true;
                self.wready.next = true;
                if self.awvalid.val() {
                    self.wr_addr_reg.d.next = self.awaddr.val();
                    self.wr_state.d.next = bits(WR_GOT_ADDR);
                }
                if self.wvalid.val() {
                    self.wr_data_reg.d.next = self.wdata.val();
                    if self.awvalid.val() {
                        // Both arrived simultaneously
                        self.wr_state.d.next = bits(WR_RESP);
                    } else {
                        self.wr_state.d.next = bits(WR_GOT_DATA);
                    }
                }
            }
            WR_GOT_ADDR => {
                self.wready.next = true;
                if self.wvalid.val() {
                    self.wr_data_reg.d.next = self.wdata.val();
                    self.wr_state.d.next = bits(WR_RESP);
                }
            }
            WR_GOT_DATA => {
                self.awready.next = true;
                if self.awvalid.val() {
                    self.wr_addr_reg.d.next = self.awaddr.val();
                    self.wr_state.d.next = bits(WR_RESP);
                }
            }
            WR_RESP => {
                // Perform the write: address bits [5:2] select register
                let addr = bits32_to_u64(self.wr_addr_reg.q.val());
                let reg_idx = (addr >> 2) & 0x0F;
                self.write_reg(reg_idx, self.wr_data_reg.q.val());

                self.bvalid.next = true;
                if self.bready.val() {
                    self.wr_state.d.next = bits(WR_IDLE);
                }
            }
            _ => {
                self.wr_state.d.next = bits(WR_IDLE);
            }
        }

        // --- Read FSM ---
        let rd_state_val = bits4_to_u64(self.rd_state.q.val());

        match rd_state_val {
            RD_IDLE => {
                self.arready.next = true;
                if self.arvalid.val() {
                    self.rd_addr_reg.d.next = self.araddr.val();
                    self.rd_state.d.next = bits(RD_GOT_ADDR);
                }
            }
            RD_GOT_ADDR => {
                let addr = bits32_to_u64(self.rd_addr_reg.q.val());
                let reg_idx = (addr >> 2) & 0x0F;
                self.rdata.next = self.read_reg(reg_idx);
                self.rvalid.next = true;
                self.rd_state.d.next = bits(RD_RESP);
            }
            RD_RESP => {
                let addr = bits32_to_u64(self.rd_addr_reg.q.val());
                let reg_idx = (addr >> 2) & 0x0F;
                self.rdata.next = self.read_reg(reg_idx);
                self.rvalid.next = true;
                if self.rready.val() {
                    self.rd_state.d.next = bits(RD_IDLE);
                }
            }
            _ => {
                self.rd_state.d.next = bits(RD_IDLE);
            }
        }
    }
}
