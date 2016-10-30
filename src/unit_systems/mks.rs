//! The **mks** module provides a unit system for use with Gaussian MKS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!
//!

#![allow(missing_docs)]

make_units! {
    MKS;
    ONE: Unitless;
    base {
        P2, M: Meter, "m";
        P2, KG: Kilogram, "kg";
        P1, S: Second, "s";
    }
    derived {
        MPS: MeterPerSecond = (Meter / Second);
    }
    fmt = true;
}

pub use self::f64consts::*;
