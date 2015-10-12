extern crate peano;
extern crate dimensioned as dim;

use peano::{P2, P3, P6};
use dim::si::{m};
use dim::{Pow, Root, Sqrt, Cbrt};


#[test]
fn pows() {
    let x = 2.0*m;
    assert_eq!(x, P2::pow(x).sqrt());
    assert_eq!(x, P3::pow(x).cbrt());
    assert_eq!(x, P6::root(P6::pow(x)));
    println!("{}, {}", x, x*x);
}
