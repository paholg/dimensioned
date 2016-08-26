/*!
The **mks** module provides a unit system for use with Gaussian MKS units. It was
generated using the `make_units!` macro. See its documentation for more information.

It will also define derived units, although this is not implemented yet.
*/

#![allow(missing_docs)]

make_units_adv! {
    MKS, Unitless, one, f64, 1.0;
    base {
        P2, Meter, m, m;
        P2, Kilogram, kg, kg;
        P1, Second, s, s;
    }
    derived {
    }
}

pub trait FromMKS<Meter: Integer, Kilogram: Integer, Second: Integer, V> where Self: Sized {
    fn from_mks(from: Quantity<MKS<Meter, Kilogram, Second>, V>) -> Quantity<Self, V>;
}

use cgs::{CGS, FromCGS};
impl<Centimeter, Gram, Second, V> FromCGS<Centimeter, Gram, Second, V> for MKS<Centimeter, Gram, Second>
    where V: Mul<f64, Output=V>, Centimeter: Integer, Gram: Integer, Second: Integer, {
    fn from_cgs(from: Quantity<CGS<Centimeter, Gram, Second>, V>) -> Quantity<Self, V> {
        Quantity::new( from.0 * 0.01f64.sqrt().powi(Centimeter::to_i32()) * 0.001f64.sqrt().powi(Gram::to_i32()) )
    }
}
