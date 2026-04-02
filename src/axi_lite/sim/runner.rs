//! `Io`-wrapped AXI-Lite simulation.

use comp_cat_rs::effect::io::Io;

use crate::axi_lite::sim::golden::RegisterFile;
use crate::axi_lite::transaction::{
    AxiLiteReadRequest, AxiLiteReadResult,
    AxiLiteWriteRequest, AxiLiteWriteResult,
};
use crate::error::Error;

/// Configuration for a sequence of AXI-Lite operations.
#[derive(Debug, Clone)]
pub enum AxiLiteOp {
    /// Write operation.
    Write(AxiLiteWriteRequest),
    /// Read operation.
    Read(AxiLiteReadRequest),
}

/// Result of a single AXI-Lite operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AxiLiteOpResult {
    /// Write result.
    Write(AxiLiteWriteResult),
    /// Read result.
    Read(AxiLiteReadResult),
}

/// Build an `Io` that processes a sequence of AXI-Lite operations
/// against a golden register file model.
///
/// Nothing executes until [`Io::run`](comp_cat_rs::effect::io::Io::run).
#[must_use]
pub fn simulate_axi_lite(
    ops: Vec<AxiLiteOp>,
) -> Io<Error, Vec<AxiLiteOpResult>> {
    Io::suspend(move || {
        let (_, results) = ops.iter().fold(
            (RegisterFile::new(), Vec::with_capacity(ops.len())),
            |(rf, results), op| {
                let (new_rf, result) = match op {
                    AxiLiteOp::Write(req) => {
                        let (rf, wr) = rf.write(*req);
                        (rf, AxiLiteOpResult::Write(wr))
                    }
                    AxiLiteOp::Read(req) => {
                        let (rf, rd) = rf.read(*req);
                        (rf, AxiLiteOpResult::Read(rd))
                    }
                };
                let new_results = {
                    let mut r = results;
                    r.push(result);
                    r
                };
                (new_rf, new_results)
            },
        );
        Ok(results)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::axi_lite::transaction::AxiLiteReadResult;
    use crate::primitives::address::AxiAddress;

    #[test]
    fn write_then_read_via_io() -> Result<(), Error> {
        let ops = vec![
            AxiLiteOp::Write(AxiLiteWriteRequest::write32(AxiAddress::new(0x00), 0x42)),
            AxiLiteOp::Read(AxiLiteReadRequest::read(AxiAddress::new(0x00))),
        ];
        let results = simulate_axi_lite(ops).run()?;
        assert_eq!(results.len(), 2);

        let read_result = results.get(1).and_then(|r| match r {
            AxiLiteOpResult::Read(rd) => Some(rd),
            AxiLiteOpResult::Write(_) => None,
        });
        assert_eq!(read_result.map(AxiLiteReadResult::data), Some(0x42));
        Ok(())
    }
}
