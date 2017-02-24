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
    impl<V, Meter, Kilogram, Second> From<mks::MKS<V, tarr![Meter, Kilogram, Second]>>
        for CGS<Prod<V, f64>, tarr![Meter, Kilogram, Second]> where
        Meter: Integer, Kilogram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: mks::MKS<V, tarr![Meter, Kilogram, Second]>) -> Self {
            // Note we have to be a bit careful here because these unit systems are special and use
            // double the regular unit power, so that they can be represented with half integer
            // powers. E.g. The unit for area will really be `m^4`. That is why we take a sqrt first.
            let mfac = 100.0f64.sqrt().powi(Meter::to_i32());
            let kgfac = 1000.0f64.sqrt().powi(Kilogram::to_i32());

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
    impl<V, Centimeter, Gram, Second> From<cgs::CGS<V, tarr![Centimeter, Gram, Second]>>
        for MKS<Prod<V, f64>, tarr![Centimeter, Gram, Second]> where
        Centimeter: Integer, Gram: Integer, Second: Integer,
        V: Mul<f64>,
    {
        fn from(other: cgs::CGS<V, tarr![Centimeter, Gram, Second]>) -> Self {
            // Note we have to be a bit careful here because these unit systems are special and use
            // double the regular unit power, so that they can be represented with half integer
            // powers. E.g. The unit for area will really be `cm^4`. That is why we take a sqrt first.
            let cmfac = 0.01f64.sqrt().powi(Centimeter::to_i32());
            let gfac = 0.001f64.sqrt().powi(Gram::to_i32());

            let fac = cmfac * gfac;

            MKS::new( other.value_unsafe * fac )
        }
    }
}
