use ffi;
use std::mem;

use Result;

/// A type of temperature analysis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnalysisType {
    /// The steady-state analysis.
    Steady,
    /// The transient analysis.
    Transient,
    /// An undefined analysis.
    None,
}

/// Temperature analysis.
pub struct Analysis {
    raw: ffi::Analysis_t,
}

impl Analysis {
    /// Return the type.
    pub fn kind(&self) -> AnalysisType {
        match self.raw.AnalysisType {
            ffi::TDICE_ANALYSIS_TYPE_STEADY => AnalysisType::Steady,
            ffi::TDICE_ANALYSIS_TYPE_TRANSIENT => AnalysisType::Transient,
            ffi::TDICE_ANALYSIS_TYPE_NONE => AnalysisType::None,
        }
    }
}

impl Drop for Analysis {
    fn drop(&mut self) {
        unsafe { ffi::analysis_destroy(&mut self.raw) };
    }
}

implement_raw!(Analysis, ffi::Analysis_t);

pub unsafe fn new() -> Result<Analysis> {
    let mut raw = mem::uninitialized();
    ffi::analysis_init(&mut raw);
    Ok(Analysis { raw: raw })
}
