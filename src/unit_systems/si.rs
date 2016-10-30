//! The **si** module provides a unit system for use with SI units. It was generated using
//! the `make_units!` macro. See its documentation for more information.
//!
//! It will also define derived units, although this is not implemented yet.
//!

#![allow(missing_docs)]

make_units! {
    SI;
    ONE: Unitless;
    base {
        M:   Meter,    "m";
        KG:  Kilogram, "kg";
        S:   Second,   "s";
        A:   Ampere,   "A";
        K:   Kelvin,   "K";
        CD:  Candela,  "cd";
        MOL: Mole,     "mol";
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
}

pub use self::f64consts::*;


#[test]
fn test() {
    use si::f64consts::*;

    let mut x = 2.25 * M*M/S/KG*N;
    let y = x / 3.0;
    x /= 3.0;

    assert_eq!(x, y);

    let mut z = ONE;
    let w = z + 4.0;
    z += 4.0;
    assert_eq!(z, w);
}


// #[test]
// fn kg_test() {
//     use si::consts::{kg, m, s};
//     let mass = 3.0 * kg;
//     let dist = 2.0 * m;
//     let time = 2.0 * s;
//     let f = Newton::new(1.5);

//     assert_eq!(f, mass * dist / time / time);
// }
