extern crate fixture;
extern crate temporary;
extern crate threed_ice;

use std::path::{Path, PathBuf};
use temporary::Directory;

use threed_ice::Circuit;

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn circuit_new() {
    setup(None, |stack| {
        let _circuit = ok!(Circuit::new(stack));
    })
}

fn setup<F>(name: Option<&str>, mut code: F) where F: FnMut(&Path) {
    let source = find(name.unwrap_or("default"));
    let directory = ok!(Directory::new("threed_ice"));
    let destination = directory.path().join(ok!(source.file_name()));
    ok!(fixture::copy::with_references(&source, &destination));
    code(&destination)
}

fn find(name: &str) -> PathBuf {
    let path = PathBuf::from("tests/fixtures").join(name);
    match fixture::find::with_extension(&path, "stk") {
        Some(path) => path,
        None => panic!("cannot find a stack description in {:?}", path),
    }
}
