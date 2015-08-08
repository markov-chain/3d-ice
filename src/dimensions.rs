use ffi;

/// The dimensions of a stack.
pub struct Dimensions {
    raw: *mut ffi::Dimensions_t,
}

impl Dimensions {
    /// Return the number of columns.
    #[inline]
    pub fn columns(&self) -> usize {
        unsafe { ffi::get_number_of_columns(self.raw) as usize }
    }

    /// Return the number of connections.
    #[inline]
    pub fn connections(&self) -> usize {
        unsafe { ffi::get_number_of_connections(self.raw)  as usize }
    }

    /// Return the number of layers.
    #[inline]
    pub fn layers(&self) -> usize {
        unsafe { ffi::get_number_of_layers(self.raw) as usize }
    }

    /// Return the number of rows.
    #[inline]
    pub fn rows(&self) -> usize {
        unsafe { ffi::get_number_of_rows(self.raw) as usize }
    }
}

pub fn new(raw: *mut ffi::Dimensions_t) -> Dimensions {
    Dimensions { raw: raw }
}
