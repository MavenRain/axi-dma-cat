//! AXI-Lite action descriptors: morphism target type for the free category.

/// An action performed during an AXI-Lite handshaking transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AxiLiteAction {
    /// No-op (identity morphism).
    Identity,
    /// Assert address valid (AWVALID or ARVALID) with address.
    AssertAddrValid,
    /// Address handshake complete (READY received).
    AddrHandshake,
    /// Assert write data valid (WVALID) with data and strobe.
    AssertDataValid,
    /// Write data handshake complete (WREADY received).
    DataHandshake,
    /// Assert response ready (BREADY or RREADY).
    AssertRespReady,
    /// Response handshake complete (BVALID or RVALID received).
    RespHandshake,
    /// Transfer complete, return to idle.
    Complete,
    /// A composed sequence of actions.
    Sequence(Vec<AxiLiteAction>),
}

impl AxiLiteAction {
    /// The identity action.
    #[must_use]
    pub fn identity() -> Self {
        Self::Identity
    }

    /// Compose two actions sequentially.
    #[must_use]
    pub fn compose(self, other: Self) -> Self {
        match (self, other) {
            (Self::Identity, b) => b,
            (a, Self::Identity) => a,
            (Self::Sequence(a), Self::Sequence(b)) => {
                Self::Sequence(a.into_iter().chain(b).collect())
            }
            (Self::Sequence(a), b) => {
                Self::Sequence(a.into_iter().chain(std::iter::once(b)).collect())
            }
            (a, Self::Sequence(b)) => {
                Self::Sequence(std::iter::once(a).chain(b).collect())
            }
            (a, b) => Self::Sequence(vec![a, b]),
        }
    }

    /// Number of atomic actions.
    #[must_use]
    pub fn action_count(&self) -> usize {
        match self {
            Self::Identity => 0,
            Self::Sequence(actions) => actions.iter().map(Self::action_count).sum(),
            _ => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identity_is_neutral() {
        let a = AxiLiteAction::AssertAddrValid;
        assert_eq!(AxiLiteAction::identity().compose(a.clone()), a);
        assert_eq!(a.clone().compose(AxiLiteAction::identity()), a);
    }

    #[test]
    fn compose_counts() {
        let c = AxiLiteAction::AssertAddrValid
            .compose(AxiLiteAction::AddrHandshake)
            .compose(AxiLiteAction::Complete);
        assert_eq!(c.action_count(), 3);
    }
}
