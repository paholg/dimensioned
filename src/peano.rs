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
    type Output;
}

/// Adding things to zero (e.g. 0 + 3)
impl<Rhs: Peano> AddPeano<Rhs> for Zero {
    type Output = Rhs;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<T: NonNeg + AddPeano<Succ<Rhs>>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Succ<T> {
    type Output = Succ<<T as AddPeano<Succ<Rhs>>>::Output>;
}
/// Adding zero to positive numbers (e.g. 3 + 0)
impl<T: NonNeg> AddPeano<Zero> for Succ<T> {
    type Output = Succ<T>;
}
/// Adding negative numbers to positive numbers (e.g. 2 + -3)
impl<T: NonNeg + AddPeano<Rhs>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Succ<T> {
    type Output = <T as AddPeano<Rhs>>::Output;
}

/// Adding negative numbers (e.g. -1 + -2)
impl<T: NonPos + AddPeano<Pred<Rhs>>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Pred<T> {
    type Output = Pred<<T as AddPeano<Pred<Rhs>>>::Output>;
}
/// Adding zero to negative numbers (e.g. -3 + 0)
impl<T: NonPos> AddPeano<Zero> for Pred<T> {
    type Output = Pred<T>;
}
/// Adding positive numbers to negative numbers (e.g. -2 + 3)
impl<T: NonPos + AddPeano<Rhs>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Pred<T> {
    type Output = <T as AddPeano<Rhs>>::Output;
}

pub trait Negate {
    type Output;
}
impl Negate for Zero {
    type Output = Zero;
}
impl<T: NonNeg + Negate> Negate for Succ<T> {
    type Output = Pred<<T as Negate>::Output>;
}
impl<T: NonPos + Negate> Negate for Pred<T> {
    type Output = Succ<<T as Negate>::Output>;
}

pub trait SubPeano<Rhs> {
    type Output;
}

/// Subtracting from zero
impl<Rhs: Peano + Negate> SubPeano<Rhs> for Zero {
    type Output = <Rhs as Negate>::Output;
}

/// Subtracting positive numbers from positive numbers (e.g. 3 - 4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Succ<T> {
    type Output = <T as SubPeano<Rhs>>::Output;
}
/// Subtracting zero from positive numbers (e.g. 3 - 0)
impl<T: NonNeg> SubPeano<Zero> for Succ<T> {
    type Output = Succ<T>;
}
/// Subtracting negative numbers from positive numbers (e.g. 3 - -4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Succ<T> {
    type Output = Succ<Succ<<T as SubPeano<Rhs>>::Output>>;
}

/// Subtracting positive numbers from negative numbers (e.g. -3 - 4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Pred<T> {
    type Output = Pred<Pred<<T as SubPeano<Rhs>>::Output>>;
}
/// Subtracting zero from negative numbers (e.g. -3 - 0)
impl<T: NonPos> SubPeano<Zero> for Pred<T> {
    type Output = Pred<T>;
}
/// Subtracting negative numbers from negative numbers (e.g. -3 - -4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Pred<T> {
    type Output = <T as SubPeano<Rhs>>::Output;
}

pub trait MulPeano<Rhs> {
    type Output;
}

/// Multiplying zero by things (e.g. 0 * 7)
impl<Rhs: Peano> MulPeano<Rhs> for Zero {
    type Output = Zero;
}

/// Multiplying a positive integer by an arbitrary integer (e.g. 2 * N)
impl<T, Rhs> MulPeano<Rhs> for Succ<T>
    where T: NonNeg + MulPeano<Rhs>, Rhs: AddPeano<<T as MulPeano<Rhs>>::Output> {
        type Output = <Rhs as AddPeano<<T as MulPeano<Rhs>>::Output>>::Output;
}

/// Multiplying a negative integer by an arbitrary integer (e.g. -2 * N)
impl<T, Rhs> MulPeano<Rhs> for Pred<T>
    where T: NonPos + MulPeano<Rhs>, Rhs: Peano, <T as MulPeano<Rhs>>::Output: SubPeano<Rhs> {
        type Output = <<T as MulPeano<Rhs>>::Output as SubPeano<Rhs>>::Output;
}

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
}
