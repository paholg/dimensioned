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
