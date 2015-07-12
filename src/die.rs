use ffi;

use floorplan::{self, Floorplan};

/// A die.
#[derive(Clone, Debug)]
pub struct Die {
    /// The identifier.
    pub name: String,
    /// The floorplan.
    pub floorplan: Floorplan,
}

pub unsafe fn new(raw: &ffi::Die_t) -> Die {
    Die {
        name: c_str_to_string!(raw.Id),
        floorplan: floorplan::new(&raw.Floorplan),
    }
}
