//! The `mks` module provides a unit system for use with Gaussian MKS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!
//!

#![allow(missing_docs)]

make_units! {
    MKS;
    ONE: Unitless;

    base {
        M:  Meter,    "m",  P2;
        KG: Kilogram, "kg", P2;
        S:  Second,   "s",  P1;
    }

    derived {
        MPS: MeterPerSecond = (Meter / Second);
    }

    constants {}

    fmt = true;
}

pub use self::f64consts::*;
