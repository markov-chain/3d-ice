use ffi;

use Result;

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

pub unsafe fn new(floorplan: &ffi::Floorplan_t) -> Result<Floorplan> {
    let mut elements = vec![];
    let mut cursor = floorplan.ElementsList.First;
    for _ in 0..floorplan.ElementsList.Size {
        assert!(!cursor.is_null());
        elements.push(try!(new_element(&(*cursor).Data)));
        cursor = (*cursor).Next;
    }
    Ok(Floorplan { elements: elements })
}

unsafe fn new_element(element: &ffi::FloorplanElement_t) -> Result<FloorplanElement> {
    Ok(FloorplanElement { name: c_str_to_string!(element.Id) })
}