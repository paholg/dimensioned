// use std::intrinsics::get_tydesc;

pub struct Zero;
pub struct Succ<T: NonNeg>;
pub struct Pred<T: NonPos>;

pub type One = Succ<Zero>;

pub trait Peano {}
pub trait NonZero: Peano {}
pub trait NonNeg: Peano {}
pub trait NonPos: Peano {}
pub trait Pos: Peano + NonZero + NonNeg {}
pub trait Neg: Peano + NonZero + NonPos {}

impl Peano for Zero {}
impl NonNeg for Zero {}
impl NonPos for Zero {}

impl<T: NonNeg> Peano for Succ<T> {}
impl<T: NonNeg> NonNeg for Succ<T> {}
impl<T: NonNeg> NonZero for Succ<T> {}
impl<T: NonNeg> Pos for Succ<T> {}

impl<T: NonPos> Peano for Pred<T> {}
impl<T: NonPos> NonPos for Pred<T> {}
impl<T: NonPos> NonZero for Pred<T> {}
impl<T: NonPos> Neg for Pred<T> {}

impl Copy for Zero {}

pub trait AddPeano<Rhs> {
    type Result;
}

/// Adding things to zero (e.g. 0 + 3)
impl<Rhs: Peano> AddPeano<Rhs> for Zero {
    type Result = Rhs;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<T: NonNeg + AddPeano<Succ<Rhs>>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Succ<T> {
    type Result = Succ<<T as AddPeano<Succ<Rhs>>>::Result>;
}
/// Adding zero to positive numbers (e.g. 3 + 0)
impl<T: NonNeg> AddPeano<Zero> for Succ<T> {
    type Result = Succ<T>;
}
/// Adding negative numbers to positive numbers (e.g. 2 + -3)
impl<T: NonNeg + AddPeano<Rhs>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Succ<T> {
    type Result = <T as AddPeano<Rhs>>::Result;
}

/// Adding negative numbers (e.g. -1 + -2)
impl<T: NonPos + AddPeano<Pred<Rhs>>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Pred<T> {
    type Result = Pred<<T as AddPeano<Pred<Rhs>>>::Result>;
}
/// Adding zero to negative numbers (e.g. -3 + 0)
impl<T: NonPos> AddPeano<Zero> for Pred<T> {
    type Result = Pred<T>;
}
/// Adding positive numbers to negative numbers (e.g. -2 + 3)
impl<T: NonPos + AddPeano<Rhs>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Pred<T> {
    type Result = <T as AddPeano<Rhs>>::Result;
}

pub trait Negate {
    type Result;
}
impl Negate for Zero {
    type Result = Zero;
}
impl<T: NonNeg + Negate> Negate for Succ<T> {
    type Result = Pred<<T as Negate>::Result>;
}
impl<T: NonPos + Negate> Negate for Pred<T> {
    type Result = Succ<<T as Negate>::Result>;
}

pub trait SubPeano<Rhs> {
    type Result;
}

/// Subtracting from zero
impl<Rhs: Peano + Negate> SubPeano<Rhs> for Zero {
    type Result = <Rhs as Negate>::Result;
}

/// Subtracting positive numbers from positive numbers (e.g. 3 - 4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Succ<T> {
    type Result = <T as SubPeano<Rhs>>::Result;
}
/// Subtracting zero from positive numbers (e.g. 3 - 0)
impl<T: NonNeg> SubPeano<Zero> for Succ<T> {
    type Result = Succ<T>;
}
/// Subtracting negative numbers from positive numbers (e.g. 3 - -4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Succ<T> {
    type Result = Succ<Succ<<T as SubPeano<Rhs>>::Result>>;
}

/// Subtracting positive numbers from negative numbers (e.g. -3 - 4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Pred<T> {
    type Result = Pred<Pred<<T as SubPeano<Rhs>>::Result>>;
}
/// Subtracting zero from negative numbers (e.g. -3 - 0)
impl<T: NonPos> SubPeano<Zero> for Pred<T> {
    type Result = Pred<T>;
}
/// Subtracting negative numbers from negative numbers (e.g. -3 - -4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Pred<T> {
    type Result = <T as SubPeano<Rhs>>::Result;
}

pub trait MulPeano<Rhs> {
    type Result;
}

/// Multiplying zero by things (e.g. 0 * 7)
impl<Rhs: Peano> MulPeano<Rhs> for Zero {
    type Result = Zero;
}

/// Multiplying a positive integer by an arbitrary integer (e.g. 2 * N)
impl<T, Rhs> MulPeano<Rhs> for Succ<T>
    where T: NonNeg + MulPeano<Rhs>, Rhs: AddPeano<<T as MulPeano<Rhs>>::Result> {
        type Result = <Rhs as AddPeano<<T as MulPeano<Rhs>>::Result>>::Result;
}

/// Multiplying a negative integer by an arbitrary integer (e.g. -2 * N)
impl<T, Rhs> MulPeano<Rhs> for Pred<T>
    where T: NonPos + MulPeano<Rhs>, Rhs: Peano, <T as MulPeano<Rhs>>::Result: SubPeano<Rhs> {
        type Result = <<T as MulPeano<Rhs>>::Result as SubPeano<Rhs>>::Result;
}

// #[allow(dead_code)]
// fn print_type<T>() {
//     let type_name = unsafe { (*get_tydesc::<T>()).name };
//     println!("{}", type_name);
// }

trait ToInt {
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<T:NonNeg + ToInt> ToInt for Succ<T> {
    fn to_int() -> i32 { 1 + <T as ToInt>::to_int() }
}
impl<T:NonPos + ToInt> ToInt for Pred<T> {
    fn to_int() -> i32 { -1 + <T as ToInt>::to_int() }
}


#[test]
fn test_peano() {
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type Three = Succ<Two>;

    type NegOne = Pred<Zero>;
    type NegTwo = Pred<NegOne>;
    type NegThree = Pred<NegTwo>;


    // Testing equality
    // 0 == 0
    assert_eq!( 0, <Zero as ToInt>::to_int() );
    // 2 == 2
    assert_eq!( 2, <Two as ToInt>::to_int() );
    // -2 == -2
    assert_eq!( -2, <NegTwo as ToInt>::to_int() );


    // Testing addition
    // 0 + 0 == 0
    assert_eq!( 0, <<Zero as AddPeano<Zero>>::Result as ToInt>::to_int() );
    // 0 + 3 == 3
    assert_eq!( 3, <<Zero as AddPeano<Three>>::Result as ToInt>::to_int() );
    // 0 + -3 == -3
    assert_eq!( -3, <<Zero as AddPeano<NegThree>>::Result as ToInt>::to_int() );

    // 2 + 0 == 2
    assert_eq!( 2, <<Two as AddPeano<Zero>>::Result as ToInt>::to_int() );
    // 2 + 3 == 5
    assert_eq!( 5, <<Two as AddPeano<Three>>::Result as ToInt>::to_int() );
    // 2 + -3 == -1
    assert_eq!( -1, <<Two as AddPeano<NegThree>>::Result as ToInt>::to_int() );
    // 3 + -2 == 1
    assert_eq!( 1, <<Three as AddPeano<NegTwo>>::Result as ToInt>::to_int() );

    // -2 + 0 == 2
    assert_eq!( -2, <<NegTwo as AddPeano<Zero>>::Result as ToInt>::to_int() );
    // -2 + -3 == -5
    assert_eq!( -5, <<NegTwo as AddPeano<NegThree>>::Result as ToInt>::to_int() );
    // -2 + 3 == 1
    assert_eq!( 1, <<NegTwo as AddPeano<Three>>::Result as ToInt>::to_int() );
    // -3 + 2 == -1
    assert_eq!( -1, <<NegThree as AddPeano<Two>>::Result as ToInt>::to_int() );


    // Testing Negation
    // -3 == -(3)
    assert_eq!( -3, <<Three as Negate>::Result as ToInt>::to_int() );
    // 3 == -(-3)
    assert_eq!( 3, <<NegThree as Negate>::Result as ToInt>::to_int() );
    // 0 == -0
    assert_eq!( 0, <<Zero as Negate>::Result as ToInt>::to_int() );


    // Testing Subtraction
    // 0 - 0 == 0
    assert_eq!( 0, <<Zero as SubPeano<Zero>>::Result as ToInt>::to_int() );
    // 0 - 3 == -3
    assert_eq!( -3, <<Zero as SubPeano<Three>>::Result as ToInt>::to_int() );
    // 0 - -3 == 3
    assert_eq!( 3, <<Zero as SubPeano<NegThree>>::Result as ToInt>::to_int() );

    // 2 - 0 == 2
    assert_eq!( 2, <<Two as SubPeano<Zero>>::Result as ToInt>::to_int() );
    // 2 - 3 == -1
    assert_eq!( -1, <<Two as SubPeano<Three>>::Result as ToInt>::to_int() );
    // 2 - -3 == 5
    assert_eq!( 5, <<Two as SubPeano<NegThree>>::Result as ToInt>::to_int() );
    // 3 - -2 == 5
    assert_eq!( 5, <<Three as SubPeano<NegTwo>>::Result as ToInt>::to_int() );

    // -2 - 0 == -2
    assert_eq!( -2, <<NegTwo as SubPeano<Zero>>::Result as ToInt>::to_int() );
    // -2 - -3 == 1
    assert_eq!( 1, <<NegTwo as SubPeano<NegThree>>::Result as ToInt>::to_int() );
    // -2 - 3 == -5
    assert_eq!( -5, <<NegTwo as SubPeano<Three>>::Result as ToInt>::to_int() );
    // -3 - 2 == -5
    assert_eq!( -5, <<NegThree as SubPeano<Two>>::Result as ToInt>::to_int() );


    // Testing Multiplication
    // 0 * 0 == 0
    assert_eq!( 0, <<Zero as MulPeano<Zero>>::Result as ToInt>::to_int() );
    // 0 * 2 == 0
    assert_eq!( 0, <<Zero as MulPeano<Two>>::Result as ToInt>::to_int() );
    // 2 * 0 == 0
    assert_eq!( 0, <<Two as MulPeano<Zero>>::Result as ToInt>::to_int() );

    // 2 * 3 == 6
    assert_eq!( 6, <<Two as MulPeano<Three>>::Result as ToInt>::to_int() );
    // 2 * -3 == -6
    assert_eq!( -6, <<Two as MulPeano<NegThree>>::Result as ToInt>::to_int() );
    // -2 * 3 == -6
    assert_eq!( -6, <<NegTwo as MulPeano<Three>>::Result as ToInt>::to_int() );
    // -2 * -3 == 6
    assert_eq!( 6, <<NegTwo as MulPeano<NegThree>>::Result as ToInt>::to_int() );

}
