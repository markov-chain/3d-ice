use ffi;
use std::mem;

use analysis::Analysis;
use stack::Stack;
use thermal_grid::ThermalGrid;
use {Raw, Result};

/// A system matrix.
pub struct SystemMatrix {
    raw: ffi::SystemMatrix_t,
}

impl Drop for SystemMatrix {
    fn drop(&mut self) {
        unsafe { ffi::system_matrix_destroy(&mut self.raw) };
    }
}

implement_raw!(SystemMatrix, ffi::SystemMatrix_t);

pub unsafe fn new(stack: &Stack, analysis: &Analysis, grid: &ThermalGrid) -> Result<SystemMatrix> {
    let mut raw = mem::uninitialized();
    ffi::system_matrix_init(&mut raw);

    let stack = stack.raw();
    let cells = ffi::get_number_of_cells(stack.Dimensions);
    let connections = ffi::get_number_of_connections(stack.Dimensions);

    success!(ffi::system_matrix_build(&mut raw, cells, connections), "build the system matrix");
    ffi::fill_system_matrix(&mut raw, grid.raw() as *const _ as *mut _,
                            analysis.raw() as *const _ as *mut _, stack.Dimensions);

    Ok(SystemMatrix { raw: raw })
}
