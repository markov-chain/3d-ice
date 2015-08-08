use ffi;

use floorplan::{self, Floorplan};

/// A die contained in a stack.
#[derive(Clone, Debug)]
pub struct Die {
    /// The identifier.
    pub id: String,
    /// The floorplan.
    pub floorplan: Floorplan,
}

pub unsafe fn new(raw: &ffi::Die_t) -> Die {
    Die {
        id: c_str_to_string!(raw.Id),
        floorplan: floorplan::new(&raw.Floorplan),
    }
}
