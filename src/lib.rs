extern crate threed_ice_sys as raw;

use raw::*;
use std::{fs, iter, mem, ptr};
use std::ffi::CString;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// A thermal RC circuit.
#[derive(Debug)]
pub struct Circuit {
    /// The number of layers.
    pub layers: usize,
    /// The number of rows per layer.
    pub rows: usize,
    /// The number of columns per layer.
    pub columns: usize,
    /// The number of cells, which is `layers × rows × columns`.
    pub cells: usize,
    /// The thermal capacitance matrix, which is diagonal, and, hence, only diagonal elements are
    /// stored.
    pub capacitance: Vec<f64>,
    /// The thermal conductance matrix, which is sparse, and, hence, only nonzero elements are
    /// stored.
    pub conductance: SparseMatrix,
}

/// A sparse matrix stored in the Harwell–Boeing format.
#[derive(Debug)]
pub struct SparseMatrix {
    /// The number of rows.
    pub rows: usize,
    /// The number of columns.
    pub columns: usize,
    /// The number of nonzero elements.
    pub nonzeros: usize,
    /// The values of the nonzero elements.
    pub values: Vec<f64>,
    /// The row indices of the nonzero elements.
    pub row_indices: Vec<usize>,
    /// The offsets of the columns such that the values and row indices of the `i`th column are
    /// stored starting from `values[j]` and `row_indices[j]`, respectively, where `j =
    /// column_offsets[i]`. The vector has one additional element, which is always equal to
    /// `nonzeros`, that is, `column_offsets[columns] = nonzeros`.
    pub column_offsets: Vec<usize>,
}

macro_rules! raise(
    () => (raise!(Other, "failed to call a 3D-ICE function"));
    ($message:expr) => (raise!(Other, $message));
    ($kind:ident, $message:expr) => (return Err(Error::new(ErrorKind::$kind, $message)));
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(ok) => ok,
            Err(_) => raise!("something went wrong"),
        }
    );
);

macro_rules! some(
    ($result:expr) => (
        match $result {
            Some(some) => some,
            None => raise!("something went wrong"),
        }
    );
);

macro_rules! failed(
    ($result:expr) => ($result != raw::TDICE_SUCCESS);
);

macro_rules! str_to_c_str(
    ($str:expr) => (ok!(CString::new($str)));
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!(some!($path.to_str())));
);

impl Circuit {
    /// Create a thermal RC circuit based on the 3D-ICE model.
    pub fn new(stack_description: &Path) -> Result<Circuit> {
        if fs::metadata(stack_description).is_err() {
            raise!("the stack description file does not exist");
        }
        unsafe { construct(stack_description) }
    }
}

unsafe fn construct(path: &Path) -> Result<Circuit> {
    let mut stack: StackDescription_t = mem::uninitialized();
    let mut analysis: Analysis_t = mem::uninitialized();
    let mut output: Output_t = mem::uninitialized();

    stack_description_init(&mut stack);
    analysis_init(&mut analysis);
    output_init(&mut output);

    let mut cleanup = move || {
        stack_description_destroy(&mut stack);
        analysis_destroy(&mut analysis);
        output_destroy(&mut output);
    };

    if failed!(parse_stack_description_file(path_to_c_str!(path).as_ptr() as *mut _, &mut stack,
                                            &mut analysis, &mut output)) {

        cleanup();
        raise!();
    }

    let cells = get_number_of_cells(stack.Dimensions);
    let columns = get_number_of_columns(stack.Dimensions);
    let layers = get_number_of_layers(stack.Dimensions);
    let rows = get_number_of_rows(stack.Dimensions);

    let capacitance = match extract_capacitance(&mut stack) {
        Ok(capacitance) => capacitance,
        Err(error) => {
            cleanup();
            return Err(error);
        },
    };

    let conductance = match extract_conductance(&mut stack, &mut analysis) {
        Ok(conductance) => conductance,
        Err(error) => {
            cleanup();
            return Err(error);
        },
    };

    cleanup();

    Ok(Circuit{
        layers: layers as usize,
        rows: rows as usize,
        columns: columns as usize,
        cells: cells as usize,
        capacitance: capacitance,
        conductance: conductance,
    })
}

unsafe fn extract_capacitance(stack: &mut StackDescription_t) -> Result<Vec<f64>> {
    let mut grid: ThermalGrid_t = mem::uninitialized();

    let cells = get_number_of_cells(stack.Dimensions);
    let columns = get_number_of_columns(stack.Dimensions);
    let layers = get_number_of_layers(stack.Dimensions);
    let rows = get_number_of_rows(stack.Dimensions);

    thermal_grid_init(&mut grid);

    if failed!(thermal_grid_build(&mut grid, layers)) {
        raise!();
    }

    fill_thermal_grid(&mut grid, &mut stack.StackElements, stack.Dimensions);

    let mut capacitance = Vec::with_capacity(cells as usize);
    for layer in 0..layers {
        for row in 0..rows {
            for column in 0..columns {
                capacitance.push(get_capacity(&mut grid, stack.Dimensions, layer, row, column));
            }
        }
    }

    thermal_grid_destroy(&mut grid);

    Ok(capacitance)
}

unsafe fn extract_conductance(stack: &mut StackDescription_t,
                              analysis: &mut Analysis_t) -> Result<SparseMatrix> {

    let mut grid: ThermalGrid_t = mem::uninitialized();
    let mut matrix: SystemMatrix_t = mem::uninitialized();

    let cells = get_number_of_cells(stack.Dimensions);
    let connections = get_number_of_connections(stack.Dimensions);
    let layers = get_number_of_layers(stack.Dimensions);

    thermal_grid_init(&mut grid);
    if failed!(thermal_grid_build(&mut grid, layers)) {
        raise!();
    }
    fill_thermal_grid(&mut grid, &mut stack.StackElements, stack.Dimensions);

    system_matrix_init(&mut matrix);
    if failed!(system_matrix_build(&mut matrix, cells, connections)) {
        thermal_grid_destroy(&mut grid);
        raise!();
    }
    fill_system_matrix(&mut matrix, &mut grid, analysis, stack.Dimensions);

    let dimension = cells as usize;
    let nonzeros = connections as usize;

    let mut values = iter::repeat(0.0).take(nonzeros).collect::<Vec<f64>>();
    let mut row_indices = iter::repeat(0).take(nonzeros).collect::<Vec<usize>>();
    let mut column_offsets = iter::repeat(0).take(dimension + 1).collect::<Vec<usize>>();

    ptr::copy_nonoverlapping(matrix.Values as *const _,
                             values.as_mut_ptr(), nonzeros);
    ptr::copy_nonoverlapping(matrix.RowIndices as *const _,
                             row_indices.as_mut_ptr(), nonzeros);
    ptr::copy_nonoverlapping(matrix.ColumnPointers as *const _,
                             column_offsets.as_mut_ptr(), dimension + 1);

    thermal_grid_destroy(&mut grid);
    system_matrix_destroy(&mut matrix);

    Ok(SparseMatrix {
        rows: dimension,
        columns: dimension,
        nonzeros: nonzeros,
        values: values,
        row_indices: row_indices,
        column_offsets: column_offsets,
    })
}
