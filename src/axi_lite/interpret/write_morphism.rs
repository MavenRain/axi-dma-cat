//! Graph morphism mapping the AXI-Lite write FSM into action descriptors.

use comp_cat_rs::collapse::free_category::{Edge, GraphMorphism, Vertex};

use crate::axi_lite::graph::write_graph::AxiLiteWriteGraph;
use crate::axi_lite::interpret::descriptor::AxiLiteAction;

/// Bus state at a write FSM vertex.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteChannelState {
    vertex: usize,
    aw_valid: bool,
    w_valid: bool,
    b_ready: bool,
}

impl WriteChannelState {
    /// The vertex index.
    #[must_use]
    pub fn vertex(&self) -> usize {
        self.vertex
    }

    /// Whether AWVALID is asserted.
    #[must_use]
    pub fn aw_valid(&self) -> bool {
        self.aw_valid
    }

    /// Whether WVALID is asserted.
    #[must_use]
    pub fn w_valid(&self) -> bool {
        self.w_valid
    }

    /// Whether BREADY is asserted.
    #[must_use]
    pub fn b_ready(&self) -> bool {
        self.b_ready
    }
}

/// Maps `AxiLiteWriteGraph` into action descriptors.
pub struct AxiLiteWriteMorphism;

impl GraphMorphism<AxiLiteWriteGraph> for AxiLiteWriteMorphism {
    type Object = WriteChannelState;
    type Morphism = AxiLiteAction;

    fn map_vertex(&self, v: Vertex) -> WriteChannelState {
        match v.index() {
            0 | 4 => WriteChannelState {
                vertex: v.index(),
                aw_valid: false,
                w_valid: false,
                b_ready: false,
            },
            1 => WriteChannelState {
                vertex: v.index(),
                aw_valid: true,
                w_valid: false,
                b_ready: false,
            },
            2 => WriteChannelState {
                vertex: v.index(),
                aw_valid: false,
                w_valid: true,
                b_ready: false,
            },
            _ => WriteChannelState {
                vertex: v.index(),
                aw_valid: false,
                w_valid: false,
                b_ready: true,
            },
        }
    }

    fn map_edge(&self, e: Edge) -> AxiLiteAction {
        match e.index() {
            0 => AxiLiteAction::AssertAddrValid,  // Idle -> SendAddr
            1 => AxiLiteAction::AssertDataValid,   // SendAddr -> SendData
            2 => AxiLiteAction::AssertRespReady,   // SendData -> WaitResp
            3 => AxiLiteAction::RespHandshake,     // WaitResp -> GotResp
            4 => AxiLiteAction::Complete,          // GotResp -> Idle
            _ => AxiLiteAction::Identity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use comp_cat_rs::collapse::free_category::interpret;
    use crate::axi_lite::graph::write_graph::{AxiLiteWriteGraph, write_transfer_path};

    #[test]
    fn write_transfer_has_5_actions() -> Result<(), crate::error::Error> {
        let morphism = AxiLiteWriteMorphism;
        let path = write_transfer_path()?;
        let composed = interpret::<AxiLiteWriteGraph, _>(
            &morphism,
            &path,
            |_| AxiLiteAction::identity(),
            AxiLiteAction::compose,
        );
        assert_eq!(composed.action_count(), 5);
        Ok(())
    }

    #[test]
    fn idle_state_nothing_asserted() {
        let morphism = AxiLiteWriteMorphism;
        let state = morphism.map_vertex(Vertex::new(0));
        assert!(!state.aw_valid());
        assert!(!state.w_valid());
        assert!(!state.b_ready());
    }

    #[test]
    fn send_addr_state_aw_valid() {
        let morphism = AxiLiteWriteMorphism;
        let state = morphism.map_vertex(Vertex::new(1));
        assert!(state.aw_valid());
        assert!(!state.w_valid());
    }
}
