#![feature(type_macros)]
#[macro_use]
extern crate dimensioned as dim;

use dim::Quantity;
use dim::si::{Meter, Second};
use std::ops::Div;
use std::marker::PhantomData;

type MPS = unit!(Meter / Second);

fn speed(dist: Quantity<Meter, f64>, time: Quantity<Second, f64>) -> Quantity<MPS, f64> {
    dist / time
}

#[test]
fn derived() {
    let d: Quantity<Meter, f64> = Quantity::new(3.0);
    let t: Quantity<Second, f64> = Quantity::new(2.0);

    let v = speed(d, t);

    assert_eq!(d/t, v);
}
