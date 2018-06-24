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
    use si::SI;
    use typenum::{Integer, Sum, Prod, Z0};
    use core::convert::From;
    use core::ops::{Mul, Add};
    use ucum;
    use f64prefixes::*;

    impl<V, Meter, Second, Gram, Kelvin, Coulomb, Candela> From<
            ucum::UCUM<V, tarr![Meter, Second, Gram, Z0, Kelvin, Coulomb, Candela]>>
        for SI<Prod<V, f64>, tarr![Meter, Gram, Sum<Second, Coulomb>, Coulomb, Kelvin, Candela, Z0]> where
        Meter: Integer, Second: Integer + Add<Coulomb>, Gram: Integer, Kelvin: Integer, Coulomb: Integer, Candela: Integer,
        V: Mul<f64>,
    {
        fn from(other: ucum::UCUM<V, tarr![Meter, Second, Gram, Z0, Kelvin, Coulomb, Candela]>) -> Self {
            use core::intrinsics::powif64;
            let gfac = unsafe { powif64(MILLI, Gram::to_i32()) };

            let fac = gfac;

            SI::new( other.value_unsafe * fac )
        }
    }
}

mod to_ucum {
    // From SI
    use ucum::UCUM;
    use typenum::{Integer, Diff, Prod, Z0};
    use core::convert::From;
    use core::ops::{Mul, Sub};
    use si;
    use f64prefixes::*;

    impl<V, Meter, Kilogram, Second, Ampere, Kelvin, Candela> From<
            si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Z0]>>
        for UCUM<Prod<V, f64>, tarr![Meter, Diff<Second, Ampere>, Kilogram, Z0, Kelvin, Ampere, Candela]> where
        Meter: Integer, Kilogram: Integer, Second: Integer + Sub<Ampere>, Ampere: Integer, Kelvin: Integer, Candela: Integer,
        V: Mul<f64>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Z0]>) -> Self {
            use core::intrinsics::powif64;
            let kgfac = unsafe { powif64(KILO, Kilogram::to_i32()) };

            let fac = kgfac;

            UCUM::new( other.value_unsafe * fac )
        }
    }
}

mod to_cgs {
    use cgs::CGS;
    // From MKS
    use typenum::{Integer, Prod, Sum};
    use core::convert::From;
    use core::ops::{Mul, Add};
    use mks;
    use f64prefixes::*;
    impl<V, SqrtMeter, SqrtKilogram, Second> From<mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>>
        for CGS<Prod<V, f64>, tarr![SqrtMeter, SqrtKilogram, Second]> where
        SqrtMeter: Integer, SqrtKilogram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>) -> Self {
            use core::intrinsics::{powif64, sqrtf64};
            let mfac = match SqrtMeter::to_i32() {
                e if e % 2 == 0 => unsafe { powif64(HECTO, e / 2) },
                e => unsafe { powif64(sqrtf64(HECTO), e) },
            };
            let kgfac = match SqrtKilogram::to_i32() {
                e if e % 2 == 0 => unsafe { powif64(KILO, e / 2) },
                e => unsafe { powif64(sqrtf64(KILO), e) },
            };

            let fac = mfac * kgfac;

            CGS::new( other.value_unsafe * fac )
        }
    }

    // From SI
    use si;
    use typenum::{Z0, P2, P3};
    impl<V, Meter, Kilogram, Second, Ampere> From<
            si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>>
        for CGS<Prod<Prod<V, f64>, f64>, tarr![
            Sum<Prod<Meter, P2>, Prod<Ampere, P3>>,
            Sum<Prod<Kilogram, P2>, Ampere>,
            Sum<Second, Prod<Ampere, P2>>
        ]> where V: Mul<f64>,
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
    use mks::MKS;
    use typenum::{Integer, Prod, Sum};
    use core::convert::From;
    use core::ops::{Mul, Add};
    use f64prefixes::*;
    // From CGS
    use cgs;
    impl<V, SqrtCentimeter, SqrtGram, Second> From<cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>>
        for MKS<Prod<V, f64>, tarr![SqrtCentimeter, SqrtGram, Second]> where
        SqrtCentimeter: Integer, SqrtGram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>) -> Self {
            use core::intrinsics::{powif64, sqrtf64};
            let cmfac = match SqrtCentimeter::to_i32() {
                e if e % 2 == 0 => unsafe { powif64(CENTI, e / 2) },
                e => unsafe { powif64(sqrtf64(CENTI), e) },
            };
            let gfac = match SqrtGram::to_i32() {
                e if e % 2 == 0 => unsafe { powif64(MILLI, e / 2) },
                e => unsafe { powif64(sqrtf64(MILLI), e) },
            };

            let fac = cmfac * gfac;

            MKS::new( other.value_unsafe * fac )
        }
    }

    // From SI
    use si;
    use typenum::{Z0, P2, P3};
    impl<V, Meter, Kilogram, Second, Ampere> From<
            si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>>
        for MKS<Prod<V, f64>, tarr![
            Sum<Prod<Meter, P2>, Prod<Ampere, P3>>,
            Sum<Prod<Kilogram, P2>, Ampere>,
            Sum<Second, Prod<Ampere, P2>>
        ]> where V: Mul<f64>,
                 Meter: Integer + Mul<P2>,
                 Kilogram: Integer + Mul<P2>,
                 Second: Integer + Add<Prod<Ampere, P2>>,
                 Ampere: Integer + Mul<P2> + Mul<P3>,
                 Prod<Meter, P2>: Add<Prod<Ampere, P3>>,
                 Prod<Kilogram, P2>: Add<Ampere>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Z0, Z0, Z0]>) -> Self {
            MKS::new( other.value_unsafe * 1.0 )
        }
    }
}
