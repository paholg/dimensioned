extern crate dimensioned;

use dimensioned::peano::{ToInt, Zero, P1, P2, P3, P4, N2, N3, N4};

use std::ops::{Add, Neg, Sub, Mul, Div};

#[test]
fn test_peano() {
    // Testing equality
    // 0 == 0
    assert_eq!( 0, <Zero as ToInt>::to_int() );
    // 2 == 2
    assert_eq!( 2, <P2 as ToInt>::to_int() );
    // -2 == -2
    assert_eq!( -2, <N2 as ToInt>::to_int() );

    // Testing addition
    // 0 + 0 == 0
    assert_eq!( 0, <<Zero as Add<Zero>>::Output as ToInt>::to_int() );
    // 0 + 3 == 3
    assert_eq!( 3, <<Zero as Add<P3>>::Output as ToInt>::to_int() );
    // 0 + -3 == -3
    assert_eq!( -3, <<Zero as Add<N3>>::Output as ToInt>::to_int() );

    // 2 + 0 == 2
    assert_eq!( 2, <<P2 as Add<Zero>>::Output as ToInt>::to_int() );
    // 2 + 3 == 5
    assert_eq!( 5, <<P2 as Add<P3>>::Output as ToInt>::to_int() );
    // 2 + -3 == -1
    assert_eq!( -1, <<P2 as Add<N3>>::Output as ToInt>::to_int() );
    // 3 + -2 == 1
    assert_eq!( 1, <<P3 as Add<N2>>::Output as ToInt>::to_int() );

    // -2 + 0 == 2
    assert_eq!( -2, <<N2 as Add<Zero>>::Output as ToInt>::to_int() );
    // -2 + -3 == -5
    assert_eq!( -5, <<N2 as Add<N3>>::Output as ToInt>::to_int() );
    // -2 + 3 == 1
    assert_eq!( 1, <<N2 as Add<P3>>::Output as ToInt>::to_int() );
    // -3 + 2 == -1
    assert_eq!( -1, <<N3 as Add<P2>>::Output as ToInt>::to_int() );


    // Testing Negation
    // -3 == -(3)
    assert_eq!( -3, <<P3 as Neg>::Output as ToInt>::to_int() );
    // 3 == -(-3)
    assert_eq!( 3, <<N3 as Neg>::Output as ToInt>::to_int() );
    // 0 == -0
    assert_eq!( 0, <<Zero as Neg>::Output as ToInt>::to_int() );


    // Testing Subtraction
    // 0 - 0 == 0
    assert_eq!( 0, <<Zero as Sub<Zero>>::Output as ToInt>::to_int() );
    // 0 - 3 == -3
    assert_eq!( -3, <<Zero as Sub<P3>>::Output as ToInt>::to_int() );
    // 0 - -3 == 3
    assert_eq!( 3, <<Zero as Sub<N3>>::Output as ToInt>::to_int() );

    // 2 - 0 == 2
    assert_eq!( 2, <<P2 as Sub<Zero>>::Output as ToInt>::to_int() );
    // 2 - 3 == -1
    assert_eq!( -1, <<P2 as Sub<P3>>::Output as ToInt>::to_int() );
    // 2 - -3 == 5
    assert_eq!( 5, <<P2 as Sub<N3>>::Output as ToInt>::to_int() );
    // 3 - -2 == 5
    assert_eq!( 5, <<P3 as Sub<N2>>::Output as ToInt>::to_int() );

    // -2 - 0 == -2
    assert_eq!( -2, <<N2 as Sub<Zero>>::Output as ToInt>::to_int() );
    // -2 - -3 == 1
    assert_eq!( 1, <<N2 as Sub<N3>>::Output as ToInt>::to_int() );
    // -2 - 3 == -5
    assert_eq!( -5, <<N2 as Sub<P3>>::Output as ToInt>::to_int() );
    // -3 - 2 == -5
    assert_eq!( -5, <<N3 as Sub<P2>>::Output as ToInt>::to_int() );


    // Testing Multiplication
    // 0 * 0 == 0
    assert_eq!( 0, <<Zero as Mul<Zero>>::Output as ToInt>::to_int() );
    // 0 * 2 == 0
    assert_eq!( 0, <<Zero as Mul<P2>>::Output as ToInt>::to_int() );
    // 2 * 0 == 0
    assert_eq!( 0, <<P2 as Mul<Zero>>::Output as ToInt>::to_int() );
    // 0 * -2 == 0
    assert_eq!( 0, <<Zero as Mul<N2>>::Output as ToInt>::to_int() );
    // -2 * 0 == 0
    assert_eq!( 0, <<N2 as Mul<Zero>>::Output as ToInt>::to_int() );

    // 2 * 3 == 6
    assert_eq!( 6, <<P2 as Mul<P3>>::Output as ToInt>::to_int() );
    // 2 * -3 == -6
    assert_eq!( -6, <<P2 as Mul<N3>>::Output as ToInt>::to_int() );
    // -2 * 3 == -6
    assert_eq!( -6, <<N2 as Mul<P3>>::Output as ToInt>::to_int() );
    // -2 * -3 == 6
    assert_eq!( 6, <<N2 as Mul<N3>>::Output as ToInt>::to_int() );

    // Testing Division
    // 0 / 2 == 0
    assert_eq!( 0, <<Zero as Div<P2>>::Output as ToInt>::to_int() );
    // 1 / 1 == 1
    assert_eq!( 1, <<P1 as Div<P1>>::Output as ToInt>::to_int() );
    // 4 / 2 == 2
    assert_eq!( 2, <<P4 as Div<P2>>::Output as ToInt>::to_int() );
    // 4 / -2 == -2
    assert_eq!( -2, <<P4 as Div<N2>>::Output as ToInt>::to_int() );
    // -4 / 2 == -2
    assert_eq!( -2, <<N4 as Div<P2>>::Output as ToInt>::to_int() );
    // -4 / -2 == 2
    assert_eq!( 2, <<N4 as Div<N2>>::Output as ToInt>::to_int() );

    // // Uncomment for erroneous divisions
    // // 3 / 2
    // <<P3 as Div<P2>>::Output as ToInt>::to_int();
    // // -3 / 2
    // <<N3 as Div<P2>>::Output as ToInt>::to_int();
    // // 3 / -2
    // <<P3 as Div<N2>>::Output as ToInt>::to_int();
    // // -3 / -2
    // <<N3 as Div<N2>>::Output as ToInt>::to_int();
    // // 2 / 0
    // <<P2 as Div<Zero>>::Output as ToInt>::to_int();
    // // -2 / 0
    // <<N2 as Div<Zero>>::Output as ToInt>::to_int();
}

#[test]
fn test_addition_time() {
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

