use matrix;
use std::io::Result;
use std::path::Path;
use std::{fs, mem};
use threed_ice_sys::*;

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
    /// The thermal capacitance matrix, which is diagonal, and, hence, only
    /// diagonal elements are stored.
    pub capacitance: Vec<f64>,
    /// The thermal conductance matrix, which is sparse, and, hence, only
    /// nonzero elements are stored.
    pub conductance: matrix::Compressed<f64>,
}

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

    Ok(Circuit {
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
                              analysis: &mut Analysis_t) -> Result<matrix::Compressed<f64>> {

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

    thermal_grid_destroy(&mut grid);
    system_matrix_destroy(&mut matrix);

    Ok(matrix::Compressed {
        rows: dimension,
        columns: dimension,
        nonzeros: nonzeros,
        format: matrix::CompressedFormat::Column,
        data: values,
        indices: indices,
        offsets: offsets,
    })
}
