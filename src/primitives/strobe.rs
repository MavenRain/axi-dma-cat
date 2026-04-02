//! Write strobe newtype.
//!
//! The write strobe indicates which byte lanes of the data bus
//! carry valid data.  For a 32-bit bus, the strobe is 4 bits;
//! for a 64-bit bus, 8 bits.

/// Write strobe (byte-enable mask).
///
/// Each bit corresponds to one byte lane of the data bus.
/// Bit 0 = bytes 0, bit 1 = byte 1, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[must_use]
pub struct WriteStrobe(u8);

impl WriteStrobe {
    /// All bytes valid (full-width write).
    pub const ALL_32: Self = Self(0x0F);

    /// All bytes valid for 64-bit bus.
    pub const ALL_64: Self = Self(0xFF);

    /// Create a write strobe from a raw byte mask.
    pub fn new(mask: u8) -> Self {
        Self(mask)
    }

    /// The raw mask value.
    #[must_use]
    pub fn mask(self) -> u8 {
        self.0
    }

    /// Whether all byte lanes are active for a given data width (in bytes).
    #[must_use]
    pub fn is_full(self, width_bytes: u8) -> bool {
        let full_mask = (1_u16 << width_bytes) - 1;
        u16::from(self.0) == full_mask
    }
}

impl std::fmt::Display for WriteStrobe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WSTRB({:#04x})", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_32_is_full_for_4_bytes() {
        assert!(WriteStrobe::ALL_32.is_full(4));
        assert!(!WriteStrobe::ALL_32.is_full(8));
    }

    #[test]
    fn all_64_is_full_for_8_bytes() {
        assert!(WriteStrobe::ALL_64.is_full(8));
    }

    #[test]
    fn partial_strobe_not_full() {
        assert!(!WriteStrobe::new(0x03).is_full(4));
    }
}
