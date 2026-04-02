//! AXI4-Lite bus protocol domain.
//!
//! Provides channel signal bundles, transaction types, free category
//! graphs for write/read handshaking FSMs, categorical interpretation,
//! `RustHDL` modules, and behavioral simulation.

pub mod channel;
pub mod graph;
pub mod hdl;
pub mod interpret;
pub mod sim;
pub mod transaction;
