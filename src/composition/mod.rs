//! Bus composition via monoidal category structure.
//!
//! - [`axi_channel_tensor`]: Tensor product for independent AXI-Lite channels.
//! - [`dma_pipeline`]: DMA + AXI-Lite pipeline composition.

pub mod axi_channel_tensor;
pub mod dma_pipeline;
