# axi-dma-cat

AXI4-Lite bus bridge and DMA engine in `RustHDL`, with protocol
FSMs modeled as free category graphs via `comp-cat-rs`.

## Overview

- **AXI4-Lite master and slave**: 5-channel VALID/READY handshaking
  (AW, W, B, AR, R), 32-bit address, 32-bit data, full write strobes,
  16-register slave with independent write and read FSMs.
- **DMA engine**: register-mapped via AXI-Lite slave (control, status,
  source, destination, length), golden model with memcpy semantics,
  scatter-gather descriptor support (future).
- **Categorical FSM modeling**: write/read handshaking and DMA
  controller state machines as `comp_cat_rs::collapse::free_category::Graph`.
  `interpret()` composes transitions into full transfer sequences.
- **Monoidal composition**: tensor product models AXI channel
  independence (`AW ⊗ W`); DMA pipeline composes register interface
  with data interface via `Controller ; (Registers ⊗ DataPath)`.
- **Property-based tests**: `proptest` suites for write-read
  round-trip, response code invariants, DMA data integrity, transfer
  length, zero-length handling, in-place copy.

## Architecture

```text
Layer 1 (Pure)                          Layer 2 (HDL)
----------------------------------      ----------------------------
primitives/                             axi_lite/hdl/
  AxiAddress, AxiResponse,                bus, master, slave
  WriteStrobe, AxiProt

axi_lite/                               dma/hdl/
  channel, transaction                    register_file, dma_engine
  graph/ (write 5V/5E, read 4V/4E)
  interpret/ (AxiLiteAction)
  sim/ (RegisterFile golden, Io runner)

dma/                                    composition/
  register (Control, Status, Length)       axi_channel_tensor
  transaction (DmaTransferDescriptor)     dma_pipeline
  graph/ (controller 7V/10E)
  interpret/ (DmaAction)
  sim/ (MemoryModel golden, Io runner)
```

## AXI4-Lite Protocol

| Channel | Signals | Handshake |
|---------|---------|-----------|
| Write Address (AW) | AWADDR, AWPROT, AWVALID/AWREADY | master -> slave |
| Write Data (W) | WDATA, WSTRB, WVALID/WREADY | master -> slave |
| Write Response (B) | BRESP, BVALID/BREADY | slave -> master |
| Read Address (AR) | ARADDR, ARPROT, ARVALID/ARREADY | master -> slave |
| Read Data (R) | RDATA, RRESP, RVALID/RREADY | slave -> master |

## DMA Register Map

| Offset | Register | Bits |
|--------|----------|------|
| 0x00 | Control | start(0), stop(1), irq_en(2) |
| 0x04 | Status | busy(0), done(1), error(2), irq(3) |
| 0x08 | Source Address | 32-bit |
| 0x0C | Dest Address | 32-bit |
| 0x10 | Transfer Length | 32-bit (bytes) |

## Building

```sh
cargo build
cargo test
RUSTFLAGS="-D warnings" cargo clippy
cargo doc --no-deps --open
```

## Testing

78 tests across four levels:

- **Unit tests** (62): primitives, graphs, interpretations, golden
  models, composition, HDL module construction.
- **Property tests** (8): AXI-Lite write-read round-trip, response
  codes, address independence; DMA data integrity, transfer length,
  zero-length, in-place copy.
- **Integration tests** (3): end-to-end AXI-Lite sequence via `Io`,
  DMA transfer via `Io`, full pipeline (configure + transfer).
- **Doctests** (5): `AxiAddress`, `AxiResponse`, `RegisterFile`,
  `DmaTransferDescriptor`, `dma_transfer_golden`.

## License

Licensed under either of

- MIT license
- Apache License, Version 2.0

at your option.
