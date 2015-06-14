extern crate dimensioned;
use std::ops::{Add, Mul};

use dimensioned::peano::{Peano, NonNeg, ToInt, Succ, Zero, P1, P8};

trait Fac: Peano {
    type Output;
}

impl Fac for Zero {
    type Output = P1;
}
impl<N> Fac for Succ<N> where N: NonNeg + Fac + Mul<<N as Fac>::Output>,
          <N as Fac>::Output: Add<<N as Mul<<N as Fac>::Output>>::Output> {
    type Output = <Succ<N> as Mul<<N as Fac>::Output>>::Output;
}

fn main() {
    type X = <P8 as Mul<P8>>::Output;
    assert_eq!(24, <X as ToInt>::to_int());
}
