//! The **cgs** module provides a unit system for use with Gaussian CGS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!

#![allow(missing_docs)]

make_units! {
    CGS, Unitless;
    base {
        P2, Centimeter, cm;
        P2, Gram, g;
        P1, Second, s;
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

        SqrtCentimeter = (<Centimeter as Sqrt>::Output);
        SqrtGram = (<Gram as Sqrt>::Output);

        StatCoulomb = (SqrtGram * SqrtCentimeter * Centimeter / Second);
        StatAmpere = (StatCoulomb / Second);
        StatVolt = (Erg / StatCoulomb);
    }
    fmt = true;
}

#[allow(non_upper_case_globals)]
mod consts {
    use reexported::marker::PhantomData;
    use super::*;
    pub const one: Unitless<f64> = CGS { value: 1.0, _marker: PhantomData };
    pub const cm: Centimeter<f64> = CGS { value: 1.0, _marker: PhantomData };
    pub const g: Gram<f64> = CGS { value: 1.0, _marker: PhantomData };
    pub const s: Second<f64> = CGS { value: 1.0, _marker: PhantomData };
}

pub use self::consts::*;

use typenum::{Prod, Integer};
use reexported::convert::From;
use mks;
impl<V, Meter, Kilogram, Second> From<mks::MKS<V, tarr![Meter, Kilogram, Second]>>
    for CGS<Prod<V, f64>, tarr![Meter, Kilogram, Second]> where
    Meter: Integer, Kilogram: Integer, Second: Integer,
    V: Mul<f64>,
{
    fn from(other: mks::MKS<V, tarr![Meter, Kilogram, Second]>) -> Self {
        // Note we have to be a bit careful here because these unit systems are special and use
        // double the regular unit power, so that they can be represented with half integer
        // powers. E.g. The unit for area will really be `m^4`.
        let mfac = 100.0f64.powf(Meter::to_i32() as f64 / 2.0);
        let kgfac = 1000.0f64.powf(Meter::to_i32() as f64/ 2.0);
        // Factor due to seconds is always 1!
        // let sfac = 1.0f64.powi(Meter::to_i32());
        let fac = mfac * kgfac;

        CGS::new( other.value * fac )
    }
}

#[test]
fn test_convert() {
    let force_mks = mks::Unitless::new(10.0) * mks::kg * mks::m / mks::s / mks::s;
    let force_cgs = Unitless::new(1_000_000.0) * g * cm / s / s;
    assert_eq!(CGS::from(force), force2);
}
