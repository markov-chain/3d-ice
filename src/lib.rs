extern crate matrix;
extern crate superlu;
extern crate threed_ice_sys as ffi;

macro_rules! raise(
    ($message:expr) => (raise!(Other, $message));
    ($kind:ident, $message:expr) => (
        return Err(::std::io::Error::new(::std::io::ErrorKind::$kind, $message))
    );
);

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(ok) => ok,
        _ => raise!("something went wrong"),
    });
);

macro_rules! some(
    ($result:expr) => (match $result {
        Some(some) => some,
        _ => raise!("something went wrong"),
    });
);

macro_rules! success(
    ($result:expr, $message:expr) => (if $result != ::ffi::TDICE_SUCCESS {
        raise!(concat!("failed to ", $message));
    });
);

macro_rules! c_str_to_string(
    ($string:expr) => (
        String::from_utf8_lossy(::std::ffi::CStr::from_ptr($string as *const _).to_bytes())
               .into_owned()
    );
);

macro_rules! str_to_cstr(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_cstr(
    ($path:expr) => (str_to_cstr!(some!($path.to_str())));
);

macro_rules! slice(
    ($pointer:expr, $size:expr) => (unsafe {
        ::std::slice::from_raw_parts($pointer, $size)
    });
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
            fn raw_mut(&mut self) -> &mut Self::Target {
                &mut self.raw
            }
        }
    );
    ($kind:ident, $target:ty, l) => (
        impl<'l> ::Raw for $kind<'l> {
            type Target = $target;

            #[inline(always)]
            fn raw(&self) -> &Self::Target {
                &self.raw
            }

            #[inline(always)]
            fn raw_mut(&mut self) -> &mut Self::Target {
                &mut self.raw
            }
        }
    );
);

mod analysis;
mod die;
mod dimensions;
mod floorplan;
mod output;
mod power_grid;
mod stack;
mod system;
mod system_matrix;
mod thermal_grid;

pub use analysis::{Analysis, AnalysisType};
pub use die::Die;
pub use dimensions::Dimensions;
pub use floorplan::{Floorplan, FloorplanElement};
pub use stack::{Stack, StackElement};
pub use system::System;
