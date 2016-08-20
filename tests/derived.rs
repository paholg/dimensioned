#![feature(type_macros)]
#[macro_use]
extern crate dimensioned as dim;

use dim::Dim;
use dim::si::{Meter, Second};
use std::ops::Div;
use std::marker::PhantomData;

type MPS = unit!(Meter / Second);

fn speed(dist: Dim<Meter, f64>, time: Dim<Second, f64>) -> Dim<MPS, f64> {
    dist / time
}

#[test]
fn derived() {
    let d: Dim<Meter, f64> = Dim::new(3.0);
    let t: Dim<Second, f64> = Dim::new(2.0);

    let v = speed(d, t);

    assert_eq!(d/t, v);
}
