//! AXI response codes per AMBA spec.

use crate::error::Error;

/// AXI response code (2-bit field).
///
/// # Examples
///
/// ```
/// use axi_dma_cat::primitives::response::AxiResponse;
///
/// let resp = AxiResponse::Okay;
/// assert_eq!(resp.to_bits(), 0b00);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxiResponse {
    /// Normal access success.
    Okay,
    /// Slave error.
    SlvErr,
    /// Decode error (no slave at address).
    DecErr,
}

impl AxiResponse {
    /// Decode from the 2-bit BRESP/RRESP field.
    ///
    /// # Errors
    ///
    /// Returns an error for reserved code `0b01`.
    pub fn from_bits(bits: u8) -> Result<Self, Error> {
        match bits & 0b11 {
            0b00 => Ok(Self::Okay),
            0b10 => Ok(Self::SlvErr),
            0b11 => Ok(Self::DecErr),
            other => Err(Error::InvalidResponse { code: other }),
        }
    }

    /// Encode to the 2-bit field.
    #[must_use]
    pub fn to_bits(self) -> u8 {
        match self {
            Self::Okay => 0b00,
            Self::SlvErr => 0b10,
            Self::DecErr => 0b11,
        }
    }

    /// Whether this response indicates success.
    #[must_use]
    pub fn is_okay(self) -> bool {
        self == Self::Okay
    }
}

impl std::fmt::Display for AxiResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Okay => write!(f, "OKAY"),
            Self::SlvErr => write!(f, "SLVERR"),
            Self::DecErr => write!(f, "DECERR"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_all_valid_codes() -> Result<(), Error> {
        [AxiResponse::Okay, AxiResponse::SlvErr, AxiResponse::DecErr]
            .iter()
            .try_for_each(|resp| {
                let decoded = AxiResponse::from_bits(resp.to_bits())?;
                if decoded == *resp {
                    Ok(())
                } else {
                    Err(Error::InvalidResponse {
                        code: resp.to_bits(),
                    })
                }
            })
    }

    #[test]
    fn reserved_code_is_error() {
        assert!(AxiResponse::from_bits(0b01).is_err());
    }

    #[test]
    fn okay_is_okay() {
        assert!(AxiResponse::Okay.is_okay());
        assert!(!AxiResponse::SlvErr.is_okay());
    }
}
