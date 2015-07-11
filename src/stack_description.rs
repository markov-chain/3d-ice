use ffi;
use std::mem;

use Result;
use die::{self, Die};
use dimensions::{self, Dimensions};

/// A stack description.
pub struct StackDescription {
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

impl StackDescription {
    /// Extract the elements.
    pub fn elements(&self) -> Result<Vec<StackElement>> {
        unsafe { extract_elements(&self.raw) }
    }

    /// Return the dimensions.
    #[inline]
    pub fn dimensions<'l>(&'l self) -> Dimensions<'l> {
        unsafe { dimensions::new(&*self.raw.Dimensions) }
    }
}

impl Drop for StackDescription {
    fn drop(&mut self) {
        unsafe { ffi::stack_description_destroy(&mut self.raw) };
    }
}

implement_raw!(StackDescription, ffi::StackDescription_t);

pub unsafe fn new() -> Result<StackDescription> {
    let mut raw = mem::uninitialized();
    ffi::stack_description_init(&mut raw);
    Ok(StackDescription { raw: raw })
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
