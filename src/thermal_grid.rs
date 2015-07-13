use ffi;
use std::marker::PhantomData;
use std::mem;

use stack::Stack;
use {Raw, Result};

/// A thermal grid.
pub struct ThermalGrid<'l> {
    raw: ffi::ThermalGrid_t,
    phantom: PhantomData<&'l ffi::ThermalGrid_t>,
}

impl<'l> Drop for ThermalGrid<'l> {
    fn drop(&mut self) {
        unsafe { ffi::thermal_grid_destroy(&mut self.raw) };
    }
}

implement_raw!(ThermalGrid, ffi::ThermalGrid_t, l);

pub unsafe fn new<'l>(stack: &'l Stack) -> Result<ThermalGrid<'l>> {
    let mut raw = mem::uninitialized();
    ffi::thermal_grid_init(&mut raw);

    let stack = stack.raw();
    let layers = ffi::get_number_of_layers(stack.Dimensions);

    success!(ffi::thermal_grid_build(&mut raw, layers), "build the thermal grid");
    ffi::fill_thermal_grid(&mut raw, &stack.StackElements as *const _ as *mut _, stack.Dimensions);

    Ok(ThermalGrid { raw: raw, phantom: PhantomData })
}
