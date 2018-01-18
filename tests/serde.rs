#![cfg(feature = "serde")]

extern crate dimensioned as dim;
extern crate serde;
extern crate serde_test;

use dim::si;
use serde_test::{assert_tokens, Token};

#[test]
fn serialization() {
    let dist = 6.0 * si::M;
    assert_tokens(&dist, &[Token::F64(6.0)]);

    let speed = 2.0 * si::MPS;
    assert_tokens(&speed, &[Token::F64(2.0)]);
}
