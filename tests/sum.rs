extern crate dimensioned as dim;

use crate::dim::si::*;

#[test]
fn sum() {
    let lengths = [Meter::new(1), Meter::new(3), Meter::new(5)];
    let sum = lengths.iter().map(|l| *l).sum();
    assert_eq!(Meter::new(9), sum);

    let empty_lengths: [Meter<f64>; 0] = [];
    let sum = empty_lengths.iter().map(|l| *l).sum();
    assert_eq!(Meter::new(0.0), sum);
}
