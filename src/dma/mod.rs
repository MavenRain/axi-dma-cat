//! DMA engine domain.
//!
//! Provides register map newtypes, transfer descriptors, controller
//! FSM graph, categorical interpretation, `RustHDL` modules, and
//! behavioral simulation.

pub mod graph;
pub mod hdl;
pub mod interpret;
pub mod register;
pub mod sim;
pub mod transaction;
