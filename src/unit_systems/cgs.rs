//! The **cgs** module provides a unit system for use with Gaussian CGS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!
//! It will also define derived units, although this is not implemented yet.
//!

#![allow(missing_docs)]

make_units_adv! {
    CGS, Unitless, one, f64, 1.0;
    base {
        P2, Centimeter, cm, cm;
        P2, Gram, g, g;
        P1, Second, s, s;
    }
    derived {
        Centimeter2 = (Centimeter * Centimeter);
        Centimeter3 = (Centimeter2 * Centimeter);

        Second2 = (Second * Second);
        Second3 = (Second2 * Second);
        Second4 = (Second3 * Second);

        CentimeterPerSecond = (Centimeter / Second);
        CentimeterPerSecond2 = (Centimeter / Second2);
        CentimeterPerSecond3 = (Centimeter / Second3);
        CentimeterPerSecond4 = (Centimeter / Second4);

        Centimeter2PerSecond = (Centimeter2 / Second);
        Centimeter2PerSecond2 = (Centimeter2 / Second2);
        Centimeter2PerSecond3 = (Centimeter2 / Second3);

        Centimeter3PerSecond = (Centimeter3 / Second);
        Centimeter3PerSecond2 = (Centimeter3 / Second2);
        Centimeter3PerSecond3 = (Centimeter3 / Second3);

        Gal = (Centimeter / Second2);
        Dyne = (Gram *  Gal);
        Erg = (Dyne * Centimeter);
        ErgPerSecond = (Erg / Second);
        Barye = (Gram / Centimeter / Second2);
        Poise = (Gram / Centimeter / Second);
        Stokes = (Centimeter2 / Second);
        Kayser = (Unitless / Centimeter);

        SqrtCentimeter = (<Centimeter as Root<P2>>::Output);
        SqrtGram = (<Gram as Root<P2>>::Output);

        StatCoulomb = (SqrtGram * SqrtCentimeter * Centimeter / Second);
        StatAmpere = (StatCoulomb / Second);
        StatVolt = (Erg / StatCoulomb);
    }
}

pub trait FromCGS<Centimeter: Integer, Gram: Integer, Second: Integer, V>
    where Self: Sized
{
    fn from_cgs(from: Dim<CGS<Centimeter, Gram, Second>, V>) -> Dim<Self, V>;
}

#[allow(unused_imports)]
// needed for some reason
#[cfg(not(feature="std"))]
use core::num::Float;

#[allow(unused_imports)]
// needed for some reason
#[cfg(not(feature="std"))]
use dim::Sqrt;

use mks::{MKS, FromMKS};
impl<Meter, Kilogram, Second, V> FromMKS<Meter, Kilogram, Second, V>
    for CGS<Meter, Kilogram, Second>
    where V: Mul<f64, Output = V>,
          Meter: Integer,
          Kilogram: Integer,
          Second: Integer
{
    fn from_mks(from: Dim<MKS<Meter, Kilogram, Second>, V>) -> Dim<Self, V> {
        Dim::new(from.0 * 100f64.sqrt().powi(Meter::to_i32()) *
                 1000f64.sqrt().powi(Kilogram::to_i32()))
    }
}


#[test]
fn gal_test() {
    let x = 3.0 * cm;
    let t = 2.0 * s;
    // fixme
    // let a = 4.5 * gal;
    // assert_eq!(a, x * x / t);
}
