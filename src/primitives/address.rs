//! AXI address newtype.

use crate::error::Error;

/// A 32-bit AXI address.
///
/// # Examples
///
/// ```
/// use axi_dma_cat::primitives::address::AxiAddress;
///
/// let addr = AxiAddress::new(0x4000_0000);
/// assert_eq!(addr.value(), 0x4000_0000);
/// assert!(addr.is_aligned(4));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[must_use]
pub struct AxiAddress(u32);

impl AxiAddress {
    /// The zero address.
    pub const ZERO: Self = Self(0);

    /// Create a new AXI address.
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    /// The raw 32-bit address value.
    #[must_use]
    pub fn value(self) -> u32 {
        self.0
    }

    /// Whether this address is aligned to the given power-of-two boundary.
    #[must_use]
    pub fn is_aligned(self, alignment: u32) -> bool {
        if alignment == 0 { true } else { self.0.is_multiple_of(alignment) }
    }

    /// Offset this address by `delta` bytes.
    ///
    /// # Errors
    ///
    /// Returns an error if the result would overflow 32 bits.
    pub fn offset(self, delta: u32) -> Result<Self, Error> {
        self.0.checked_add(delta).map(Self).ok_or(Error::InvalidAddress {
            addr: self.0,
        })
    }
}

impl std::fmt::Display for AxiAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#010x}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alignment_check() {
        let addr = AxiAddress::new(0x100);
        assert!(addr.is_aligned(4));
        assert!(addr.is_aligned(256));
        assert!(!addr.is_aligned(512));
    }

    #[test]
    fn offset_wraps_error() {
        let addr = AxiAddress::new(u32::MAX);
        assert!(addr.offset(1).is_err());
    }

    #[test]
    fn offset_success() -> Result<(), Error> {
        let addr = AxiAddress::new(0x1000);
        let next = addr.offset(4)?;
        assert_eq!(next.value(), 0x1004);
        Ok(())
    }
}
