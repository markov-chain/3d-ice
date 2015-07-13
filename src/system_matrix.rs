use ffi;
use matrix::Compressed;
use std::convert::From;
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

impl From<SystemMatrix> for Compressed<f64> {
    fn from(matrix: SystemMatrix) -> Compressed<f64> {
        use matrix::CompressedFormat::Column;

        let raw = matrix.raw();

        let size = raw.Size as usize;
        let nonzeros = raw.NNz as usize;

        let mut values = Vec::with_capacity(nonzeros);
        let mut indices = Vec::with_capacity(nonzeros);
        let mut offsets = Vec::with_capacity(size + 1);

        unsafe {
            for i in 0..(nonzeros as isize) {
                values.push(*raw.Values.offset(i));
                indices.push(*raw.RowIndices.offset(i) as usize);
            }
            for i in 0..(size as isize + 1) {
                offsets.push(*raw.ColumnPointers.offset(i) as usize);
            }
        }

        Compressed {
            rows: size,
            columns: size,
            nonzeros: nonzeros,
            format: Column,
            data: values,
            indices: indices,
            offsets: offsets,
        }
    }
}

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
