//! The **cgs** module provides a unit system for use with Gaussian CGS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!

#![allow(missing_docs)]

make_units! {
    CGS;
    ONE: Unitless;
    base {
        P2, CM: Centimeter, "cm";
        P2, G: Gram, "g";
        P1, S: Second, "s";
    }
    derived {
        CM2: Centimeter2 = (Centimeter * Centimeter);
        CM3: Centimeter3 = (Centimeter2 * Centimeter);

        S2: Second2 = (Second * Second);
        S3: Second3 = (Second2 * Second);
        S4: Second4 = (Second3 * Second);

        CMPS: CentimeterPerSecond = (Centimeter / Second);
        CMPS2: CentimeterPerSecond2 = (Centimeter / Second2);
        CMPS3: CentimeterPerSecond3 = (Centimeter / Second3);
        CMPS4: CentimeterPerSecond4 = (Centimeter / Second4);

        CM2PS: Centimeter2PerSecond = (Centimeter2 / Second);
        CM2PS2: Centimeter2PerSecond2 = (Centimeter2 / Second2);
        CM2PS3: Centimeter2PerSecond3 = (Centimeter2 / Second3);

        CM3PS: Centimeter3PerSecond = (Centimeter3 / Second);
        CM3PS2: Centimeter3PerSecond2 = (Centimeter3 / Second2);
        CM3PS3: Centimeter3PerSecond3 = (Centimeter3 / Second3);

        GAL: Gal = (Centimeter / Second2);
        DYN: Dyne = (Gram *  Gal);
        ERG: Erg = (Dyne * Centimeter);
        ERGPS: ErgPerSecond = (Erg / Second);
        BA: Barye = (Gram / Centimeter / Second2);
        P: Poise = (Gram / Centimeter / Second);
        ST: Stokes = (Centimeter2 / Second);
        K: Kayser = (Unitless / Centimeter);

        // SQRT_CM: SqrtCentimeter = (<Centimeter as Sqrt>::Output);
        // SQRT_G: SqrtGram = (<Gram as Sqrt>::Output);

        // STATC: StatCoulomb = (SqrtGram * SqrtCentimeter * Centimeter / Second);
        // STATA: StatAmpere = (StatCoulomb / Second);
        // STATV: StatVolt = (Erg / StatCoulomb);
    }
    constants {}
    fmt = true;
}

pub use self::f64consts::*;


use typenum::{Integer};
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
        let kgfac = 1000.0f64.powf(Meter::to_i32() as f64 / 2.0);
        // Factor due to seconds are always 1!
        // let sfac = 1.0f64.powi(Meter::to_i32());
        let fac = mfac * kgfac;

        CGS::new( other.value_unsafe * fac )
    }
}

#[test]
fn test_convert() {
    use mks;
    use cgs::f64consts::*;
    let force_mks = 10.0 * mks::KG * mks::M / mks::S / mks::S;
    let force_cgs = 1_000_000.0 * G*CM/S/S;
    assert_eq!(CGS::from(force_mks), force_cgs);
}

// // We're converting from a system with 7 base units to one with 3, so things are gonna get weird
// // Note that we can't convert from cgs to si as we can't be sure where any of the cgs units came
// // from.
// use si;
// impl<V, Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole> From
//     <si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>>
//     for CGS<Prod<V, f64>, tarr![Prod<Meter, P2>, Prod<Kilogram, P2>, Second][> where
//     Meter: Integer, Kilogram: Integer, Second: Integer,
//     V: Mul<f64>,
// {
//     fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>>) -> Self {
//         let mfac = 100.0f64.powf(Meter::to_i32() as f64 / 2.0);
//         let kgfac = 1000.0f64.powf(Meter::to_i32() as f64/ 2.0);
//         // Factor due to seconds are always 1!
//         // let sfac = 1.0f64.powi(Meter::to_i32());
//         let fac = mfac * kgfac;

//         CGS::new( other.value * fac )
//     }
// }
