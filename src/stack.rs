use ffi;

use Result;
use die::{self, Die};

/// A stack description.
#[derive(Clone)]
pub struct Stack<'l> {
    /// The list of elements.
    pub elements: Vec<StackElement>,

    raw: &'l ffi::StackDescription_t,
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

impl<'l> Stack<'l> {
    /// Return the number of layers.
    #[inline]
    pub fn layers(&self) -> usize {
        unsafe { ffi::get_number_of_layers(self.raw.Dimensions) as usize }
    }

    /// Return the number of rows per layer.
    #[inline]
    pub fn rows(&self) -> usize {
        unsafe { ffi::get_number_of_rows(self.raw.Dimensions) as usize }
    }

    /// Return the number of columns per layer.
    #[inline]
    pub fn columns(&self) -> usize {
        unsafe { ffi::get_number_of_columns(self.raw.Dimensions) as usize }
    }

    /// Return the number of cells, which is `layers × rows × columns`.
    #[inline]
    pub fn cells(&self) -> usize {
        unsafe { ffi::get_number_of_cells(self.raw.Dimensions) as usize }
    }
}

pub unsafe fn new<'l>(raw: &'l ffi::StackDescription_t) -> Result<Stack<'l>> {
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
                elements.push(StackElement::Die(try!(die::new(&*element.Pointer.Die()))));
            },
            ffi::TDICE_STACK_ELEMENT_HEATSINK => {
                elements.push(StackElement::HeatSink);
            },
        }
        cursor = (*cursor).Next;
    }

    Ok(Stack { elements: elements, raw: raw })
}
