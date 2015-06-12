/*!
Peano numbers allow us to do arithmetic at compile time using Rust's type system.

This module should not require much (if any) direct interaction from users of **dimensioned**.
*/
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// The type corresponding to the Peano number 0.
#[derive(Copy, Clone)]
pub struct Zero;

/// For any non-negative Peano number `N`, we define its successor, `Succ<N>`.
///
/// This gives us Peano numbers up to 63, as Rust only supports 64 levels of embedded structs.
#[derive(Copy, Clone)]
pub struct Succ<N: NonNeg> {
    _marker: PhantomData<N>
}
/// For any non-positive Peano number `N`, we define its predecessor, `Pred<N>`.
///
/// This gives us Peano numbers down to -63, as Rust only supports 64 levels of embedded structs.
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
/// All numbers of the form `Pred<N>` and `Zero` will belong to the trait **NonPos**.
pub trait NonPos: Peano {}

impl NonZero for .. {}
impl NonNeg for .. {}
impl NonPos for .. {}

impl Peano for Zero {}
impl !NonZero for Zero {}
impl NonNeg for Zero {}
impl NonPos for Zero {}


impl<N: NonNeg> Peano for Succ<N> {}
impl<N: NonNeg> NonNeg for Succ<N> {}
impl<N: NonNeg> !NonPos for Succ<N> {}
impl<N: NonNeg> NonZero for Succ<N> {}

impl<N: NonPos> Peano for Pred<N> {}
impl<N: NonPos> !NonNeg for Pred<N> {}
impl<N: NonPos> NonPos for Pred<N> {}
impl<N: NonPos> NonZero for Pred<N> {}

// Add
// -------------------------------------------------------------------------------------
/// Add to Zero (e.g. 0 + 3)
impl<Rhs> Add<Rhs> for Zero where Rhs: Peano {
    type Output = Rhs;
    #[allow(unused_variables)]
    fn add(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Add non-negative numbers to positive numbers (e.g. 2 + 3, 2 + 0)
impl<Lhs, Rhs> Add<Rhs> for Succ<Lhs> where Lhs: NonNeg + Add<Rhs>, Rhs: NonNeg, <Lhs as Add<Rhs>>::Output: NonNeg {
    type Output = Succ<<Lhs as Add<Rhs>>::Output>;
    #[allow(unused_variables)]
    fn add(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Add non-positive numbers to negative numbers (e.g. -2 + -3, -2 + 0)
impl<Lhs, Rhs> Add<Rhs> for Pred<Lhs> where Lhs: NonPos + Add<Rhs>, Rhs: NonPos, <Lhs as Add<Rhs>>::Output: NonPos {
    type Output = Pred<<Lhs as Add<Rhs>>::Output>;
    #[allow(unused_variables)]
    fn add(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Add positive and negative numbers (e.g. -2 + 3)
impl<Lhs, Rhs> Add<Succ<Rhs>> for Pred<Lhs> where Lhs: NonPos + Add<Rhs>, Rhs: NonNeg {
    type Output = <Lhs as Add<Rhs>>::Output;
    #[allow(unused_variables)]
    fn add(self, rhs: Succ<Rhs>) -> Self::Output { unreachable!() }
}
/// Add negative and positive numbers (e.g. 2 + -3)
impl<Lhs, Rhs> Add<Pred<Rhs>> for Succ<Lhs> where Lhs: NonNeg + Add<Rhs>, Rhs: NonPos {
    type Output = <Lhs as Add<Rhs>>::Output;
    #[allow(unused_variables)]
    fn add(self, rhs: Pred<Rhs>) -> Self::Output { unreachable!() }
}

// Neg
// -------------------------------------------------------------------------------------
/// The negation of Zero is itself.
impl Neg for Zero {
    type Output = Zero;
    fn neg(self) -> Self::Output { unreachable!() }
}
/// Negate positive numbers (e.g. 5 -> -5)
impl<N> Neg for Succ<N> where N: NonNeg + Neg, <N as Neg>::Output: NonPos {
    type Output = Pred<<N as Neg>::Output>;
    fn neg(self) -> Self::Output { unreachable!() }
}
/// Negate negative numbers (e.g. -5 -> 5)
impl<N> Neg for Pred<N> where N: NonPos + Neg, <N as Neg>::Output: NonNeg {
    type Output = Succ<<N as Neg>::Output>;
    fn neg(self) -> Self::Output { unreachable!() }
}

// Sub
// -------------------------------------------------------------------------------------
/// Subtract from Zero (e.g. 0 - 4)
impl<Rhs> Sub<Rhs> for Zero where Rhs: Neg{
    type Output = <Rhs as Neg>::Output;
    #[allow(unused_variables)]
    fn sub(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Subtract Zero from positive numbers (e.g. 2 - 0)
impl<Lhs> Sub<Zero> for Succ<Lhs> where Lhs: NonNeg {
    type Output = Succ<Lhs>;
    #[allow(unused_variables)]
    fn sub(self, rhs: Zero) -> Self::Output { unreachable!() }
}
/// Subtract Zero from negative numbers (e.g. -2 - 0)
impl<Lhs> Sub<Zero> for Pred<Lhs> where Lhs: NonPos {
    type Output = Pred<Lhs>;
    #[allow(unused_variables)]
    fn sub(self, rhs: Zero) -> Self::Output { unreachable!() }
}
/// Subtract positive numbers from positive numbers (e.g. 3 - 4)
impl<Lhs, Rhs> Sub<Succ<Rhs>> for Succ<Lhs> where Lhs: NonNeg + Sub<Rhs>, Rhs: NonNeg {
    type Output = <Lhs as Sub<Rhs>>::Output;
    #[allow(unused_variables)]
    fn sub(self, rhs: Succ<Rhs>) -> Self::Output { unreachable!() }
}
/// Subtract negative numbers from negative numbers (e.g. -3 - -4)
impl<Lhs, Rhs> Sub<Pred<Rhs>> for Pred<Lhs> where Lhs: NonPos + Sub<Rhs>, Rhs: NonPos {
    type Output = <Lhs as Sub<Rhs>>::Output;
    #[allow(unused_variables)]
    fn sub(self, rhs: Pred<Rhs>) -> Self::Output { unreachable!() }
}
/// Subtract negative numbers from positive numbers (e.g. 3 - -4)
impl<Lhs, Rhs> Sub<Pred<Rhs>> for Succ<Lhs> where Lhs: NonNeg + Sub<Rhs>, Rhs: NonPos, <Lhs as Sub<Rhs>>::Output: NonNeg {
    type Output = Succ<Succ<<Lhs as Sub<Rhs>>::Output>>;
    #[allow(unused_variables)]
    fn sub(self, rhs: Pred<Rhs>) -> Self::Output { unreachable!() }
}
/// Subtract positive numbers from negative numbers (e.g. -3 - 4)
impl<Lhs, Rhs> Sub<Succ<Rhs>> for Pred<Lhs> where Lhs: NonPos + Sub<Rhs>, Rhs: NonNeg, <Lhs as Sub<Rhs>>::Output: NonPos {
    type Output = Pred<Pred<<Lhs as Sub<Rhs>>::Output>>;
    #[allow(unused_variables)]
    fn sub(self, rhs: Succ<Rhs>) -> Self::Output { unreachable!() }
}

// Mul
// -------------------------------------------------------------------------------------
/// Multiply Zero by peano numbers (e.g. 0 * 7)
impl<Rhs> Mul<Rhs> for Zero where Rhs: Peano {
    type Output = Zero;
    #[allow(unused_variables)]
    fn mul(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Multiply positive numbers by integers (e.g. 2 * N)
impl<Lhs, Rhs> Mul<Rhs> for Succ<Lhs> where Lhs: NonNeg + Mul<Rhs>, Rhs: Add<<Lhs as Mul<Rhs>>::Output> {
    type Output = <Rhs as Add<<Lhs as Mul<Rhs>>::Output>>::Output;
    #[allow(unused_variables)]
    fn mul(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Multiply negative numbers by integers (e.g. -2 * N)
impl<Lhs, Rhs> Mul<Rhs> for Pred<Lhs> where Lhs: NonPos + Mul<Rhs>, Rhs: Peano, <Lhs as Mul<Rhs>>::Output: Sub<Rhs> {
    type Output = <<Lhs as Mul<Rhs>>::Output as Sub<Rhs>>::Output;
    #[allow(unused_variables)]
    fn mul(self, rhs: Rhs) -> Self::Output { unreachable!() }
}

// Div
// -------------------------------------------------------------------------------------
/// Divide Zero by a number (e.g. 0 / 5)
impl<Rhs> Div<Rhs> for Zero where Rhs: Peano {
    type Output = Zero;
    #[allow(unused_variables)]
    fn div(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Divide a positive number by a non-zero number (e.g. 4 / 2, 4 / -2).
/// Only defined for numbers that are evenly divisible.
impl<Lhs, Rhs> Div<Rhs> for Succ<Lhs> where Lhs: NonNeg, Succ<Lhs>: DivPrivate<Rhs> {
    type Output = <Succ<Lhs> as DivPrivate<Rhs>>::Output;
    #[allow(unused_variables)]
    fn div(self, rhs: Rhs) -> Self::Output { unreachable!() }
}
/// Divide a negative number by a non-zero number (e.g. -4 / 2, -4 / -2).
/// Only defined for numbers that are evenly divisible.
impl<Lhs, Rhs> Div<Rhs> for Pred<Lhs> where Lhs: NonPos + Neg, Rhs: Neg, Succ<<Lhs as Neg>::Output>: DivPrivate<<Rhs as Neg>::Output>, <Lhs as Neg>::Output: NonNeg {
    type Output = <<Pred<Lhs> as Neg>::Output as DivPrivate<<Rhs as Neg>::Output>>::Output;
    #[allow(unused_variables)]
    fn div(self, rhs: Rhs) -> Self::Output { unreachable!() }
}


// DivPrivate only supports positive numerators. As division is implemented via repeated
// subtraction, we can be sure to only support division between divisible numbers by terminating when the numerator is Zero and not allowing it to become negative. We can support negative numerators by mapping e.g. -4 / 2 -> 4 / -2, which is what is done in `Div`
trait DivPrivate<Rhs>: Peano {
    type Output;
}
impl<Rhs: NonZero> DivPrivate<Rhs> for Zero {
    type Output = Zero;
}
// Dividing a positive integer by a positive integer (e.g. 4 / 2)
impl<Lhs, Rhs> DivPrivate<Succ<Rhs>> for Succ<Lhs>
    where Lhs: NonNeg, Succ<Lhs>: DivPrivate<Succ<Rhs>> + Sub<Succ<Rhs>>, Rhs: NonNeg {
        // Lhs / Rhs = 1 + (Lhs - Rhs) / Rhs
        type Output = <One as Add<<<Succ<Lhs> as Sub<Succ<Rhs>>>::Output as DivPrivate<Succ<Rhs>>>::Output>>::Output;
}
// Dividing a positive integer by a negative integer (e.g. 4 / -2)
impl<Lhs, Rhs> DivPrivate<Pred<Rhs>> for Succ<Lhs>
    where Lhs: NonNeg, Succ<Lhs>: DivPrivate<Pred<Rhs>> + Add<Pred<Rhs>>, Rhs: NonPos {
        // Lhs / Rhs = -1 + (Lhs + Rhs) / Rhs
        type Output = <NegOne as Add<<<Succ<Lhs> as Add<Pred<Rhs>>>::Output as DivPrivate<Pred<Rhs>>>::Output>>::Output;
    }

/// `Same` is used to ensure that two types are the same. Its `Output` should be that type.
/// # Example:
/// ```
/// use dimensioned::peano::{Succ, One, Two, Same, ToInt};
///
/// assert_eq!(2, <<Succ<One> as Same<Two>>::Output as ToInt>::to_int());
/// ```
pub trait Same<Rhs = Self> {
    type Output;
}
impl<N> Same<N> for N where N: Peano {
    type Output = N;
}



// ToInt
// -------------------------------------------------------------------------------------

/// Convert a Peano number to the integer it represents.
///
/// Once Rust implements associated constants, an associated constant will replace the function `to_int()`.
/// # Example:
/// ```
/// use dimensioned::peano::{Two, ToInt};
///
/// assert_eq!(2, <Two as ToInt>::to_int());
/// ```
pub trait ToInt: Peano {
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<N: NonNeg + ToInt> ToInt for Succ<N> {
    fn to_int() -> i32 { 1 + N::to_int() }
}
impl<N: NonPos + ToInt> ToInt for Pred<N> {
    fn to_int() -> i32 { -1 + N::to_int() }
}
