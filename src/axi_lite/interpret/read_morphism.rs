//! Graph morphism mapping the AXI-Lite read FSM into action descriptors.

use comp_cat_rs::collapse::free_category::{Edge, GraphMorphism, Vertex};

use crate::axi_lite::graph::read_graph::AxiLiteReadGraph;
use crate::axi_lite::interpret::descriptor::AxiLiteAction;

/// Bus state at a read FSM vertex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReadChannelState {
    vertex: usize,
    ar_valid: bool,
    r_ready: bool,
}

impl ReadChannelState {
    /// The vertex index.
    #[must_use]
    pub fn vertex(&self) -> usize {
        self.vertex
    }

    /// Whether ARVALID is asserted.
    #[must_use]
    pub fn ar_valid(&self) -> bool {
        self.ar_valid
    }

    /// Whether RREADY is asserted.
    #[must_use]
    pub fn r_ready(&self) -> bool {
        self.r_ready
    }
}

/// Maps `AxiLiteReadGraph` into action descriptors.
pub struct AxiLiteReadMorphism;

impl GraphMorphism<AxiLiteReadGraph> for AxiLiteReadMorphism {
    type Object = ReadChannelState;
    type Morphism = AxiLiteAction;

    fn map_vertex(&self, v: Vertex) -> ReadChannelState {
        match v.index() {
            0 | 3 => ReadChannelState {
                vertex: v.index(),
                ar_valid: false,
                r_ready: false,
            },
            1 => ReadChannelState {
                vertex: v.index(),
                ar_valid: true,
                r_ready: false,
            },
            _ => ReadChannelState {
                vertex: v.index(),
                ar_valid: false,
                r_ready: true,
            },
        }
    }

    fn map_edge(&self, e: Edge) -> AxiLiteAction {
        match e.index() {
            0 => AxiLiteAction::AssertAddrValid, // Idle -> SendAddr
            1 => AxiLiteAction::AssertRespReady, // SendAddr -> WaitData
            2 => AxiLiteAction::RespHandshake,   // WaitData -> GotData
            3 => AxiLiteAction::Complete,        // GotData -> Idle
            _ => AxiLiteAction::Identity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comp_cat_rs::collapse::free_category::interpret;
    use crate::axi_lite::graph::read_graph::{AxiLiteReadGraph, read_transfer_path};

    #[test]
    fn read_transfer_has_4_actions() -> Result<(), crate::error::Error> {
        let morphism = AxiLiteReadMorphism;
        let path = read_transfer_path()?;
        let composed = interpret::<AxiLiteReadGraph, _>(
            &morphism,
            &path,
            |_| AxiLiteAction::identity(),
            AxiLiteAction::compose,
        );
        assert_eq!(composed.action_count(), 4);
        Ok(())
    }

    #[test]
    fn idle_nothing_asserted() {
        let morphism = AxiLiteReadMorphism;
        let state = morphism.map_vertex(Vertex::new(0));
        assert!(!state.ar_valid());
        assert!(!state.r_ready());
    }

    #[test]
    fn send_addr_ar_valid() {
        let morphism = AxiLiteReadMorphism;
        let state = morphism.map_vertex(Vertex::new(1));
        assert!(state.ar_valid());
    }
}
