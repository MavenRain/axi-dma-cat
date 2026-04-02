//! AXI-Lite write channel handshaking FSM as a free category graph.
//!
//! ```text
//! Idle(0) -> SendAddr(1) -> SendData(2) -> WaitResp(3) -> GotResp(4) -> Idle(0)
//! ```
//!
//! Simplified model: address phase completes before data phase.
//! The full AXI spec allows simultaneous AW and W, modeled
//! categorically via tensor product in the composition layer.

use comp_cat_rs::collapse::free_category::{Edge, FreeCategoryError, Graph, Path, Vertex};

/// Number of states in the write handshaking FSM.
pub const WRITE_VERTICES: usize = 5;

/// Number of transitions in the write handshaking FSM.
pub const WRITE_EDGES: usize = 5;

/// AXI-Lite write channel handshaking graph.
pub struct AxiLiteWriteGraph;

/// Edge definitions:
/// 0: Idle(0) -> SendAddr(1)      [assert AWVALID + AWADDR]
/// 1: SendAddr(1) -> SendData(2)  [AWREADY handshake, assert WVALID + WDATA]
/// 2: SendData(2) -> WaitResp(3)  [WREADY handshake, assert BREADY]
/// 3: WaitResp(3) -> GotResp(4)   [BVALID handshake, capture BRESP]
/// 4: GotResp(4) -> Idle(0)       [complete, return]
const EDGE_TABLE: [(usize, usize); WRITE_EDGES] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 0),
];

impl Graph for AxiLiteWriteGraph {
    fn vertex_count(&self) -> usize {
        WRITE_VERTICES
    }

    fn edge_count(&self) -> usize {
        WRITE_EDGES
    }

    fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(src, _)| Vertex::new(*src))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: WRITE_EDGES,
            })
    }

    fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(_, tgt)| Vertex::new(*tgt))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: WRITE_EDGES,
            })
    }
}

/// Build the full write transfer path: `Idle -> ... -> Idle`.
///
/// # Errors
///
/// Returns an error if path construction fails.
pub fn write_transfer_path() -> Result<Path, FreeCategoryError> {
    let graph = AxiLiteWriteGraph;
    (0..WRITE_EDGES)
        .map(|k| Path::singleton(&graph, Edge::new(k)))
        .try_fold(
            Path::identity(Vertex::new(0)),
            |acc, edge_path| acc.compose(edge_path?),
        )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_dimensions() {
        let g = AxiLiteWriteGraph;
        assert_eq!(g.vertex_count(), 5);
        assert_eq!(g.edge_count(), 5);
    }

    #[test]
    fn all_edges_valid() -> Result<(), FreeCategoryError> {
        let g = AxiLiteWriteGraph;
        (0..WRITE_EDGES).try_for_each(|k| {
            let s = g.source(Edge::new(k))?;
            let t = g.target(Edge::new(k))?;
            assert!(s.index() < WRITE_VERTICES);
            assert!(t.index() < WRITE_VERTICES);
            Ok(())
        })
    }

    #[test]
    fn write_path_is_round_trip() -> Result<(), FreeCategoryError> {
        let path = write_transfer_path()?;
        assert_eq!(path.source().index(), 0);
        assert_eq!(path.target().index(), 0);
        assert_eq!(path.len(), 5);
        Ok(())
    }
}
