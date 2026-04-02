//! AXI protection type newtype.
//!
//! The 3-bit AWPROT/ARPROT field encodes access permissions:
//! bit 0 = privileged, bit 1 = non-secure, bit 2 = instruction.

/// AXI protection type (3-bit field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct AxiProt(u8);

impl AxiProt {
    /// Default: unprivileged, secure, data access.
    pub const DEFAULT: Self = Self(0b000);

    /// Create from a raw 3-bit value.
    pub fn new(value: u8) -> Self {
        Self(value & 0b111)
    }

    /// The raw 3-bit value.
    #[must_use]
    pub fn value(self) -> u8 {
        self.0
    }

    /// Whether this is a privileged access.
    #[must_use]
    pub fn privileged(self) -> bool {
        self.0 & 0b001 != 0
    }

    /// Whether this is a non-secure access.
    #[must_use]
    pub fn non_secure(self) -> bool {
        self.0 & 0b010 != 0
    }

    /// Whether this is an instruction access.
    #[must_use]
    pub fn instruction(self) -> bool {
        self.0 & 0b100 != 0
    }
}

impl std::fmt::Display for AxiProt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PROT({:#05b})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_unprivileged_secure_data() {
        let p = AxiProt::DEFAULT;
        assert!(!p.privileged());
        assert!(!p.non_secure());
        assert!(!p.instruction());
    }

    #[test]
    fn privileged_bit() {
        assert!(AxiProt::new(0b001).privileged());
    }

    #[test]
    fn masks_to_3_bits() {
        assert_eq!(AxiProt::new(0xFF).value(), 0b111);
    }
}
