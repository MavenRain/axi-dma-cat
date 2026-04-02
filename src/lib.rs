//! # axi-dma-cat
//!
//! AXI4-Lite bus bridge and DMA engine in `RustHDL`, with protocol
//! FSMs modeled as free category graphs via `comp-cat-rs`.
//!
//! ## Architecture
//!
//! ```text
//! primitives/    AxiAddress, AxiResponse, WriteStrobe, AxiProt
//! axi_lite/      Channel bundles, transactions, FSM graphs, HDL modules
//! dma/           Register map, controller FSM, DMA engine
//! composition/   Monoidal tensor for AXI channel independence
//! ```
//!
//! **Layer 1 (Pure):** Domain types, FSM graphs, interpretations,
//! golden models.  Zero `mut`, combinators only.
//!
//! **Layer 2 (HDL):** `RustHDL` `Logic` impls with `mut` quarantined.

pub mod axi_lite;
pub mod composition;
pub mod dma;
pub mod error;
pub mod primitives;
