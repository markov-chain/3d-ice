use ffi;

/// The floorplan of the source layer of a die.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Floorplan {
    /// The list of elements.
    pub elements: Vec<FloorplanElement>,
}

/// An element of a floorplan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FloorplanElement {
    /// The identifier.
    pub id: String,
}

pub unsafe fn new(raw: &ffi::Floorplan_t) -> Floorplan {
    let mut elements = vec![];
    let mut cursor = raw.ElementsList.First;
    for _ in 0..raw.ElementsList.Size {
        assert!(!cursor.is_null());
        elements.push(new_element(&(*cursor).Data));
        cursor = (*cursor).Next;
    }
    Floorplan { elements: elements }
}

unsafe fn new_element(raw: &ffi::FloorplanElement_t) -> FloorplanElement {
    FloorplanElement { id: c_str_to_string!(raw.Id) }
}
