extern crate dimensioned as dim;

use crate::dim::si;

// Just a couple simple tests to ensure that we're creating int consts.

#[test]
fn int_consts() {
    use crate::si::Meter;

    assert_eq!(Meter::new(1), si::i8consts::M);
    assert_eq!(Meter::new(1), si::i16consts::M);
    assert_eq!(Meter::new(1), si::i32consts::M);
    assert_eq!(Meter::new(1), si::i64consts::M);
    assert_eq!(Meter::new(1), si::isize_consts::M);

    assert_eq!(Meter::new(1), si::u8consts::M);
    assert_eq!(Meter::new(1), si::u16consts::M);
    assert_eq!(Meter::new(1), si::u32consts::M);
    assert_eq!(Meter::new(1), si::u64consts::M);
    assert_eq!(Meter::new(1), si::usize_consts::M);
}

#[test]
fn derived_int_consts() {
    use crate::si::Newton;

    assert_eq!(Newton::new(1), si::i8consts::N);
    assert_eq!(Newton::new(1), si::i16consts::N);
    assert_eq!(Newton::new(1), si::i32consts::N);
    assert_eq!(Newton::new(1), si::i64consts::N);
    assert_eq!(Newton::new(1), si::isize_consts::N);

    assert_eq!(Newton::new(1), si::u8consts::N);
    assert_eq!(Newton::new(1), si::u16consts::N);
    assert_eq!(Newton::new(1), si::u32consts::N);
    assert_eq!(Newton::new(1), si::u64consts::N);
    assert_eq!(Newton::new(1), si::usize_consts::N);
}
