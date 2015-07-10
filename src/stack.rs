use ffi;

use Result;
use die::{self, Die};

/// A stack description.
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

pub unsafe fn new(stack: &ffi::StackDescription_t) -> Result<Stack> {
    let mut elements = vec![];
    let mut cursor = stack.StackElements.First;
    for _ in 0..stack.StackElements.Size {
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
                elements.push(StackElement::Die(try!(die::new(&*element.Pointer.Die()))));
            },
            ffi::TDICE_STACK_ELEMENT_HEATSINK => {
                elements.push(StackElement::HeatSink);
            },
        }
        cursor = (*cursor).Next;
    }

    Ok(Stack { elements: elements })
}
