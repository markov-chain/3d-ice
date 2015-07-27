use ffi;
use std::{fs, mem};
use std::path::Path;

use analysis::{self, Analysis};
use die::{self, Die};
use dimensions::{self, Dimensions};
use output::{self, Output};
use {Raw, Result};

/// A stack.
pub struct Stack {
    /// The dimensions.
    pub dimensions: Dimensions,
    /// The list of elements.
    pub elements: Vec<StackElement>,

    raw: ffi::StackDescription_t,
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

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { ffi::stack_description_destroy(&mut self.raw) };
    }
}

implement_raw!(Stack, ffi::StackDescription_t);

pub unsafe fn new(path: &Path) -> Result<(Stack, Analysis, Output)> {
    if fs::metadata(path).is_err() {
        raise!("the stack-description file does not exist");
    }

    let mut raw = mem::uninitialized();
    ffi::stack_description_init(&mut raw);

    let mut analysis = try!(analysis::new());
    let mut output = try!(output::new());

    success!(ffi::parse_stack_description_file(path_to_cstr!(path).as_ptr() as *mut _, &mut raw,
                                               analysis.raw_mut(), output.raw_mut()),
             "parse the stack-description file");

    let stack = Stack {
        dimensions: dimensions::new(raw.Dimensions),
        elements: extract_elements(&raw),
        raw: raw,
    };

    Ok((stack, analysis, output))
}

unsafe fn extract_elements(raw: &ffi::StackDescription_t) -> Vec<StackElement> {
    let mut elements = vec![];

    let mut cursor = raw.StackElements.First;
    for _ in 0..raw.StackElements.Size {
        assert!(!cursor.is_null());
        let element = &(*cursor).Data;
        match element.Type {
            ffi::TDICE_STACK_ELEMENT_NONE => {
                elements.push(StackElement::None);
            },
            ffi::TDICE_STACK_ELEMENT_LAYER => {
                elements.push(StackElement::Layer);
            },
            ffi::TDICE_STACK_ELEMENT_CHANNEL => {
                elements.push(StackElement::Channel);
            },
            ffi::TDICE_STACK_ELEMENT_DIE => {
                elements.push(StackElement::Die(die::new(&*element.Pointer.Die())));
            },
            ffi::TDICE_STACK_ELEMENT_HEATSINK => {
                elements.push(StackElement::HeatSink);
            },
        }
        cursor = (*cursor).Next;
    }

    elements
}
