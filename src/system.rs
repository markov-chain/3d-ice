use ffi;
use matrix::{Compressed, Diagonal, Make, Shape};
use std::path::Path;

use analysis:: Analysis;
use power_grid::{self, PowerGrid};
use stack::{self, Stack};
use {Raw, Result};
use {system_matrix, thermal_grid};

/// A system.
pub struct System {
    /// The stack.
    pub stack: Stack,

    analysis: Analysis,
}

impl System {
    /// Create a system given a stack description.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<System> {
        unsafe {
            let (stack, analysis, _) = try!(stack::new(path.as_ref()));
            Ok(System { stack: stack, analysis: analysis })
        }
    }

    /// Extract the thermal capacitance matrix.
    #[inline]
    pub fn capacitance(&self) -> Result<Diagonal<f64>> {
        unsafe { extract_capacitance(self) }
    }

    /// Extract the thermal conductance matrix.
    #[inline]
    pub fn conductance(&self) -> Result<Compressed<f64>> {
        unsafe { extract_conductance(self) }
    }

    /// Construct a power grid.
    #[inline]
    pub fn power_grid<'l>(&'l self) -> Result<PowerGrid<'l>> {
        unsafe { power_grid::new(&self.stack) }
    }
}

unsafe fn extract_capacitance(system: &System) -> Result<Diagonal<f64>> {
    let grid = try!(thermal_grid::new(&system.stack));
    let grid = grid.raw();

    let stack = system.stack.raw();
    let cells = ffi::get_number_of_cells(stack.Dimensions);
    let columns = ffi::get_number_of_columns(stack.Dimensions);
    let layers = ffi::get_number_of_layers(stack.Dimensions);
    let rows = ffi::get_number_of_rows(stack.Dimensions);

    let mut capacitance = Vec::with_capacity(cells as usize);
    for i in 0..layers {
        for j in 0..rows {
            for k in 0..columns {
                capacitance.push(ffi::get_capacity(grid as *const _ as *mut _,
                                                   stack.Dimensions, i, j, k));
            }
        }
    }

    Ok(Diagonal::make(capacitance, Shape::Square(cells as usize)))
}

unsafe fn extract_conductance(system: &System) -> Result<Compressed<f64>> {
    let grid = try!(thermal_grid::new(&system.stack));
    let matrix = try!(system_matrix::new(&system.stack, &system.analysis, &grid));
    Ok(matrix.into())
}
