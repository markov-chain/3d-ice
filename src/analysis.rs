use ffi;
use std::mem;

use Result;

pub struct Analysis {
    raw: ffi::Analysis_t,
}

impl Drop for Analysis {
    fn drop(&mut self) {
        unsafe { ffi::analysis_destroy(&mut self.raw) };
    }
}

pub unsafe fn new() -> Result<Analysis> {
    let mut raw = mem::uninitialized();
    ffi::analysis_init(&mut raw);
    Ok(Analysis { raw: raw })
}

#[inline(always)]
pub fn raw<'l>(analysis: &'l Analysis) -> &'l ffi::Analysis_t {
    &analysis.raw
}

#[inline(always)]
pub fn raw_mut<'l>(analysis: &'l mut Analysis) -> &'l mut ffi::Analysis_t {
    &mut analysis.raw
}
