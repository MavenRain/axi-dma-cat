//! AXI-Lite `RustHDL` hardware modules.
//!
//! All `mut` is quarantined inside `Logic::update` methods.

pub mod bus;
pub mod master;
pub mod slave;
