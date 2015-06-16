#![recursion_limit="128"]
extern crate dimensioned;
use dimensioned::peano::{Peano, NonNeg, ToInt, Succ, Zero, P1, P5};
use std::ops::{Mul, Add};

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
    type X = <P5 as Fac>::Output;
    // assert_eq!(120, <X as ToInt>::to_int());
}
