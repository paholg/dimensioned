mod to_si {
    // From UCUM
    use si::SI;
    use typenum::{Integer, Sum, Prod, Z0};
    use core::convert::From;
    use core::ops::{Mul, Add};
    use ucum;
    use f64prefixes::*;

    impl<V, Meter, Second, Gram, Radian, Kelvin, Coulomb, Candela> From<
            ucum::UCUM<V, tarr![Meter, Second, Gram, Radian, Kelvin, Coulomb, Candela]>>
        for SI<Prod<V, f64>, tarr![Meter, Gram, Sum<Second, Coulomb>, Coulomb, Kelvin, Candela, Z0]> where
        Meter: Integer, Second: Integer + Add<Coulomb>, Gram: Integer, Radian: Integer, Kelvin: Integer, Coulomb: Integer, Candela: Integer,
        V: Mul<f64>,
    {
        fn from(other: ucum::UCUM<V, tarr![Meter, Second, Gram, Radian, Kelvin, Coulomb, Candela]>) -> Self {
            let gfac = MILLI.powi(Gram::to_i32());

            let fac = gfac;

            SI::new( other.value_unsafe * fac )
        }
    }
}

mod to_ucum {
    // From SI
    use ucum::{self, UCUM};
    use typenum::{Integer, Diff, Prod, Z0};
    use core::convert::From;
    use core::ops::{Mul, Sub};
    use si;
    use f64prefixes::*;

    impl<V, Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole> From<
            si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>>
        for UCUM<Prod<V, f64>, tarr![Meter, Diff<Second, Ampere>, Kilogram, Z0, Kelvin, Ampere, Candela]> where
        Meter: Integer, Kilogram: Integer, Second: Integer + Sub<Ampere>, Ampere: Integer, Kelvin: Integer, Candela: Integer, Mole: Integer,
        V: Mul<f64>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>) -> Self {
            let kgfac = KILO.powi(Kilogram::to_i32());
            let molfac = (ucum::MOL.value_unsafe).powi(Mole::to_i32());

            let fac = kgfac * molfac;

            UCUM::new( other.value_unsafe * fac )
        }
    }
}

mod to_cgs {
    use cgs::CGS;
    // From MKS
    use typenum::{Integer, Prod};
    use core::convert::From;
    use core::ops::{Mul};
    use mks;
    use f64prefixes::*;
    impl<V, SqrtMeter, SqrtKilogram, Second> From<mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>>
        for CGS<Prod<V, f64>, tarr![SqrtMeter, SqrtKilogram, Second]> where
        SqrtMeter: Integer, SqrtKilogram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: mks::MKS<V, tarr![SqrtMeter, SqrtKilogram, Second]>) -> Self {
            let mfac = HECTO.sqrt().powi(SqrtMeter::to_i32());
            let kgfac = match SqrtKilogram::to_i32() {
                e if e % 2 == 0 => KILO.powi(e / 2),
                e => KILO.sqrt().powi(e),
            };

            let fac = mfac * kgfac;

            CGS::new( other.value_unsafe * fac )
        }
    }
}

mod to_mks {
    use mks::MKS;
    // From CGS
    use typenum::{Integer, Prod};
    use core::convert::From;
    use core::ops::{Mul};
    use cgs;
    use f64prefixes::*;
    impl<V, SqrtCentimeter, SqrtGram, Second> From<cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>>
        for MKS<Prod<V, f64>, tarr![SqrtCentimeter, SqrtGram, Second]> where
        SqrtCentimeter: Integer, SqrtGram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: cgs::CGS<V, tarr![SqrtCentimeter, SqrtGram, Second]>) -> Self {
            let cmfac = CENTI.sqrt().powi(SqrtCentimeter::to_i32());
            let gfac = match SqrtGram::to_i32() {
                e if e % 2 == 0 => MILLI.powi(e / 2),
                e => MILLI.sqrt().powi(e),
            };

            let fac = cmfac * gfac;

            MKS::new( other.value_unsafe * fac )
        }
    }
}
