use ffi;
use matrix::{Compressed, CompressedFormat};
use std::mem;

use analysis::Analysis;
use stack_description::StackDescription;
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

pub unsafe fn new(description: &StackDescription, analysis: &Analysis, grid: &ThermalGrid)
                  -> Result<SystemMatrix> {

    let mut raw = mem::uninitialized();
    ffi::system_matrix_init(&mut raw);

    let description = description.raw();
    let cells = ffi::get_number_of_cells(description.Dimensions);
    let connections = ffi::get_number_of_connections(description.Dimensions);

    success!(ffi::system_matrix_build(&mut raw, cells, connections), "build the system matrix");
    ffi::fill_system_matrix(&mut raw, grid.raw() as *const _ as *mut _,
                            analysis.raw() as *const _ as *mut _, description.Dimensions);

    Ok(SystemMatrix { raw: raw })
}

pub unsafe fn convert(matrix: &SystemMatrix) -> Compressed<f64> {
    let matrix = matrix.raw();

    let dimension = matrix.Size as usize;
    let nonzeros = matrix.NNz as usize;

    let mut values = Vec::with_capacity(nonzeros);
    let mut indices = Vec::with_capacity(nonzeros);
    let mut offsets = Vec::with_capacity(dimension + 1);

    for i in 0..nonzeros {
        values.push(*matrix.Values.offset(i as isize));
        indices.push(*matrix.RowIndices.offset(i as isize) as usize);
    }
    for i in 0..(dimension + 1) {
        offsets.push(*matrix.ColumnPointers.offset(i as isize) as usize);
    }

    Compressed {
        rows: dimension,
        columns: dimension,
        nonzeros: nonzeros,
        format: CompressedFormat::Column,
        data: values,
        indices: indices,
        offsets: offsets,
    }
}
