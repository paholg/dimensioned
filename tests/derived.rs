extern crate dimensioned as dim;

use dim::Dim;
use dim::si::{Meter, Second};
use std::ops::Div;

type MPS = <Meter as Div<Second>>::Output;

// fn doodoo() -> Dim<MPS, f64> {
//     Dim::new(1.0)
// }

#[test]
fn derived() {
    let v: Dim<MPS, f64> = Dim::new(1.0);
    println!("v: {}!", v);
}
