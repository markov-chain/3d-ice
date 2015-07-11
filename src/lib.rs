extern crate matrix;
extern crate threed_ice_sys as ffi;

macro_rules! raise(
    ($message:expr) => (raise!(Other, $message));
    ($kind:ident, $message:expr) => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::$kind, $message))
    );
);

macro_rules! ok(
    ($result:expr) => (
        match $result {
            Ok(ok) => ok,
            Err(_) => raise!("something went wrong"),
        }
    );
);

macro_rules! some(
    ($result:expr) => (
        match $result {
            Some(some) => some,
            None => raise!("something went wrong"),
        }
    );
);

macro_rules! failed(
    ($result:expr) => ($result != ::ffi::TDICE_SUCCESS);
);

macro_rules! c_str_to_string(
    ($string:expr) => (
        String::from_utf8_lossy(::std::ffi::CStr::from_ptr($string as *const _).to_bytes())
               .into_owned()
    );
);

macro_rules! str_to_c_str(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!(some!($path.to_str())));
);

/// An error.
pub type Error = std::io::Error;

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

trait Raw {
    type Target;

    fn raw(&self) -> &Self::Target;
    fn raw_mut(&mut self) -> &mut Self::Target;
}

macro_rules! implement_raw(
    ($kind:ty, $target:ty) => (
        impl ::Raw for $kind {
            type Target = $target;

            #[inline(always)]
            fn raw(&self) -> &Self::Target {
                &self.raw
            }

            #[inline(always)]
            fn raw_mut<'l>(&mut self) -> &mut Self::Target {
                &mut self.raw
            }
        }
    );
);

mod analysis;
mod die;
mod floorplan;
mod output;
mod stack_description;
mod system;

pub use die::Die;
pub use floorplan::{Floorplan, FloorplanElement};
pub use stack_description::{StackDescription, StackElement};
pub use system::System;
