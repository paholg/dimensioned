extern crate dimensioned as dim;

use crate::dim::si::{Meter, Second};
use crate::dim::typenum::Quot;

type MPS<T> = Quot<Meter<T>, Second<T>>;

fn speed(dist: Meter<f64>, time: Second<f64>) -> MPS<f64> {
    dist / time
}

#[test]
fn derived() {
    let d = Meter::new(3.0);
    let t = Second::new(2.0);

    let v = speed(d, t);

    assert_eq!(d / t, v);
}
