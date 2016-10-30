//! The **mks** module provides a unit system for use with Gaussian MKS units. It was
//! generated using the `make_units!` macro. See its documentation for more information.
//!
//!

#![allow(missing_docs)]

make_units! {
    MKS, Unitless;
    base {
        P2, Meter, "m";
        P2, Kilogram, "kg";
        P1, Second, "s";
    }
    derived {
        MeterPerSecond = (Meter / Second);
    }
    fmt = true;
}

#[allow(non_upper_case_globals)]
mod consts {
    use reexported::marker::PhantomData;
    use super::*;
    pub const one: Unitless<f64> = MKS {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const m: Meter<f64> = MKS {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const kg: Kilogram<f64> = MKS {
        value: 1.0,
        _marker: PhantomData,
    };
    pub const s: Second<f64> = MKS {
        value: 1.0,
        _marker: PhantomData,
    };
}

pub use self::consts::*;
