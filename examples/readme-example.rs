extern crate dimensioned as dim;

use dim::{cgs, si};

// Calculates speed given a distance and time. Only works for SI units.
fn speed(dist: si::Meter<f64>, time: si::Second<f64>) -> si::MeterPerSecond<f64> {
    dist / time
}

use dim::dimensions::{Length, Time};
use dim::typenum::Quot;
use std::ops::Div;

// Calculates speed as before, but now we can use *any* unit system.
fn generic_speed<L, T>(dist: L, time: T) -> Quot<L, T>
where
    L: Length + Div<T>,
    T: Time,
{
    dist / time
}

fn main() {
    let x = 6.0 * si::M;
    let t = 3.0 * si::S;
    let v = 2.0 * si::M / si::S;
    let v2 = speed(x, t);
    assert_eq!(v, v2);

    let v3 = generic_speed(6.0 * cgs::M, 3.0 * cgs::S);
    let v4 = v.into();
    assert_eq!(v3, v4);
}
