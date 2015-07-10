extern crate assert;
extern crate fixture;
extern crate matrix;
extern crate temporary;
extern crate threed_ice;

use matrix::CompressedFormat;
use std::path::{Path, PathBuf};
use temporary::Directory;

use threed_ice::{Circuit, Stack, StackElement};

macro_rules! ok(
    ($result:expr) => ($result.unwrap());
);

#[test]
fn circuit_new() {
    setup(None, |stack| {
        let circuit = ok!(Circuit::new(stack));

        assert_eq!(circuit.layers, 4);
        assert_eq!(circuit.rows, 2);
        assert_eq!(circuit.columns, 2);
        assert_eq!(circuit.cells, 4 * 2 * 2);

        assert::close(&circuit.capacitance, &vec![
            1.05000e-03, 1.05000e-03, 1.05000e-03, 1.05000e-03, 3.20000e-04, 3.20000e-04,
            3.20000e-04, 3.20000e-04, 7.98750e-01, 7.98750e-01, 7.98750e-01, 7.98750e-01,
            2.20455e+01, 2.20455e+01, 2.20455e+01, 2.20455e+01,
        ], 1e-15);

        let conductance = &circuit.conductance;
        assert_eq!(conductance.rows, 4 * 2 * 2);
        assert_eq!(conductance.columns, 4 * 2 * 2);
        assert_eq!(conductance.nonzeros, 56);
        assert_eq!(conductance.format, CompressedFormat::Column);
        assert::close(&conductance.data, &vec![
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
fn stack_new() {
    setup(None, |stack| {
        let stack = ok!(Stack::new(stack));
        assert_eq!(stack.elements.len(), 2);

        let die = match (&stack.elements[0], &stack.elements[1]) {
            (&StackElement::HeatSink, &StackElement::Die(ref die)) => die,
            _ => unreachable!(),
        };
        assert_eq!(&die.name, "DIE");

        let floorplan = &die.floorplan;
        assert_eq!(floorplan.elements.iter().map(|element| &element.name).collect::<Vec<_>>(),
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
