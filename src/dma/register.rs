//! DMA register map newtypes with bit-field accessors.
//!
//! | Offset | Register | Description |
//! |--------|----------|-------------|
//! | 0x00 | Control | start, stop, interrupt enable |
//! | 0x04 | Status | busy, done, error, interrupt pending |
//! | 0x08 | Source Address | 32-bit source |
//! | 0x0C | Dest Address | 32-bit destination |
//! | 0x10 | Transfer Length | bytes to transfer |

use crate::primitives::address::AxiAddress;

/// Register offset constants.
pub const REG_CONTROL: u32 = 0x00;
/// Status register offset.
pub const REG_STATUS: u32 = 0x04;
/// Source address register offset.
pub const REG_SOURCE: u32 = 0x08;
/// Destination address register offset.
pub const REG_DEST: u32 = 0x0C;
/// Transfer length register offset.
pub const REG_LENGTH: u32 = 0x10;

/// DMA control register (offset 0x00).
///
/// Bit 0: start.  Bit 1: stop.  Bit 2: interrupt enable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct DmaControl(u32);

impl DmaControl {
    /// Create from a raw 32-bit value.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// The raw value.
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }

    /// Whether the start bit is set.
    #[must_use]
    pub fn start(self) -> bool {
        self.0 & 0x01 != 0
    }

    /// Whether the stop bit is set.
    #[must_use]
    pub fn stop(self) -> bool {
        self.0 & 0x02 != 0
    }

    /// Whether interrupts are enabled.
    #[must_use]
    pub fn interrupt_enable(self) -> bool {
        self.0 & 0x04 != 0
    }

    /// Build a control word with specific flags.
    pub fn with_start() -> Self {
        Self(0x01)
    }

    /// Build a control word with stop flag.
    pub fn with_stop() -> Self {
        Self(0x02)
    }
}

impl std::fmt::Display for DmaControl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CTRL(start={}, stop={}, irq={})",
            self.start(),
            self.stop(),
            self.interrupt_enable()
        )
    }
}

/// DMA status register (offset 0x04).
///
/// Bit 0: busy.  Bit 1: done.  Bit 2: error.  Bit 3: interrupt pending.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct DmaStatus(u32);

impl DmaStatus {
    /// Create from a raw 32-bit value.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// The raw value.
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }

    /// Whether the DMA is currently transferring.
    #[must_use]
    pub fn busy(self) -> bool {
        self.0 & 0x01 != 0
    }

    /// Whether the last transfer completed successfully.
    #[must_use]
    pub fn done(self) -> bool {
        self.0 & 0x02 != 0
    }

    /// Whether an error occurred.
    #[must_use]
    pub fn error(self) -> bool {
        self.0 & 0x04 != 0
    }

    /// Whether an interrupt is pending.
    #[must_use]
    pub fn interrupt_pending(self) -> bool {
        self.0 & 0x08 != 0
    }

    /// Status for idle state.
    pub fn idle() -> Self {
        Self(0)
    }

    /// Status for busy state.
    pub fn busy_status() -> Self {
        Self(0x01)
    }

    /// Status for done state.
    pub fn done_status() -> Self {
        Self(0x02)
    }

    /// Status for error state.
    pub fn error_status() -> Self {
        Self(0x04)
    }
}

impl std::fmt::Display for DmaStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "STAT(busy={}, done={}, err={}, irq={})",
            self.busy(),
            self.done(),
            self.error(),
            self.interrupt_pending()
        )
    }
}

/// Transfer length in bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct TransferLength(u32);

impl TransferLength {
    /// Create a new transfer length.
    pub fn new(bytes: u32) -> Self {
        Self(bytes)
    }

    /// The length in bytes.
    #[must_use]
    pub fn bytes(self) -> u32 {
        self.0
    }

    /// Whether this is a zero-length transfer.
    #[must_use]
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }
}

impl std::fmt::Display for TransferLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} bytes", self.0)
    }
}

/// A complete DMA register set (snapshot).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct DmaRegisterSet {
    control: DmaControl,
    status: DmaStatus,
    source: AxiAddress,
    dest: AxiAddress,
    length: TransferLength,
}

impl DmaRegisterSet {
    /// Create a new register set (all zeros).
    pub fn new() -> Self {
        Self {
            control: DmaControl::new(0),
            status: DmaStatus::idle(),
            source: AxiAddress::ZERO,
            dest: AxiAddress::ZERO,
            length: TransferLength::new(0),
        }
    }

    /// The control register.
    pub fn control(&self) -> DmaControl {
        self.control
    }

    /// The status register.
    pub fn status(&self) -> DmaStatus {
        self.status
    }

    /// The source address.
    pub fn source(&self) -> AxiAddress {
        self.source
    }

    /// The destination address.
    pub fn dest(&self) -> AxiAddress {
        self.dest
    }

    /// The transfer length.
    pub fn length(&self) -> TransferLength {
        self.length
    }

    /// Return a new register set with the control register updated.
    pub fn with_control(self, control: DmaControl) -> Self {
        Self { control, ..self }
    }

    /// Return a new register set with the status register updated.
    pub fn with_status(self, status: DmaStatus) -> Self {
        Self { status, ..self }
    }

    /// Return a new register set with the source address updated.
    pub fn with_source(self, source: AxiAddress) -> Self {
        Self { source, ..self }
    }

    /// Return a new register set with the destination address updated.
    pub fn with_dest(self, dest: AxiAddress) -> Self {
        Self { dest, ..self }
    }

    /// Return a new register set with the transfer length updated.
    pub fn with_length(self, length: TransferLength) -> Self {
        Self { length, ..self }
    }
}

impl Default for DmaRegisterSet {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn control_bit_fields() {
        let ctrl = DmaControl::new(0x07);
        assert!(ctrl.start());
        assert!(ctrl.stop());
        assert!(ctrl.interrupt_enable());
    }

    #[test]
    fn control_start_only() {
        let ctrl = DmaControl::with_start();
        assert!(ctrl.start());
        assert!(!ctrl.stop());
    }

    #[test]
    fn status_bit_fields() {
        let stat = DmaStatus::new(0x0F);
        assert!(stat.busy());
        assert!(stat.done());
        assert!(stat.error());
        assert!(stat.interrupt_pending());
    }

    #[test]
    fn status_idle_is_all_clear() {
        let stat = DmaStatus::idle();
        assert!(!stat.busy());
        assert!(!stat.done());
        assert!(!stat.error());
    }

    #[test]
    fn register_set_with_updates() {
        let regs = DmaRegisterSet::new()
            .with_source(AxiAddress::new(0x1000))
            .with_dest(AxiAddress::new(0x2000))
            .with_length(TransferLength::new(256));
        assert_eq!(regs.source().value(), 0x1000);
        assert_eq!(regs.dest().value(), 0x2000);
        assert_eq!(regs.length().bytes(), 256);
    }

    #[test]
    fn transfer_length_zero() {
        assert!(TransferLength::new(0).is_zero());
        assert!(!TransferLength::new(1).is_zero());
    }
}
