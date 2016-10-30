//! The **si** module provides a unit system for use with SI units. It was generated using
//! the `make_units!` macro. See its documentation for more information.
//!
//! It will also define derived units, although this is not implemented yet.
//!

#![allow(missing_docs)]

make_units! {
    SI, Unitless;
    base {
        Meter, "m";
        Kilogram, "kg";
        Second, "s";
        Ampere, "A";
        Kelvin, "K";
        Candela, "cd";
        Mole, "mol";
    }

    derived {
        Meter2 = (Meter * Meter);
        Meter3 = (Meter2 * Meter);

        Second2 = (Second * Second);
        Second3 = (Second2 * Second);
        Second4 = (Second3 * Second);

        MeterPerSecond = (Meter / Second);
        MeterPerSecond2 = (Meter / Second2);
        MeterPerSecond3 = (Meter / Second3);
        MeterPerSecond4 = (Meter / Second4);

        Meter2PerSecond = (Meter2 / Second);
        Meter2PerSecond2 = (Meter2 / Second2);
        Meter2PerSecond3 = (Meter2 / Second3);

        Meter3PerSecond = (Meter3 / Second);
        Meter3PerSecond2 = (Meter3 / Second2);
        Meter3PerSecond3 = (Meter3 / Second3);

        Hertz = (Unitless / Second);
        Newton = (Kilogram * Meter / Second2);
        Pascal = (Newton / Meter2);
        Joule = (Newton * Meter);
        Watt = (Joule / Second);
        Coulomb = (Second * Ampere);
        Volt = (Watt / Ampere);
        Farad = (Coulomb / Volt);
        Ohm = (Volt / Ampere);
        Siemens = (Ampere / Volt);
        Weber = (Joule / Ampere);
        Tesla = (Weber / Meter2);
        Henry = (Ohm * Second);
        Lumen = (Candela);
        Lux = (Candela / Meter2);
        Becquerel = (Hertz);
        Gray = (Joule / Kilogram);
        Sievert = (Gray);
        Katal = (Mole / Second);

        NewtonSecond = (Newton * Second);
        JouleSecond = (Joule * Second);
        NewtonPerSecond = (Newton / Second);
    }
}


#[test]
fn test() {
    use si;
    use typenum::consts::*;

    let mut x = si::Unitless::new(1.0) * si::m * si::m / si::s * si::kg;
    let y = x / 3.0;
    x /= 3.0;

    assert_eq!(x, y);

    let mut z = si::Unitless::new(1.0);
    let w = z + 4.0;
    z += 4.0;
    assert_eq!(z, w);
}

#[allow(non_upper_case_globals)]
mod consts {
    use reexported::marker::PhantomData;
    use super::*;
    pub const one: Unitless<f64> = SI {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const m: Meter<f64> = SI {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const kg: Kilogram<f64> = SI {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const s: Second<f64> = SI {
        value: 1.0,
        _marker: PhantomData,
    };
}

pub use self::consts::*;


// #[test]
// fn kg_test() {
//     use si::consts::{kg, m, s};
//     let mass = 3.0 * kg;
//     let dist = 2.0 * m;
//     let time = 2.0 * s;
//     let f = Newton::new(1.5);

//     assert_eq!(f, mass * dist / time / time);
// }
