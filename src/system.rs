use ffi;
use matrix::{Compressed, CompressedFormat};
use std::path::Path;
use std::{fs, mem};

use analysis::{self, Analysis};
use output::{self, Output};
use stack::{self, Stack};
use {Raw, Result};

/// A system.
pub struct System {
    stack: Stack,
    analysis: Analysis,
    output: Output,
}

impl System {
    /// Create a system given a stack description.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<System> {
        let path = path.as_ref();
        if fs::metadata(path).is_err() {
            raise!("the stack-description file does not exist");
        }

        unsafe {
            let mut system = System {
                stack: try!(stack::new()),
                analysis: try!(analysis::new()),
                output: try!(output::new()),
            };

            if failed!(ffi::parse_stack_description_file(path_to_c_str!(path).as_ptr() as *mut _,
                                                         system.stack.raw_mut(),
                                                         system.analysis.raw_mut(),
                                                         system.output.raw_mut())) {
                raise!("failed to parse the stack-description file");
            }

            Ok(system)
        }
    }

    /// Extract the thermal capacitance matrix.
    ///
    /// The matrix is diagonal, and, hence, only diagonal elements are stored.
    #[inline]
    pub fn capacitance(&self) -> Result<Vec<f64>> {
        unsafe { extract_capacitance(self.stack.raw()) }
    }

    /// Extract the thermal conductance matrix.
    ///
    /// The matrix is sparse, and, hence, only nonzero elements are stored.
    #[inline]
    pub fn conductance(&self) -> Result<Compressed<f64>> {
        unsafe { extract_conductance(self.stack.raw(), self.analysis.raw()) }
    }

    /// Return the stack description.
    #[inline]
    pub fn stack(&self) -> &Stack {
        &self.stack
    }
}

unsafe fn extract_capacitance(stack: &ffi::StackDescription_t) -> Result<Vec<f64>> {
    let mut grid: ffi::ThermalGrid_t = mem::uninitialized();

    let cells = ffi::get_number_of_cells(stack.Dimensions);
    let columns = ffi::get_number_of_columns(stack.Dimensions);
    let layers = ffi::get_number_of_layers(stack.Dimensions);
    let rows = ffi::get_number_of_rows(stack.Dimensions);

    ffi::thermal_grid_init(&mut grid);

    if failed!(ffi::thermal_grid_build(&mut grid, layers)) {
        raise!("failed to build the thermal grid");
    }

    ffi::fill_thermal_grid(&mut grid, &stack.StackElements as *const _ as *mut _,
                           stack.Dimensions);

    let mut capacitance = Vec::with_capacity(cells as usize);
    for layer in 0..layers {
        for row in 0..rows {
            for column in 0..columns {
                capacitance.push(ffi::get_capacity(&mut grid, stack.Dimensions,
                                                   layer, row, column));
            }
        }
    }

    ffi::thermal_grid_destroy(&mut grid);

    Ok(capacitance)
}

unsafe fn extract_conductance(stack: &ffi::StackDescription_t, analysis: &ffi::Analysis_t)
                              -> Result<Compressed<f64>> {

    let mut grid: ffi::ThermalGrid_t = mem::uninitialized();
    let mut matrix: ffi::SystemMatrix_t = mem::uninitialized();

    let cells = ffi::get_number_of_cells(stack.Dimensions);
    let connections = ffi::get_number_of_connections(stack.Dimensions);
    let layers = ffi::get_number_of_layers(stack.Dimensions);

    ffi::thermal_grid_init(&mut grid);
    if failed!(ffi::thermal_grid_build(&mut grid, layers)) {
        raise!("failed to build the thermal grid");
    }
    ffi::fill_thermal_grid(&mut grid, &stack.StackElements as *const _ as *mut _,
                           stack.Dimensions);

    ffi::system_matrix_init(&mut matrix);
    if failed!(ffi::system_matrix_build(&mut matrix, cells, connections)) {
        ffi::thermal_grid_destroy(&mut grid);
        raise!("failed to build the system matrix");
    }
    ffi::fill_system_matrix(&mut matrix, &mut grid, analysis as *const _ as *mut _,
                            stack.Dimensions);

    let dimension = cells as usize;
    let nonzeros = connections as usize;

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

    ffi::thermal_grid_destroy(&mut grid);
    ffi::system_matrix_destroy(&mut matrix);

    Ok(Compressed {
        rows: dimension,
        columns: dimension,
        nonzeros: nonzeros,
        format: CompressedFormat::Column,
        data: values,
        indices: indices,
        offsets: offsets,
    })
}
