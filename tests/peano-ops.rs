extern crate dimensioned;

use dimensioned::peano::*;

use std::ops::{Add, Neg, Sub, Mul, Div};

#[test]
fn test_peano() {
    // Testing equality
    // 0 == 0
    assert_eq!( 0, <Zero as ToInt>::to_int() );
    // 2 == 2
    assert_eq!( 2, <Two as ToInt>::to_int() );
    // -2 == -2
    assert_eq!( -2, <NegTwo as ToInt>::to_int() );

    // Testing addition
    // 0 + 0 == 0
    assert_eq!( 0, <<Zero as Add<Zero>>::Output as ToInt>::to_int() );
    // 0 + 3 == 3
    assert_eq!( 3, <<Zero as Add<Three>>::Output as ToInt>::to_int() );
    // 0 + -3 == -3
    assert_eq!( -3, <<Zero as Add<NegThree>>::Output as ToInt>::to_int() );

    // 2 + 0 == 2
    assert_eq!( 2, <<Two as Add<Zero>>::Output as ToInt>::to_int() );
    // 2 + 3 == 5
    assert_eq!( 5, <<Two as Add<Three>>::Output as ToInt>::to_int() );
    // 2 + -3 == -1
    assert_eq!( -1, <<Two as Add<NegThree>>::Output as ToInt>::to_int() );
    // 3 + -2 == 1
    assert_eq!( 1, <<Three as Add<NegTwo>>::Output as ToInt>::to_int() );

    // -2 + 0 == 2
    assert_eq!( -2, <<NegTwo as Add<Zero>>::Output as ToInt>::to_int() );
    // -2 + -3 == -5
    assert_eq!( -5, <<NegTwo as Add<NegThree>>::Output as ToInt>::to_int() );
    // -2 + 3 == 1
    assert_eq!( 1, <<NegTwo as Add<Three>>::Output as ToInt>::to_int() );
    // -3 + 2 == -1
    assert_eq!( -1, <<NegThree as Add<Two>>::Output as ToInt>::to_int() );


    // Testing Negation
    // -3 == -(3)
    assert_eq!( -3, <<Three as Neg>::Output as ToInt>::to_int() );
    // 3 == -(-3)
    assert_eq!( 3, <<NegThree as Neg>::Output as ToInt>::to_int() );
    // 0 == -0
    assert_eq!( 0, <<Zero as Neg>::Output as ToInt>::to_int() );


    // Testing Subtraction
    // 0 - 0 == 0
    assert_eq!( 0, <<Zero as Sub<Zero>>::Output as ToInt>::to_int() );
    // 0 - 3 == -3
    assert_eq!( -3, <<Zero as Sub<Three>>::Output as ToInt>::to_int() );
    // 0 - -3 == 3
    assert_eq!( 3, <<Zero as Sub<NegThree>>::Output as ToInt>::to_int() );

    // 2 - 0 == 2
    assert_eq!( 2, <<Two as Sub<Zero>>::Output as ToInt>::to_int() );
    // 2 - 3 == -1
    assert_eq!( -1, <<Two as Sub<Three>>::Output as ToInt>::to_int() );
    // 2 - -3 == 5
    assert_eq!( 5, <<Two as Sub<NegThree>>::Output as ToInt>::to_int() );
    // 3 - -2 == 5
    assert_eq!( 5, <<Three as Sub<NegTwo>>::Output as ToInt>::to_int() );

    // -2 - 0 == -2
    assert_eq!( -2, <<NegTwo as Sub<Zero>>::Output as ToInt>::to_int() );
    // -2 - -3 == 1
    assert_eq!( 1, <<NegTwo as Sub<NegThree>>::Output as ToInt>::to_int() );
    // -2 - 3 == -5
    assert_eq!( -5, <<NegTwo as Sub<Three>>::Output as ToInt>::to_int() );
    // -3 - 2 == -5
    assert_eq!( -5, <<NegThree as Sub<Two>>::Output as ToInt>::to_int() );


    // Testing Multiplication
    // 0 * 0 == 0
    assert_eq!( 0, <<Zero as Mul<Zero>>::Output as ToInt>::to_int() );
    // 0 * 2 == 0
    assert_eq!( 0, <<Zero as Mul<Two>>::Output as ToInt>::to_int() );
    // 2 * 0 == 0
    assert_eq!( 0, <<Two as Mul<Zero>>::Output as ToInt>::to_int() );
    // 0 * -2 == 0
    assert_eq!( 0, <<Zero as Mul<NegTwo>>::Output as ToInt>::to_int() );
    // -2 * 0 == 0
    assert_eq!( 0, <<NegTwo as Mul<Zero>>::Output as ToInt>::to_int() );

    // 2 * 3 == 6
    assert_eq!( 6, <<Two as Mul<Three>>::Output as ToInt>::to_int() );
    // 2 * -3 == -6
    assert_eq!( -6, <<Two as Mul<NegThree>>::Output as ToInt>::to_int() );
    // -2 * 3 == -6
    assert_eq!( -6, <<NegTwo as Mul<Three>>::Output as ToInt>::to_int() );
    // -2 * -3 == 6
    assert_eq!( 6, <<NegTwo as Mul<NegThree>>::Output as ToInt>::to_int() );

    // Testing Division
    // 0 / 2 == 0
    assert_eq!( 0, <<Zero as Div<Two>>::Output as ToInt>::to_int() );
    // 1 / 1 == 1
    assert_eq!( 1, <<One as Div<One>>::Output as ToInt>::to_int() );
    // 4 / 2 == 2
    assert_eq!( 2, <<Four as Div<Two>>::Output as ToInt>::to_int() );
    // 4 / -2 == -2
    assert_eq!( -2, <<Four as Div<NegTwo>>::Output as ToInt>::to_int() );
    // -4 / 2 == -2
    assert_eq!( -2, <<NegFour as Div<Two>>::Output as ToInt>::to_int() );
    // -4 / -2 == 2
    assert_eq!( 2, <<NegFour as Div<NegTwo>>::Output as ToInt>::to_int() );

    // Uncomment for erroneous divisions!
    // // 3 / 2
    // <<Three as DivPeano<Two>>::Output as ToInt>::to_int();
    // // -3 / 2
    // <<NegThree as DivPeano<Two>>::Output as ToInt>::to_int();
    // // 3 / -2
    // <<Three as DivPeano<NegTwo>>::Output as ToInt>::to_int();
    // // -3 / -2
    // <<NegThree as DivPeano<NegTwo>>::Output as ToInt>::to_int();
    // // 2 / 0
    // <<Two as DivPeano<Zero>>::Output as ToInt>::to_int();
    // // -2 / 0
    // <<NegTwo as DivPeano<Zero>>::Output as ToInt>::to_int();
}
