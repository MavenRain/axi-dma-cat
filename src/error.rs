//! Project-wide error type.

use comp_cat_rs::collapse::free_category::FreeCategoryError;

/// Unified error type for all operations in this crate.
#[derive(Debug)]
pub enum Error {
    /// AXI-Lite protocol error.
    AxiLite(String),
    /// DMA engine error.
    Dma(String),
    /// Free category graph error.
    Graph(FreeCategoryError),
    /// HDL simulation error.
    Simulation(String),
    /// Invalid AXI address.
    InvalidAddress { addr: u32 },
    /// Invalid AXI response code.
    InvalidResponse { code: u8 },
    /// DMA transfer error.
    TransferError { source: u32, dest: u32 },
    /// IO error.
    Io(std::io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AxiLite(msg) => write!(f, "AXI-Lite error: {msg}"),
            Self::Dma(msg) => write!(f, "DMA error: {msg}"),
            Self::Graph(e) => write!(f, "graph error: {e}"),
            Self::Simulation(msg) => write!(f, "simulation error: {msg}"),
            Self::InvalidAddress { addr } => write!(f, "invalid address: {addr:#010x}"),
            Self::InvalidResponse { code } => write!(f, "invalid response code: {code:#04x}"),
            Self::TransferError { source, dest } => {
                write!(f, "transfer error: src={source:#010x} dst={dest:#010x}")
            }
            Self::Io(e) => write!(f, "IO error: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::Graph(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<FreeCategoryError> for Error {
    fn from(e: FreeCategoryError) -> Self {
        Self::Graph(e)
    }
}
