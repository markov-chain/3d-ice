use ffi;

/// Dimensions.
#[derive(Clone, Copy)]
pub struct Dimensions<'l> {
    raw: &'l ffi::Dimensions_t,
}

impl<'l> Dimensions<'l> {
    /// Return the number of layers.
    #[inline]
    pub fn layers(&self) -> usize {
        unsafe { ffi::get_number_of_layers(self.raw as *const _ as *mut _) as usize }
    }

    /// Return the number of rows.
    #[inline]
    pub fn rows(&self) -> usize {
        unsafe { ffi::get_number_of_rows(self.raw as *const _ as *mut _) as usize }
    }

    /// Return the number of columns.
    #[inline]
    pub fn columns(&self) -> usize {
        unsafe { ffi::get_number_of_columns(self.raw as *const _ as *mut _) as usize }
    }

    /// Return the number of connections.
    #[inline]
    pub fn connections(&self) -> usize {
        unsafe { ffi::get_number_of_connections(self.raw as *const _ as *mut _)  as usize }
    }
}

pub fn new<'l>(raw: &'l ffi::Dimensions_t) -> Dimensions<'l> {
    Dimensions { raw: raw }
}
