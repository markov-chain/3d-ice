use ffi;

use Result;
use floorplan::{self, Floorplan};

/// A die.
#[derive(Clone, Debug)]
pub struct Die {
    /// The identifier.
    pub name: String,
    /// The floorplan.
    pub floorplan: Floorplan,
}

pub unsafe fn new(raw: &ffi::Die_t) -> Result<Die> {
    Ok(Die {
        name: c_str_to_string!(raw.Id),
        floorplan: try!(floorplan::new(&raw.Floorplan)),
    })
}
