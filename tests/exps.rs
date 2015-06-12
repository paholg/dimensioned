extern crate dimensioned as dim;

use dim::si::{m};
use dim::{Pow, Root, Sqrt, Cbrt};
use dim::peano::{P2, P3, P6};


#[test]
fn pows() {
    let x = 2.0*m;
    assert_eq!(x, P2::pow(x).sqrt());
    assert_eq!(x, P3::pow(x).cbrt());
    assert_eq!(x, P6::root(P6::pow(x)));
    println!("asdsad");
}
