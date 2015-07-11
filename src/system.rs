use ffi;
use matrix::Compressed;
use std::path::Path;

use analysis:: Analysis;
use power_grid::{self, PowerGrid};
use stack_description::{self, StackDescription};
use {Raw, Result};
use {system_matrix, thermal_grid};

/// A system.
pub struct System {
    description: StackDescription,
    analysis: Analysis,
}

impl System {
    /// Create a system given a stack description.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<System> {
        unsafe {
            let (description, analysis, _) = try!(stack_description::new(path.as_ref()));
            Ok(System { description: description, analysis: analysis })
        }
    }

    /// Extract the thermal capacitance matrix.
    ///
    /// The matrix is diagonal, and, hence, only diagonal elements are stored.
    #[inline]
    pub fn capacitance(&self) -> Result<Vec<f64>> {
        unsafe { extract_capacitance(self) }
    }

    /// Extract the thermal conductance matrix.
    ///
    /// The matrix is sparse, and, hence, only nonzero elements are stored.
    #[inline]
    pub fn conductance(&self) -> Result<Compressed<f64>> {
        unsafe { extract_conductance(self) }
    }

    /// Construct a power grid.
    #[inline]
    pub fn power_grid<'l>(&'l self) -> Result<PowerGrid<'l>> {
        unsafe { power_grid::new(&self.description) }
    }

    /// Return the stack description.
    #[inline]
    pub fn stack_description(&self) -> &StackDescription {
        &self.description
    }
}

unsafe fn extract_capacitance(system: &System) -> Result<Vec<f64>> {
    let grid = try!(thermal_grid::new(&system.description));
    let grid = grid.raw();

    let description = system.description.raw();
    let cells = ffi::get_number_of_cells(description.Dimensions);
    let columns = ffi::get_number_of_columns(description.Dimensions);
    let layers = ffi::get_number_of_layers(description.Dimensions);
    let rows = ffi::get_number_of_rows(description.Dimensions);

    let mut capacitance = Vec::with_capacity(cells as usize);
    for i in 0..layers {
        for j in 0..rows {
            for k in 0..columns {
                capacitance.push(ffi::get_capacity(grid as *const _ as *mut _,
                                                   description.Dimensions, i, j, k));
            }
        }
    }

    Ok(capacitance)
}

unsafe fn extract_conductance(system: &System) -> Result<Compressed<f64>> {
    let grid = try!(thermal_grid::new(&system.description));
    let matrix = try!(system_matrix::new(&system.description, &system.analysis, &grid));
    Ok(system_matrix::convert(&matrix))
}
