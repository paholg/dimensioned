extern crate dimensioned;
use dimensioned::peano::*;

#[test]
fn test_addition() {
    type A1 = <One as AddPeano<One>>::Output;
    type A2 = <A1 as AddPeano<A1>>::Output;
    type A3 = <A2 as AddPeano<A2>>::Output;
    type A4 = <A3 as AddPeano<A3>>::Output;
    type A5 = <A4 as AddPeano<A4>>::Output;
    type A6 = <A5 as AddPeano<A4>>::Output;
    type A7 = <A6 as AddPeano<A3>>::Output;
    type A8 = <A7 as AddPeano<A2>>::Output;
    type A9 = <A8 as AddPeano<Three>>::Output;
    println!("THING: {}", <A9 as ToInt>::to_int());

}

