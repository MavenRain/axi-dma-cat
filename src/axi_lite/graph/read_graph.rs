//! AXI-Lite read channel handshaking FSM as a free category graph.
//!
//! ```text
//! Idle(0) -> SendAddr(1) -> WaitData(2) -> GotData(3) -> Idle(0)
//! ```

use comp_cat_rs::collapse::free_category::{Edge, FreeCategoryError, Graph, Path, Vertex};

/// Number of states in the read handshaking FSM.
pub const READ_VERTICES: usize = 4;

/// Number of transitions in the read handshaking FSM.
pub const READ_EDGES: usize = 4;

/// AXI-Lite read channel handshaking graph.
pub struct AxiLiteReadGraph;

/// Edge definitions:
/// 0: Idle(0) -> SendAddr(1)      [assert ARVALID + ARADDR]
/// 1: SendAddr(1) -> WaitData(2)  [ARREADY handshake, assert RREADY]
/// 2: WaitData(2) -> GotData(3)   [RVALID handshake, capture RDATA + RRESP]
/// 3: GotData(3) -> Idle(0)       [complete, return]
const EDGE_TABLE: [(usize, usize); READ_EDGES] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
];

impl Graph for AxiLiteReadGraph {
    fn vertex_count(&self) -> usize {
        READ_VERTICES
    }

    fn edge_count(&self) -> usize {
        READ_EDGES
    }

    fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(src, _)| Vertex::new(*src))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: READ_EDGES,
            })
    }

    fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(_, tgt)| Vertex::new(*tgt))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: READ_EDGES,
            })
    }
}

/// Build the full read transfer path: `Idle -> ... -> Idle`.
///
/// # Errors
///
/// Returns an error if path construction fails.
pub fn read_transfer_path() -> Result<Path, FreeCategoryError> {
    let graph = AxiLiteReadGraph;
    (0..READ_EDGES)
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
        let g = AxiLiteReadGraph;
        assert_eq!(g.vertex_count(), 4);
        assert_eq!(g.edge_count(), 4);
    }

    #[test]
    fn all_edges_valid() -> Result<(), FreeCategoryError> {
        let g = AxiLiteReadGraph;
        (0..READ_EDGES).try_for_each(|k| {
            let s = g.source(Edge::new(k))?;
            let t = g.target(Edge::new(k))?;
            assert!(s.index() < READ_VERTICES);
            assert!(t.index() < READ_VERTICES);
            Ok(())
        })
    }

    #[test]
    fn read_path_is_round_trip() -> Result<(), FreeCategoryError> {
        let path = read_transfer_path()?;
        assert_eq!(path.source().index(), 0);
        assert_eq!(path.target().index(), 0);
        assert_eq!(path.len(), 4);
        Ok(())
    }
}
