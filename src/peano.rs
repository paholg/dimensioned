/*!
Peano numbers allow us to do arithmetic at compile time using Rust's type system.

This module creates many traits. They are all used for arithmetic between Peano numbers
and it is not recommended that you implement them for anything else.
*/
use std::marker::PhantomData;

/// The type-level number 0
#[derive(Copy, Clone)]
pub struct Zero;

/// For any number N, we define its successor. In mathematics, this and Zero are enough
/// to get all the natural numbers; in Rust, it only gets us up to 63 as that is the
/// most levels of embedded structs we can have.
///
/// We further specify that N must not be negative. This way we never have a situation
/// like `Succ<Pred<Zero>>` and all numbers are uniquely defined.
#[derive(Copy, Clone)]
pub struct Succ<N: NonNeg> {
    _marker: PhantomData<N>
}
/// For any number N that isn't a positive number, we define its
/// predecessor. Mathematically, we have now defined all of the integers. We have
/// actually defined -63 to 63.
#[derive(Copy, Clone)]
pub struct Pred<N: NonPos> {
    _marker: PhantomData<N>
}

#[allow(missing_docs)]
pub type One = Succ<Zero>;
#[allow(missing_docs)]
pub type Two = Succ<One>;
#[allow(missing_docs)]
pub type Three = Succ<Two>;
#[allow(missing_docs)]
pub type Four = Succ<Three>;
#[allow(missing_docs)]
pub type Five = Succ<Four>;
#[allow(missing_docs)]
pub type Six = Succ<Five>;
#[allow(missing_docs)]
pub type Seven = Succ<Six>;
#[allow(missing_docs)]
pub type Eight = Succ<Seven>;
#[allow(missing_docs)]
pub type Nine = Succ<Eight>;
#[allow(missing_docs)]
pub type Ten = Succ<Nine>;

#[allow(missing_docs)]
pub type NegOne = Pred<Zero>;
#[allow(missing_docs)]
pub type NegTwo = Pred<NegOne>;
#[allow(missing_docs)]
pub type NegThree = Pred<NegTwo>;
#[allow(missing_docs)]
pub type NegFour = Pred<NegThree>;
#[allow(missing_docs)]
pub type NegFive = Pred<NegFour>;
#[allow(missing_docs)]
pub type NegSix = Pred<NegFive>;
#[allow(missing_docs)]
pub type NegSeven = Pred<NegSix>;
#[allow(missing_docs)]
pub type NegEight = Pred<NegSeven>;
#[allow(missing_docs)]
pub type NegNine = Pred<NegEight>;
#[allow(missing_docs)]
pub type NegTen = Pred<NegNine>;

/// All numbers defined in this module will belong to the **Peano** trait.
pub trait Peano {}
/// All numbers of the form `Succ<M>` or `Pred<N>` will belong to the trait **NonZero**.
pub trait NonZero: Peano {}
/// All numbers of the form `Succ<N>` and `Zero` will belong to the trait **NonNeg**.
pub trait NonNeg: Peano {}
/// All numbers of the form `Pred<N>` and `Zero` will belong to the trait **NonNeg**.
pub trait NonPos: Peano {}

impl Peano for Zero {}
impl NonNeg for Zero {}
impl NonPos for Zero {}

impl<N: NonNeg> Peano for Succ<N> {}
impl<N: NonNeg> NonNeg for Succ<N> {}
impl<N: NonNeg> NonZero for Succ<N> {}

impl<N: NonPos> Peano for Pred<N> {}
impl<N: NonPos> NonPos for Pred<N> {}
impl<N: NonPos> NonZero for Pred<N> {}

/// This trait allows us to add any two Peano numbers.
pub trait AddPeano<RHS>: Peano {
    #[allow(missing_docs)]
    type Output;
}

/// Adding any number to zero (e.g. 0 + 3)
impl<RHS: Peano> AddPeano<RHS> for Zero {
    type Output = RHS;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<LHS: NonNeg + AddPeano<Succ<RHS>>, RHS: NonNeg> AddPeano<Succ<RHS>> for Succ<LHS> {
    type Output = Succ<<LHS as AddPeano<Succ<RHS>>>::Output>;
}
/// Adding zero to positive numbers (e.g. 3 + 0)
impl<LHS: NonNeg> AddPeano<Zero> for Succ<LHS> {
    type Output = Succ<LHS>;
}
/// Adding negative numbers to positive numbers (e.g. 2 + -3)
impl<LHS: NonNeg + AddPeano<RHS>, RHS: NonPos> AddPeano<Pred<RHS>> for Succ<LHS> {
    type Output = <LHS as AddPeano<RHS>>::Output;
}

/// Adding negative numbers (e.g. -1 + -2)
impl<LHS: NonPos + AddPeano<Pred<RHS>>, RHS: NonPos> AddPeano<Pred<RHS>> for Pred<LHS> {
    type Output = Pred<<LHS as AddPeano<Pred<RHS>>>::Output>;
}
/// Adding zero to negative numbers (e.g. -3 + 0)
impl<LHS: NonPos> AddPeano<Zero> for Pred<LHS> {
    type Output = Pred<LHS>;
}
/// Adding positive numbers to negative numbers (e.g. -2 + 3)
impl<LHS: NonPos + AddPeano<RHS>, RHS: NonNeg> AddPeano<Succ<RHS>> for Pred<LHS> {
    type Output = <LHS as AddPeano<RHS>>::Output;
}

/// This trait provides the negation of a number (e.g. -5 to 5)
pub trait Negate {
    #[allow(missing_docs)]
    type Output;
}
impl Negate for Zero {
    type Output = Zero;
}
impl<N: NonNeg + Negate> Negate for Succ<N> {
    type Output = Pred<<N as Negate>::Output>;
}
impl<N: NonPos + Negate> Negate for Pred<N> {
    type Output = Succ<<N as Negate>::Output>;
}


/// This trait allows us to subtract any two Peano numbers.
pub trait SubPeano<RHS>: Peano {
    #[allow(missing_docs)]
    type Output;
}

/// Subtracting from zero (e.g. 0 - 4)
impl<RHS: Peano + Negate> SubPeano<RHS> for Zero {
    type Output = <RHS as Negate>::Output;
}

/// Subtracting positive numbers from positive numbers (e.g. 3 - 4)
impl<LHS: NonNeg + SubPeano<RHS>, RHS> SubPeano<Succ<RHS>> for Succ<LHS> {
    type Output = <LHS as SubPeano<RHS>>::Output;
}
/// Subtracting zero from positive numbers (e.g. 3 - 0)
impl<LHS: NonNeg> SubPeano<Zero> for Succ<LHS> {
    type Output = Succ<LHS>;
}
/// Subtracting negative numbers from positive numbers (e.g. 3 - -4)
impl<LHS: NonNeg + SubPeano<RHS>, RHS: NonPos> SubPeano<Pred<RHS>> for Succ<LHS> {
    type Output = Succ<Succ<<LHS as SubPeano<RHS>>::Output>>;
}

/// Subtracting positive numbers from negative numbers (e.g. -3 - 4)
impl<LHS: NonPos + SubPeano<RHS>, RHS: NonNeg> SubPeano<Succ<RHS>> for Pred<LHS> {
    type Output = Pred<Pred<<LHS as SubPeano<RHS>>::Output>>;
}
/// Subtracting zero from negative numbers (e.g. -3 - 0)
impl<LHS: NonPos> SubPeano<Zero> for Pred<LHS> {
    type Output = Pred<LHS>;
}
/// Subtracting negative numbers from negative numbers (e.g. -3 - -4)
impl<LHS: NonPos + SubPeano<RHS>, RHS: NonPos> SubPeano<Pred<RHS>> for Pred<LHS> {
    type Output = <LHS as SubPeano<RHS>>::Output;
}


/// This trait allows us to multiply any two Peano numbers.
pub trait MulPeano<RHS>: Peano {
    #[allow(missing_docs)]
    type Output;
}

/// Multiplying zero by things (e.g. 0 * 7)
impl<RHS: Peano> MulPeano<RHS> for Zero {
    type Output = Zero;
}

/// Multiplying a positive integer by an arbitrary integer (e.g. 2 * N)
impl<LHS, RHS> MulPeano<RHS> for Succ<LHS>
    where LHS: NonNeg + MulPeano<RHS>, RHS: AddPeano<<LHS as MulPeano<RHS>>::Output> {
        type Output = <RHS as AddPeano<<LHS as MulPeano<RHS>>::Output>>::Output;
}

/// Multiplying a negative integer by an arbitrary integer (e.g. -2 * N)
impl<LHS, RHS> MulPeano<RHS> for Pred<LHS>
    where LHS: NonPos + MulPeano<RHS>, RHS: Peano, <LHS as MulPeano<RHS>>::Output: SubPeano<RHS> {
        type Output = <<LHS as MulPeano<RHS>>::Output as SubPeano<RHS>>::Output;
}


/// This trait allows us to divide two Peano numbers so long as the numerator is divisible by the denominator.
///
/// We are operating in a ring, so an error will be if the numerator is not divisible by
/// the denominator
pub trait DivPeano<RHS>: Peano {
    #[allow(missing_docs)]
    type Output;
}
impl<RHS> DivPeano<RHS> for Zero
    where RHS: NonZero {
        type Output = Zero;
}
impl<LHS, RHS> DivPeano<RHS> for Succ<LHS>
    where LHS: NonNeg, Succ<LHS>: DivPeanoPriv<RHS> {
        type Output = <Succ<LHS> as DivPeanoPriv<RHS>>::Output;
}
impl<LHS, RHS> DivPeano<RHS> for Pred<LHS>
    where LHS: NonPos + Negate, RHS: Negate, Succ<<LHS as Negate>::Output>: DivPeanoPriv<<RHS as Negate>::Output>    {
        type Output = <<Pred<LHS> as Negate>::Output as DivPeanoPriv<<RHS as Negate>::Output>>::Output;
}

// DivPeanoPriv only supports positive numerators. That way, it will terminate with an
// error if you ever try to divide non-divisible things. We can divide things like -4 /
// 2 by first negating both numerator and denominator, which is what DivPeano does.
trait DivPeanoPriv<RHS>: Peano {
    type Output;
}
impl<RHS: NonZero> DivPeanoPriv<RHS> for Zero {
    type Output = Zero;
}
//Dividing a positive integer by a positive integer (e.g. 4 / 2)
impl<LHS, RHS> DivPeanoPriv<Succ<RHS>> for Succ<LHS>
    where LHS: NonNeg, Succ<LHS>: DivPeanoPriv<Succ<RHS>> + SubPeano<Succ<RHS>>, RHS: NonNeg {
        // LHS / RHS = 1 + (LHS - RHS) / RHS
        type Output = <One as AddPeano<<<Succ<LHS> as SubPeano<Succ<RHS>>>::Output as DivPeanoPriv<Succ<RHS>>>::Output>>::Output;
}

// Dividing a positive integer by a negative integer (e.g. 4 / -2)
impl<LHS, RHS> DivPeanoPriv<Pred<RHS>> for Succ<LHS>
    where LHS: NonNeg, Succ<LHS>: DivPeanoPriv<Pred<RHS>> + AddPeano<Pred<RHS>>, RHS: NonPos {
        // LHS / RHS = -1 + (LHS + RHS) / RHS
        type Output = <NegOne as AddPeano<<<Succ<LHS> as AddPeano<Pred<RHS>>>::Output as DivPeanoPriv<Pred<RHS>>>::Output>>::Output;
}


/// This trait does nothing, but it forces `Self` and `RHS` to be the same peano number.
pub trait KeepPeano<RHS>: Peano {
    #[allow(missing_docs)]
    type Output;
}
/// Output = N iff both operands are type N
/// Useful for making macros to implement functions
impl<N> KeepPeano<N> for N where N: Peano {
    type Output = N;
}

/// Converts a type to the integer it represents
pub trait ToInt: Peano {
    /// This function is called with, for example, `Two::to_int();`
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<N: NonNeg + ToInt> ToInt for Succ<N> {
    fn to_int() -> i32 { 1 + <N as ToInt>::to_int() }
}
impl<N: NonPos + ToInt> ToInt for Pred<N> {
    fn to_int() -> i32 { -1 + <N as ToInt>::to_int() }
}
