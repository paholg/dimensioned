extern crate dimensioned;
use dimensioned::peano::{P1, ToInt};

use std::ops::Add;

#[test]
fn test_addition() {
    type A1 = <P1 as Add<P1>>::Output;
    type A2 = <A1 as Add<A1>>::Output;
    type A3 = <A2 as Add<A2>>::Output;
    type A4 = <A3 as Add<A3>>::Output;
    // type A5 = <A4 as Add<A4>>::Output;
    // type A6 = <A5 as Add<A4>>::Output;
    // type A7 = <A6 as Add<A3>>::Output;
    // type A8 = <A7 as Add<A2>>::Output;
    // type A9 = <A8 as Add<P3>>::Output;
    // type Test = <A9 as KeepPeano<A8>>::Output;
    // let x: TEST;
    println!("THING: {}", <A4 as ToInt>::to_int());
}

