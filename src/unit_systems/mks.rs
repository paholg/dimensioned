//! The **mks** module provides a unit system for use with Gaussian MKS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!
//! It will also define derived units, although this is not implemented yet.
//!

#![allow(missing_docs)]

make_units_adv! {
    MKS, Unitless;
    base {
        P2, Meter, m;
        P2, Kilogram, kg;
        P1, Second, s;
    }
    derived {
        MeterPerSecond = (Meter / Second);
    }
}

pub trait FromMKS<Meter: Integer, Kilogram: Integer, Second: Integer, V>
    where Self: Sized
{
    fn from_mks(from: Dim<MKS<Meter, Kilogram, Second>, V>) -> Dim<Self, V>;
}

#[allow(unused_imports)]
// needed for some reason
#[cfg(not(feature="std"))]
use core::num::Float;

#[allow(unused_imports)]
// needed for some reason
#[cfg(not(feature="std"))]
use dim::Sqrt;

use cgs::{CGS, FromCGS};
impl<Centimeter, Gram, Second, V> FromCGS<Centimeter, Gram, Second, V>
    for MKS<Centimeter, Gram, Second>
    where V: Mul<f64, Output = V>,
          Centimeter: Integer,
          Gram: Integer,
          Second: Integer
{
    fn from_cgs(from: Dim<CGS<Centimeter, Gram, Second>, V>) -> Dim<Self, V> {
        Dim::new(from.0 * 0.01f64.sqrt().powi(Centimeter::to_i32()) *
                 0.001f64.sqrt().powi(Gram::to_i32()))
    }
}

// #[test]
// fn mps_test() {
//     let dist = 2.0 * m;
//     let time = 2.0 * s;
//     let v = MeterPerSecond::new(1.0);

//     assert_eq!(v, dist / time);
// }
