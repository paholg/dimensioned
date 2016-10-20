//! The **si** module provides a unit system for use with SI units. It was generated using
//! the `make_units!` macro. See its documentation for more information.
//!
//! It will also define derived units, although this is not implemented yet.
//!

#![allow(missing_docs)]

make_units! {
    SI, Unitless, one;
    base {
        Meter, m, m;
        Kilogram, kg, kg;
        Second, s, s;
        Ampere, amp, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
        newton: Newton = (Kilogram * Meter / Second / Second);
    }
}

// #[test]
// fn kg_test() {
//     let mass = 3.0 * kg;
//     let dist = 2.0 * m;
//     let time = 2.0 * s;
//     let f: Dim<Newton, f64> = Dim::new(1.5);

//     assert_eq!(f, mass * dist / time / time);
// }
