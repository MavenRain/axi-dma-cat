//! DMA controller FSM as a free category graph.
//!
//! ```text
//! Idle(0) -> Configured(1) -> ReadPhase(2) -> WritePhase(3) -> CheckDone(4)
//!   CheckDone -> ReadPhase (loop: more data)
//!   CheckDone -> Done(5) -> Idle
//!   ReadPhase -> Error(6) -> Idle
//!   WritePhase -> Error(6) -> Idle
//! ```

use comp_cat_rs::collapse::free_category::{Edge, FreeCategoryError, Graph, Path, Vertex};

/// Number of states in the DMA controller FSM.
pub const DMA_VERTICES: usize = 7;

/// Number of transitions in the DMA controller FSM.
pub const DMA_EDGES: usize = 10;

/// The DMA controller FSM graph.
pub struct DmaControllerGraph;

/// Edge definitions:
///  0: Idle(0) -> Configured(1)       [registers written, start bit]
///  1: Configured(1) -> ReadPhase(2)  [issue AXI read from source]
///  2: ReadPhase(2) -> WritePhase(3)  [read complete, issue AXI write]
///  3: WritePhase(3) -> CheckDone(4)  [write complete, check remaining]
///  4: CheckDone(4) -> ReadPhase(2)   [more data, loop]
///  5: CheckDone(4) -> Done(5)        [transfer complete]
///  6: ReadPhase(2) -> Error(6)       [AXI read error]
///  7: WritePhase(3) -> Error(6)      [AXI write error]
///  8: Done(5) -> Idle(0)             [return to idle]
///  9: Error(6) -> Idle(0)            [error acknowledged]
const EDGE_TABLE: [(usize, usize); DMA_EDGES] = [
    (0, 1), //  0: start
    (1, 2), //  1: begin read
    (2, 3), //  2: read -> write
    (3, 4), //  3: write -> check
    (4, 2), //  4: loop
    (4, 5), //  5: done
    (2, 6), //  6: read error
    (3, 6), //  7: write error
    (5, 0), //  8: done -> idle
    (6, 0), //  9: error -> idle
];

impl Graph for DmaControllerGraph {
    fn vertex_count(&self) -> usize {
        DMA_VERTICES
    }

    fn edge_count(&self) -> usize {
        DMA_EDGES
    }

    fn source(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(src, _)| Vertex::new(*src))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: DMA_EDGES,
            })
    }

    fn target(&self, edge: Edge) -> Result<Vertex, FreeCategoryError> {
        EDGE_TABLE
            .get(edge.index())
            .map(|(_, tgt)| Vertex::new(*tgt))
            .ok_or(FreeCategoryError::EdgeOutOfBounds {
                edge,
                count: DMA_EDGES,
            })
    }
}

/// Build the single-transfer path (no looping, no error):
///
/// ```text
/// Idle -> Configured -> ReadPhase -> WritePhase -> CheckDone -> Done -> Idle
/// ```
///
/// # Errors
///
/// Returns an error if path construction fails.
pub fn single_transfer_path() -> Result<Path, FreeCategoryError> {
    let graph = DmaControllerGraph;
    // Edges: 0 (start), 1 (begin read), 2 (read->write),
    // 3 (write->check), 5 (check->done), 8 (done->idle)
    [0, 1, 2, 3, 5, 8]
        .iter()
        .map(|&e| Path::singleton(&graph, Edge::new(e)))
        .try_fold(
            Path::identity(Vertex::new(0)),
            |acc, edge_path| acc.compose(edge_path?),
        )
}

/// Build the error path from read phase:
///
/// ```text
/// Idle -> Configured -> ReadPhase -> Error -> Idle
/// ```
///
/// # Errors
///
/// Returns an error if path construction fails.
pub fn read_error_path() -> Result<Path, FreeCategoryError> {
    let graph = DmaControllerGraph;
    [0, 1, 6, 9]
        .iter()
        .map(|&e| Path::singleton(&graph, Edge::new(e)))
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
        let g = DmaControllerGraph;
        assert_eq!(g.vertex_count(), 7);
        assert_eq!(g.edge_count(), 10);
    }

    #[test]
    fn all_edges_valid() -> Result<(), FreeCategoryError> {
        let g = DmaControllerGraph;
        (0..DMA_EDGES).try_for_each(|k| {
            let s = g.source(Edge::new(k))?;
            let t = g.target(Edge::new(k))?;
            assert!(s.index() < DMA_VERTICES);
            assert!(t.index() < DMA_VERTICES);
            Ok(())
        })
    }

    #[test]
    fn single_transfer_is_round_trip() -> Result<(), FreeCategoryError> {
        let path = single_transfer_path()?;
        assert_eq!(path.source().index(), 0);
        assert_eq!(path.target().index(), 0);
        assert_eq!(path.len(), 6);
        Ok(())
    }

    #[test]
    fn read_error_is_round_trip() -> Result<(), FreeCategoryError> {
        let path = read_error_path()?;
        assert_eq!(path.source().index(), 0);
        assert_eq!(path.target().index(), 0);
        assert_eq!(path.len(), 4);
        Ok(())
    }

    #[test]
    fn loop_edge_returns_to_read_phase() -> Result<(), FreeCategoryError> {
        let g = DmaControllerGraph;
        assert_eq!(g.source(Edge::new(4))?.index(), 4); // CheckDone
        assert_eq!(g.target(Edge::new(4))?.index(), 2); // ReadPhase
        Ok(())
    }
}
