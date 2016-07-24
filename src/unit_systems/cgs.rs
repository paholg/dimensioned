/*!
The **cgs** module provides a unit system for use with Gaussian CGS units. It was
generated using the `make_units!` macro. See its documentation for more information.

It will also define derived units, although this is not implemented yet.
*/

#![allow(missing_docs)]

make_units_adv! {
    CGS, Unitless, one, f64, 1.0;
    base {
        P2, Centimeter, cm, cm;
        P2, Gram, g, g;
        P1, Second, s, s;
    }
    derived {
    }
}

pub trait FromCGS<Centimeter: Integer, Gram: Integer, Second: Integer, V> where Self: Sized {
    fn from_cgs(from: Dim<CGS<Centimeter, Gram, Second>, V>) -> Dim<Self, V>;
}

use mks::{MKS, FromMKS};
impl<Meter, Kilogram, Second, V> FromMKS<Meter, Kilogram, Second, V> for CGS<Meter, Kilogram, Second>
    where V: Mul<f64, Output=V>, Meter: Integer, Kilogram: Integer, Second: Integer, {
    fn from_mks(from: Dim<MKS<Meter, Kilogram, Second>, V>) -> Dim<Self, V> {
        Dim::new( from.0 * 100f64.sqrt().powi(Meter::to_i32()) * 1000f64.sqrt().powi(Kilogram::to_i32()) )
    }
}
