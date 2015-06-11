extern crate dimensioned as dim;

use dim::si::{m};
use dim::{Pow, Root, Sqrt, Cbrt};
use dim::peano::{Two, Three, Six};


#[test]
fn pows() {
    let x = 2.0*m;
    assert_eq!(x, Two::pow(x).sqrt());
    assert_eq!(x, Three::pow(x).cbrt());
    assert_eq!(x, Six::root(Six::pow(x)));
    println!("asdsad");
}
