//! DMA action descriptors: morphism target type for the free category.

/// An action performed during a DMA controller FSM transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DmaAction {
    /// No-op (identity morphism).
    Identity,
    /// Configure: load registers, prepare for transfer.
    Configure,
    /// Issue AXI read from source address.
    IssueRead,
    /// Issue AXI write to destination address.
    IssueWrite,
    /// Check whether more data remains to transfer.
    CheckRemaining,
    /// Transfer complete, set done status.
    Complete,
    /// Error occurred, set error status.
    Error,
    /// Return to idle.
    ReturnIdle,
    /// A composed sequence of actions.
    Sequence(Vec<DmaAction>),
}

impl DmaAction {
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
        let a = DmaAction::Configure;
        assert_eq!(DmaAction::identity().compose(a.clone()), a);
        assert_eq!(a.clone().compose(DmaAction::identity()), a);
    }

    #[test]
    fn compose_counts() {
        let c = DmaAction::Configure
            .compose(DmaAction::IssueRead)
            .compose(DmaAction::IssueWrite);
        assert_eq!(c.action_count(), 3);
    }
}
