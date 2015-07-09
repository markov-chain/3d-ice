extern crate matrix;
extern crate threed_ice_sys;

macro_rules! raise(
    () => (raise!(Other, "failed to call a 3D-ICE function"));
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
    ($result:expr) => ($result != ::threed_ice_sys::TDICE_SUCCESS);
);

macro_rules! str_to_c_str(
    ($str:expr) => (ok!(::std::ffi::CString::new($str)));
);

macro_rules! path_to_c_str(
    ($path:expr) => (str_to_c_str!(some!($path.to_str())));
);

mod circuit;

pub use circuit::Circuit;
