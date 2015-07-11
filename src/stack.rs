use ffi;
use std::mem;

use Result;
use die::{self, Die};

/// A stack.
pub struct Stack {
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

impl Stack {
    /// Extract the elements.
    pub fn elements(&self) -> Result<Vec<StackElement>> {
        unsafe { extract_elements(&self.raw) }
    }

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

impl Drop for Stack {
    fn drop(&mut self) {
        unsafe { ffi::stack_description_destroy(&mut self.raw) };
    }
}

implement_raw!(Stack, ffi::StackDescription_t);

pub unsafe fn new() -> Result<Stack> {
    let mut raw = mem::uninitialized();
    ffi::stack_description_init(&mut raw);
    Ok(Stack { raw: raw })
}

unsafe fn extract_elements(raw: &ffi::StackDescription_t) -> Result<Vec<StackElement>> {
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

    Ok(elements)
}
