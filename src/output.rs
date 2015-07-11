use ffi;
use std::mem;

use Result;

pub struct Output {
    raw: ffi::Output_t,
}

impl Drop for Output {
    fn drop(&mut self) {
        unsafe { ffi::output_destroy(&mut self.raw) };
    }
}

implement_raw!(Output, ffi::Output_t);

pub unsafe fn new() -> Result<Output> {
    let mut raw = mem::uninitialized();
    ffi::output_init(&mut raw);
    Ok(Output { raw: raw })
}
