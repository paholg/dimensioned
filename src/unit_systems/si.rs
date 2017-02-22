//! The `si` module provides a unit system for use with SI units. It was generated using
//! the `make_units!` macro. See its documentation for more information.
//!

#![allow(missing_docs)]

make_units! {
    SI;
    ONE: Unitless;

    base {
        M:   Meter,    "m",   P1;
        KG:  Kilogram, "kg",  P1;
        S:   Second,   "s",   P1;
        A:   Ampere,   "A",   P1;
        K:   Kelvin,   "K",   P1;
        CD:  Candela,  "cd",  P1;
        MOL: Mole,     "mol", P1;
    }

    derived {
        HZ: Hertz = (Unitless / Second);
        N: Newton = (Kilogram * Meter / Second2);
        PA: Pascal = (Newton / Meter2);
        J: Joule = (Newton * Meter);
        W: Watt = (Joule / Second);
        C: Coulomb = (Second * Ampere);
        V: Volt = (Watt / Ampere);
        F: Farad = (Coulomb / Volt);
        OHM: Ohm = (Volt / Ampere);
        SIEMENS: Siemens = (Ampere / Volt);
        WB: Weber = (Joule / Ampere);
        T: Tesla = (Weber / Meter2);
        H: Henry = (Ohm * Second);
        LM: Lumen = (Candela);
        LX: Lux = (Candela / Meter2);
        BQ: Becquerel = (Hertz);
        GY: Gray = (Joule / Kilogram);
        SV: Sievert = (Gray);
        KAT: Katal = (Mole / Second);

        M2: Meter2 = (Meter * Meter);
        M3: Meter3 = (Meter2 * Meter);

        IM: InverseMeter = (Unitless / Meter);
        IM2: InverseMeter2 = (InverseMeter / Meter);
        IM3: InverseMeter3 = (InverseMeter2 / Meter);

        S2: Second2 = (Second * Second);
        S3: Second3 = (Second2 * Second);
        S4: Second4 = (Second3 * Second);

        MPS: MeterPerSecond = (Meter / Second);
        MPS2: MeterPerSecond2 = (Meter / Second2);
        MPS3: MeterPerSecond3 = (Meter / Second3);
        MPS4: MeterPerSecond4 = (Meter / Second4);

        M2PS: Meter2PerSecond = (Meter2 / Second);
        M2PS2: Meter2PerSecond2 = (Meter2 / Second2);
        M2PS3: Meter2PerSecond3 = (Meter2 / Second3);

        M3PS: Meter3PerSecond = (Meter3 / Second);
        M3PS2: Meter3PerSecond2 = (Meter3 / Second2);
        M3PS3: Meter3PerSecond3 = (Meter3 / Second3);

        NS: NewtonSecond = (Newton * Second);
        JS: JouleSecond = (Joule * Second);
        NPS: NewtonPerSecond = (Newton / Second);
    }

    constants {
        MIN: Second = 60.0;
        HR: Second = 60.0*60.0;
        DAY: Second = 24.0*60.0*60.0;
    }

    fmt = true;
}

pub use self::f64consts::*;


#[test]
fn test_si() {
    use si::f64consts::*;

    let mut x = 2.25 * M * M / S / KG * N;
    let y = x / 3.0;
    x /= 3.0;

    assert_eq!(x, y);

    let mut z = ONE;
    let w = z + 4.0;
    z += 4.0;
    assert_eq!(z, w);
}

// #[cfg(feature = "std")]
// #[test]
// fn test_index() {
//     use si::Meter;
//     let mut v = Meter::new(vec![1.0, 2.0]);

//     v[0] += 1.2 * M;
//     assert_eq!(v, Meter::new(vec![2.2, 2.0]));
// }

#[test]
fn kg_test() {
    use si::{KG, M, S};
    let mass = 3.0 * KG;
    let dist = 2.0 * M;
    let time = 2.0 * S;
    let force = Newton::new(1.5);

    assert_eq!(force, mass * dist / time / time);
}


mod conversion {
    // Convert from UCUM
    use super::SI;
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
