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

implement_raw!(Analysis, ffi::Analysis_t);

pub unsafe fn new() -> Result<Analysis> {
    let mut raw = mem::uninitialized();
    ffi::analysis_init(&mut raw);
    Ok(Analysis { raw: raw })
}
