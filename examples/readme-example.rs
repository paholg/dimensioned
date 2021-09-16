#![allow(unused_imports)]

extern crate dimensioned as dim;

use std::fmt::Debug;

use crate::dim::{cgs, si};

// Calculates speed given a distance and time. Only works for SI units.
fn speed(dist: si::Meter<f64>, time: si::Second<f64>) -> si::MeterPerSecond<f64> {
    dist / time
}

use crate::dim::dimensions::{Length, Time};
use crate::dim::typenum::Quot;
use std::ops::Div;

// Calculates speed as before, but now we can use *any* unit system.
fn generic_speed<L, T>(dist: L, time: T) -> Quot<L, T>
where
    L: Length + Div<T>,
    T: Time,
{
    dist / time
}

// Conversion between unit systems is not yet available on stable 'no_std':
#[cfg(any(feature = "std", feature = "nightly"))]
fn test_conversion<V1, V2>(v1: V1, v2: V2)
where
    V1: From<V2> + PartialEq + Debug,
{
    let v3 = v2.into();
    assert_eq!(v1, v3);
}

// Dummy to skip on `no_std` without `nightly`:
#[cfg(not(any(feature = "std", feature = "nightly")))]
fn test_conversion<V1, V2>(_v1: V1, _v2: V2) {}

fn main() {
    let si_x = 6.0 * si::M;
    let si_t = 3.0 * si::S;
    let si_v = 2.0 * si::M / si::S;

    let si_v2 = speed(si_x, si_t);
    assert_eq!(si_v, si_v2);

    let cgs_x = 6.0 * cgs::M;
    let cgs_t = 3.0 * cgs::S;
    let cgs_v = 2.0 * cgs::M / cgs::S;

    let cgs_v2 = generic_speed(cgs_x, cgs_t);
    assert_eq!(cgs_v, cgs_v2);

    test_conversion(si_v2, si_v2);
}
