//! Conversion between unit systems
//!
//! Wherever it makes sense, we implement conversion between the unit systems that ship with
//! dimensioned using `core::convert::From`.
//!
//! Note that it does not always make sense to do so. For example, while one can convert from a
//! subset of the `SI` system to `CGS`, it makes no sense to convert from `CGS` to `SI`.
//!
//! If are interested in implementing conversion for your own unit system, [here is an
//! example](https://github.com/paholg/dimensioned-examples/blob/master/src/conversion.md)
//! demonstrating how to do so.
//!
//! Conversions between unit systems are implemented as follows:
//!
//! * `SI` to `UCUM`: As `UCUM` does not have a unit for amount of substance, this is defined only
//! for `SI` units that don't contain `Mole`s. In addition, as `UCUM` defines radians as a unit, it
//! is not correct to perform this conversion if the quantity expresses an angle.
//!
//! * `SI` to `CGS` and `MKS`: These conversions are only defined for `SI` units that are a
//! combination of `Meter`, `Kilogram`, `Second`, and `Ampere`.
//!
//! * `UCUM` to `SI`: As `SI` does not have a unit for angle, this is only defined for `UCUM`
//! units that don't contain `Radian`s.
//!
//! * `CGS` to `MKS`
//! * `MKS` to `CGS`

mod to_si {
    // From UCUM
    use core::convert::From;
    use core::ops::{Add, Mul};
    use f64prefixes::*;
    use si::SI;
    use typenum::{Integer, Prod, Sum, Z0};
    use ucum;

    impl<V, Meter, Second, Gram, Kelvin, Coulomb, Candela>
        From<ucum::UCUM<V, tarr![Meter, Second, Gram, Z0, Kelvin, Coulomb, Candela]>>
        for SI<Prod<V, f64>, tarr![Meter, Gram, Sum<Second, Coulomb>, Coulomb, Kelvin, Candela, Z0]>
    where
        Meter: Integer,
        Second: Integer + Add<Coulomb>,
        Gram: Integer,
        Kelvin: Integer,
        Coulomb: Integer,
        Candela: Integer,
        V: Mul<f64>,
    {
        fn from(
            other: ucum::UCUM<V, tarr![Meter, Second, Gram, Z0, Kelvin, Coulomb, Candela]>,
        ) -> Self {
            let gfac = MILLI.powi(Gram::to_i32());

            let fac = gfac;

            SI::new(other.value_unsafe * fac)
        }
    }
}

mod to_ucum {
    // From SI
    use core::convert::From;
    use core::ops::{Mul, Sub};
    use f64prefixes::*;
    use si;
    use typenum::{Diff, Integer, Prod, Z0};
    use ucum::UCUM;

    impl<V, Meter, Kilogram, Second, Ampere, Kelvin, Candela>
        From<si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Z0]>>
        for UCUM<
            Prod<V, f64>,
            tarr![Meter, Diff<Second, Ampere>, Kilogram, Z0, Kelvin, Ampere, Candela],
        > where
        Meter: Integer,
        Kilogram: Integer,
        Second: Integer + Sub<Ampere>,
        Ampere: Integer,
        Kelvin: Integer,
        Candela: Integer,
        V: Mul<f64>,
    {
        fn from(
            other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Z0]>,
        ) -> Self {
            let kgfac = KILO.powi(Kilogram::to_i32());

            let fac = kgfac;

            UCUM::new(other.value_unsafe * fac)
        }
    }
}

mod to_cgs {
    use cgs::CGS;
    // From MKS
    use core::convert::From;
    use core::ops::{Add, Mul};
    use f64prefixes::*;
    use mks;
    use typenum::{Integer, Prod, Sum};
    impl<V, SqrtMeter, SqrtKilogram, Second>
        From<mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>>
        for CGS<Prod<V, f64>, tarr![SqrtMeter, SqrtKilogram, Second]>
    where
        SqrtMeter: Integer,
        SqrtKilogram: Integer,
        Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>) -> Self {
            let mfac = match SqrtMeter::to_i32() {
                e if e % 2 == 0 => HECTO.powi(e / 2),
                e => HECTO.sqrt().powi(e),
            };
            let kgfac = match SqrtKilogram::to_i32() {
                e if e % 2 == 0 => KILO.powi(e / 2),
                e => KILO.sqrt().powi(e),
            };

            let fac = mfac * kgfac;

            CGS::new(other.value_unsafe * fac)
        }
    }

    // From SI
    use si;
    use typenum::{P2, P3, Z0};
    impl<V, Meter, Kilogram, Second, Ampere>
        From<si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>>
        for CGS<
            Prod<Prod<V, f64>, f64>,
            tarr![
            Sum<Prod<Meter, P2>, Prod<Ampere, P3>>,
            Sum<Prod<Kilogram, P2>, Ampere>,
            Sum<Second, Prod<Ampere, P2>>
        ],
        > where
        V: Mul<f64>,
        Meter: Integer + Mul<P2>,
        Kilogram: Integer + Mul<P2>,
        Second: Integer + Add<Prod<Ampere, P2>>,
        Ampere: Integer + Mul<P2> + Mul<P3>,
        Prod<Meter, P2>: Add<Prod<Ampere, P3>>,
        Prod<Kilogram, P2>: Add<Ampere>,
        Sum<Prod<Meter, P2>, Prod<Ampere, P3>>: Integer,
        Sum<Prod<Kilogram, P2>, Ampere>: Integer,
        Sum<Second, Prod<Ampere, P2>>: Integer,
        Prod<V, f64>: Mul<f64>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>) -> Self {
            CGS::from(mks::MKS::from(other))
        }
    }
}

mod to_mks {
    use core::convert::From;
    use core::ops::{Add, Mul};
    use f64prefixes::*;
    use mks::MKS;
    use typenum::{Integer, Prod, Sum};
    // From CGS
    use cgs;
    impl<V, SqrtCentimeter, SqrtGram, Second>
        From<cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>>
        for MKS<Prod<V, f64>, tarr![SqrtCentimeter, SqrtGram, Second]>
    where
        SqrtCentimeter: Integer,
        SqrtGram: Integer,
        Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>) -> Self {
            let cmfac = match SqrtCentimeter::to_i32() {
                e if e % 2 == 0 => CENTI.powi(e / 2),
                e => CENTI.sqrt().powi(e),
            };
            let gfac = match SqrtGram::to_i32() {
                e if e % 2 == 0 => MILLI.powi(e / 2),
                e => MILLI.sqrt().powi(e),
            };

            let fac = cmfac * gfac;

            MKS::new(other.value_unsafe * fac)
        }
    }

    // From SI
    use si;
    use typenum::{P2, P3, Z0};
    impl<V, Meter, Kilogram, Second, Ampere>
        From<si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>>
        for MKS<
            Prod<V, f64>,
            tarr![
            Sum<Prod<Meter, P2>, Prod<Ampere, P3>>,
            Sum<Prod<Kilogram, P2>, Ampere>,
            Sum<Second, Prod<Ampere, P2>>
        ],
        > where
        V: Mul<f64>,
        Meter: Integer + Mul<P2>,
        Kilogram: Integer + Mul<P2>,
        Second: Integer + Add<Prod<Ampere, P2>>,
        Ampere: Integer + Mul<P2> + Mul<P3>,
        Prod<Meter, P2>: Add<Prod<Ampere, P3>>,
        Prod<Kilogram, P2>: Add<Ampere>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>) -> Self {
            MKS::new(other.value_unsafe * 1.0)
        }
    }
}
