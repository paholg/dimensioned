#![cfg(feature = "std")]

extern crate dimensioned as dim;

use crate::dim::si::{self, f64consts::*};

#[test]
fn formatting() {
    assert_eq!(<si::Unitless<f32>>::to_string(), "");
    assert_eq!(format!("{}", 3.5 * ONE), "3.5");

    assert_eq!(<si::Second<f32>>::to_string(), "s");
    assert_eq!(format!("{}", 3.5 * S), "3.5 s");

    assert_eq!(<si::Newton<f32>>::to_string(), "m*kg*s^-2");
    assert_eq!(format!("{}", 3.5 * N), "3.5 m*kg*s^-2");
}
