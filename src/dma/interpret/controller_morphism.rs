//! Graph morphism mapping the DMA controller FSM into action descriptors.

use comp_cat_rs::collapse::free_category::{Edge, GraphMorphism, Vertex};

use crate::dma::graph::controller_graph::DmaControllerGraph;
use crate::dma::interpret::descriptor::DmaAction;

/// DMA controller state at a given FSM vertex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DmaControllerState {
    vertex: usize,
    transferring: bool,
    error: bool,
}

impl DmaControllerState {
    /// The vertex index.
    #[must_use]
    pub fn vertex(&self) -> usize {
        self.vertex
    }

    /// Whether a transfer is in progress.
    #[must_use]
    pub fn transferring(&self) -> bool {
        self.transferring
    }

    /// Whether an error has occurred.
    #[must_use]
    pub fn error(&self) -> bool {
        self.error
    }
}

/// Maps `DmaControllerGraph` into action descriptors.
pub struct DmaControllerMorphism;

impl GraphMorphism<DmaControllerGraph> for DmaControllerMorphism {
    type Object = DmaControllerState;
    type Morphism = DmaAction;

    fn map_vertex(&self, v: Vertex) -> DmaControllerState {
        match v.index() {
            // Idle(0), Done(5): not transferring
            0 | 5 => DmaControllerState {
                vertex: v.index(),
                transferring: false,
                error: false,
            },
            // Error(6): error state
            6 => DmaControllerState {
                vertex: v.index(),
                transferring: false,
                error: true,
            },
            // Configured(1), ReadPhase(2), WritePhase(3), CheckDone(4)
            _ => DmaControllerState {
                vertex: v.index(),
                transferring: true,
                error: false,
            },
        }
    }

    fn map_edge(&self, e: Edge) -> DmaAction {
        match e.index() {
            0 => DmaAction::Configure,      //  0: Idle -> Configured
            1 | 4 => DmaAction::IssueRead,  //  1: Configured -> ReadPhase, 4: loop
            2 => DmaAction::IssueWrite,     //  2: ReadPhase -> WritePhase
            3 => DmaAction::CheckRemaining, //  3: WritePhase -> CheckDone
            5 => DmaAction::Complete,       //  5: CheckDone -> Done
            6 | 7 => DmaAction::Error,      //  6/7: read/write error
            8 | 9 => DmaAction::ReturnIdle, //  8/9: done/error -> idle
            _ => DmaAction::Identity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comp_cat_rs::collapse::free_category::interpret;
    use crate::dma::graph::controller_graph::{
        DmaControllerGraph, single_transfer_path, read_error_path,
    };

    #[test]
    fn single_transfer_has_6_actions() -> Result<(), crate::error::Error> {
        let morphism = DmaControllerMorphism;
        let path = single_transfer_path()?;
        let composed = interpret::<DmaControllerGraph, _>(
            &morphism,
            &path,
            |_| DmaAction::identity(),
            DmaAction::compose,
        );
        assert_eq!(composed.action_count(), 6);
        Ok(())
    }

    #[test]
    fn error_path_has_4_actions() -> Result<(), crate::error::Error> {
        let morphism = DmaControllerMorphism;
        let path = read_error_path()?;
        let composed = interpret::<DmaControllerGraph, _>(
            &morphism,
            &path,
            |_| DmaAction::identity(),
            DmaAction::compose,
        );
        assert_eq!(composed.action_count(), 4);
        Ok(())
    }

    #[test]
    fn idle_not_transferring() {
        let morphism = DmaControllerMorphism;
        let state = morphism.map_vertex(Vertex::new(0));
        assert!(!state.transferring());
        assert!(!state.error());
    }

    #[test]
    fn read_phase_is_transferring() {
        let morphism = DmaControllerMorphism;
        let state = morphism.map_vertex(Vertex::new(2));
        assert!(state.transferring());
    }

    #[test]
    fn error_state_has_error() {
        let morphism = DmaControllerMorphism;
        let state = morphism.map_vertex(Vertex::new(6));
        assert!(state.error());
        assert!(!state.transferring());
    }
}
