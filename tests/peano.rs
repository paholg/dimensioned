extern crate dimensioned;
use dimensioned::peano::*;

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
    assert_eq!( 0, <<Zero as AddPeano<Zero>>::Output as ToInt>::to_int() );
    // 0 + 3 == 3
    assert_eq!( 3, <<Zero as AddPeano<Three>>::Output as ToInt>::to_int() );
    // 0 + -3 == -3
    assert_eq!( -3, <<Zero as AddPeano<NegThree>>::Output as ToInt>::to_int() );

    // 2 + 0 == 2
    assert_eq!( 2, <<Two as AddPeano<Zero>>::Output as ToInt>::to_int() );
    // 2 + 3 == 5
    assert_eq!( 5, <<Two as AddPeano<Three>>::Output as ToInt>::to_int() );
    // 2 + -3 == -1
    assert_eq!( -1, <<Two as AddPeano<NegThree>>::Output as ToInt>::to_int() );
    // 3 + -2 == 1
    assert_eq!( 1, <<Three as AddPeano<NegTwo>>::Output as ToInt>::to_int() );

    // -2 + 0 == 2
    assert_eq!( -2, <<NegTwo as AddPeano<Zero>>::Output as ToInt>::to_int() );
    // -2 + -3 == -5
    assert_eq!( -5, <<NegTwo as AddPeano<NegThree>>::Output as ToInt>::to_int() );
    // -2 + 3 == 1
    assert_eq!( 1, <<NegTwo as AddPeano<Three>>::Output as ToInt>::to_int() );
    // -3 + 2 == -1
    assert_eq!( -1, <<NegThree as AddPeano<Two>>::Output as ToInt>::to_int() );


    // Testing Negation
    // -3 == -(3)
    assert_eq!( -3, <<Three as Negate>::Output as ToInt>::to_int() );
    // 3 == -(-3)
    assert_eq!( 3, <<NegThree as Negate>::Output as ToInt>::to_int() );
    // 0 == -0
    assert_eq!( 0, <<Zero as Negate>::Output as ToInt>::to_int() );


    // Testing Subtraction
    // 0 - 0 == 0
    assert_eq!( 0, <<Zero as SubPeano<Zero>>::Output as ToInt>::to_int() );
    // 0 - 3 == -3
    assert_eq!( -3, <<Zero as SubPeano<Three>>::Output as ToInt>::to_int() );
    // 0 - -3 == 3
    assert_eq!( 3, <<Zero as SubPeano<NegThree>>::Output as ToInt>::to_int() );

    // 2 - 0 == 2
    assert_eq!( 2, <<Two as SubPeano<Zero>>::Output as ToInt>::to_int() );
    // 2 - 3 == -1
    assert_eq!( -1, <<Two as SubPeano<Three>>::Output as ToInt>::to_int() );
    // 2 - -3 == 5
    assert_eq!( 5, <<Two as SubPeano<NegThree>>::Output as ToInt>::to_int() );
    // 3 - -2 == 5
    assert_eq!( 5, <<Three as SubPeano<NegTwo>>::Output as ToInt>::to_int() );

    // -2 - 0 == -2
    assert_eq!( -2, <<NegTwo as SubPeano<Zero>>::Output as ToInt>::to_int() );
    // -2 - -3 == 1
    assert_eq!( 1, <<NegTwo as SubPeano<NegThree>>::Output as ToInt>::to_int() );
    // -2 - 3 == -5
    assert_eq!( -5, <<NegTwo as SubPeano<Three>>::Output as ToInt>::to_int() );
    // -3 - 2 == -5
    assert_eq!( -5, <<NegThree as SubPeano<Two>>::Output as ToInt>::to_int() );


    // Testing Multiplication
    // 0 * 0 == 0
    assert_eq!( 0, <<Zero as MulPeano<Zero>>::Output as ToInt>::to_int() );
    // 0 * 2 == 0
    assert_eq!( 0, <<Zero as MulPeano<Two>>::Output as ToInt>::to_int() );
    // 2 * 0 == 0
    assert_eq!( 0, <<Two as MulPeano<Zero>>::Output as ToInt>::to_int() );

    // 2 * 3 == 6
    assert_eq!( 6, <<Two as MulPeano<Three>>::Output as ToInt>::to_int() );
    // 2 * -3 == -6
    assert_eq!( -6, <<Two as MulPeano<NegThree>>::Output as ToInt>::to_int() );
    // -2 * 3 == -6
    assert_eq!( -6, <<NegTwo as MulPeano<Three>>::Output as ToInt>::to_int() );
    // -2 * -3 == 6
    assert_eq!( 6, <<NegTwo as MulPeano<NegThree>>::Output as ToInt>::to_int() );

    // Testing Division
    // 0 / 2 == 0
    assert_eq!( 0, <<Zero as DivPeano<Two>>::Output as ToInt>::to_int() );
    // 1 / 1 == 1
    assert_eq!( 1, <<One as DivPeano<One>>::Output as ToInt>::to_int() );
    // 4 / 2 == 2
    assert_eq!( 2, <<Four as DivPeano<Two>>::Output as ToInt>::to_int() );
    // 4 / -2 == -2
    assert_eq!( -2, <<Four as DivPeano<NegTwo>>::Output as ToInt>::to_int() );
    // -4 / 2 == -2
    assert_eq!( -2, <<NegFour as DivPeano<Two>>::Output as ToInt>::to_int() );
    // -4 / -2 == 2
    assert_eq!( 2, <<NegFour as DivPeano<NegTwo>>::Output as ToInt>::to_int() );

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
