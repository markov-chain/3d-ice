extern crate assert;
extern crate fixture;
extern crate matrix;
extern crate temporary;
extern crate threed_ice;

use std::path::{Path, PathBuf};
use temporary::Directory;

use threed_ice::{StackElement, System};

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn system_capacitance() {
    setup(None, |path| {
        let capacitance = ok!(ok!(System::new(path)).capacitance());
        assert::close(&capacitance.values, &vec![
            1.05000e-03, 1.05000e-03, 1.05000e-03, 1.05000e-03, 3.20000e-04, 3.20000e-04,
            3.20000e-04, 3.20000e-04, 7.98750e-01, 7.98750e-01, 7.98750e-01, 7.98750e-01,
            2.20455e+01, 2.20455e+01, 2.20455e+01, 2.20455e+01,
        ], 1e-15);
    });
}

#[test]
fn system_conductance() {
    setup(None, |path| {
        let conductance = ok!(ok!(System::new(path)).conductance());
        assert_eq!(conductance.rows, 4 * 2 * 2);
        assert_eq!(conductance.columns, 4 * 2 * 2);
        assert_eq!(conductance.nonzeros, 56);
        assert_eq!(conductance.format, matrix::compressed::Format::Column);
        assert::close(&conductance.values, &vec![
             2.080000000000000e+00, -1.500000000000000e-02, -1.500000000000000e-02,
            -1.000000000000000e+00, -1.500000000000000e-02,  2.080000000000000e+00,
            -1.500000000000000e-02, -1.000000000000000e+00, -1.500000000000000e-02,
             2.080000000000000e+00, -1.500000000000000e-02, -1.000000000000000e+00,
            -1.500000000000000e-02, -1.500000000000000e-02,  2.080000000000000e+00,
            -1.000000000000000e+00, -1.000000000000000e+00,  2.023681082934419e+00,
            -8.000000000000001e-05, -8.000000000000001e-05, -7.035210829344193e-01,
            -1.000000000000000e+00, -8.000000000000001e-05,  2.023681082934419e+00,
            -8.000000000000001e-05, -7.035210829344193e-01, -1.000000000000000e+00,
            -8.000000000000001e-05,  2.023681082934419e+00, -8.000000000000001e-05,
            -7.035210829344193e-01, -1.000000000000000e+00, -8.000000000000001e-05,
            -8.000000000000001e-05,  2.023681082934419e+00, -7.035210829344193e-01,
            -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
            -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
            -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
            -7.035210829344193e-01,  8.006550829678521e+02, -1.201561884917640e+00,
            -1.201561884917640e+00,  2.206800548917464e+04, -1.201561884917640e+00,
             2.206800548917464e+04, -1.201561884917640e+00,  2.206800548917464e+04,
            -1.201561884917640e+00,  2.206800548917464e+04,
        ], 1e-10);
        assert_eq!(&conductance.indices[..], &vec![
            0, 1, 2, 4, 0, 1, 3, 5, 0, 2, 3, 6, 1, 2, 3, 7, 0, 4, 5, 6, 8, 1, 4, 5, 7, 9, 2, 4, 6,
            7, 10, 3, 5, 6, 7, 11, 4, 8, 12, 5, 9, 13, 6, 10, 14, 7, 11, 15, 8, 12, 9, 13, 10, 14,
            11, 15,
        ][..]);
        assert_eq!(&conductance.offsets[..], &vec![
            0, 4, 8, 12, 16, 21, 26, 31, 36, 39, 42, 45, 48, 50, 52, 54, 56,
        ][..]);
    });
}

#[test]
fn system_distribution() {
    use matrix::{Conventional, Size};

    setup(Some("double"), |path| {
        let system = ok!(System::new(path));
        let distribution = ok!(system.distribution());

        assert_eq!(distribution.rows(), 4 * 4 * 4);
        assert_eq!(distribution.columns(), 4);

        let distribution: Vec<_> = Conventional::from(&distribution).into();
        assert_eq!(&distribution, &vec![
            0.25, 0.25, 0.0, 0.0, 0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,

            0.0, 0.0, 0.25, 0.25, 0.0, 0.0, 0.25, 0.25, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,

            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.25, 0.25, 0.0, 0.0, 0.25, 0.25, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0,

            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.25, 0.25, 0.0, 0.0, 0.25, 0.25,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00,
            0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.00, 0.00, 0.0, 0.0, 0.00, 0.00,
        ]);
    });
}

#[test]
fn system_stack() {
    setup(None, |path| {
        let system = ok!(System::new(path));
        let stack = &system.stack;

        let dimensions = &stack.dimensions;
        assert_eq!(dimensions.layers(), 4);
        assert_eq!(dimensions.rows(), 2);
        assert_eq!(dimensions.columns(), 2);
        assert_eq!(dimensions.connections(), 56);

        let elements = &stack.elements;
        assert_eq!(elements.len(), 2);
        let die = match (&elements[0], &elements[1]) {
            (&StackElement::HeatSink, &StackElement::Die(ref die)) => die,
            _ => unreachable!(),
        };
        assert_eq!(&die.id, "DIE");

        let floorplan = &die.floorplan;
        assert_eq!(floorplan.elements.iter().map(|element| &element.id).collect::<Vec<_>>(),
                   &["Core0", "Core1", "Core2", "Core3"]);
    });
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
    match fixture::find::first_with_extension(&path, "stk") {
        Some(path) => path,
        None => panic!("cannot find a stack description in {:?}", path),
    }
}
