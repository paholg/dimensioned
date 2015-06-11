/*!
The **cgs** module provides a unit system for use with Gaussian CGS units.

It will also define derived units, although this is not implemented yet.
*/

#![allow(missing_docs)]

make_units_adv! {
    CGS, Unitless, one, f64, 1.0;
    base {
        Two, Centimeter, cm, cm;
        Two, Gram, g, g;
        One, Second, s, s;
    }
    derived {
    }
}
