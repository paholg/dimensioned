#[macro_use] extern crate dimensioned as dim;

use dim::typenum::{Pow, P2};
use dim::si::M;
use dim::{Root, Sqrt};


#[test]
fn pows() {
    let x = 2.0*M;
    let x2 = 4.0*M*M;

    assert_eq!(x2, x.powi(P2::new()));
    assert_eq!(x, x2.sqrt());
    // fixme: This fails. Why? It should be the same as calling sqrt().
    // assert_eq!(x, x2.root(P2::new()));
}

#[test]
fn rhs_multiplication() {
    let _ = M*2.0;
}
