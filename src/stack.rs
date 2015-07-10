use ffi::*;
use std::io::Result;
use std::path::Path;
use std::{fs, mem};

/// A stack.
#[derive(Clone, Debug)]
pub struct Stack {
    /// The list of elements.
    pub elements: Vec<StackElement>,
}

/// A stack element.
#[derive(Clone, Debug)]
pub enum StackElement {
    None,
    Layer,
    Channel,
    Die(Die),
    HeatSink,
}

/// A die.
#[derive(Clone, Debug)]
pub struct Die {
    /// The identifier.
    pub name: String,
    /// The floorplan.
    pub floorplan: Floorplan,
}

/// A floorplan.
#[derive(Clone, Debug)]
pub struct Floorplan {
    /// The list of elements.
    pub elements: Vec<FloorplanElement>,
}

/// A floorplan element.
#[derive(Clone, Debug)]
pub struct FloorplanElement {
    /// The identifier.
    pub name: String,
}

impl Stack {
    /// Read a stack description.
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Stack> {
        let path = path.as_ref();
        if fs::metadata(path).is_err() {
            raise!("the stack description file does not exist");
        }
        unsafe { read(path) }
    }
}

unsafe fn read(path: &Path) -> Result<Stack> {
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

    let mut elements = vec![];
    let mut cursor = stack.StackElements.First;
    for _ in 0..stack.StackElements.Size {
        assert!(!cursor.is_null());
        let element = &(*cursor).Data;
        match element.Type {
            TDICE_STACK_ELEMENT_NONE => {
                elements.push(StackElement::None);
            },
            TDICE_STACK_ELEMENT_LAYER => {
                elements.push(StackElement::Layer);
            },
            TDICE_STACK_ELEMENT_CHANNEL => {
                elements.push(StackElement::Channel);
            },
            TDICE_STACK_ELEMENT_DIE => match read_die(&*element.Pointer.Die()) {
                Ok(die) => elements.push(StackElement::Die(die)),
                Err(error) => {
                    cleanup();
                    return Err(error);
                },
            },
            TDICE_STACK_ELEMENT_HEATSINK => {
                elements.push(StackElement::HeatSink);
            },
        }
        cursor = (*cursor).Next;
    }

    cleanup();

    Ok(Stack { elements: elements })
}

unsafe fn read_die(die: &Die_t) -> Result<Die> {
    Ok(Die {
        name: c_str_to_string!(die.Id),
        floorplan: try!(read_floorplan(&die.Floorplan)),
    })
}

unsafe fn read_floorplan(floorplan: &Floorplan_t) -> Result<Floorplan> {
    let mut elements = vec![];
    let mut cursor = floorplan.ElementsList.First;
    for _ in 0..floorplan.ElementsList.Size {
        assert!(!cursor.is_null());
        elements.push(try!(read_element(&(*cursor).Data)));
        cursor = (*cursor).Next;
    }
    Ok(Floorplan { elements: elements })
}

unsafe fn read_element(element: &FloorplanElement_t) -> Result<FloorplanElement> {
    Ok(FloorplanElement { name: c_str_to_string!(element.Id) })
}
