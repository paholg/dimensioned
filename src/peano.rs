/*!
Peano numbers allow us to do arithmetic at compile time using Rust's type system.

This module should not require much direct interaction from users of **dimensioned**.

The basic idea of the peano numbers is that we first define the number `Zero`. Then, we
inductively define the rest of the natural numbers. For any non-negative natural number
`N`, define its successor, `Succ<N>`, a positive number. We can now count!

Because we want all the integers and not just the natural numbers, we have to be a
little bit careful with these definitions, which is why it is specified that `N` above
must not be negative. Otherwise, when we define predecessors we could end up with
`Pred<Succ<Zero>>` which should represent the number `Zero` but is a distinct type and
would not be treated as such by the compiler.

We can now define negative numbers: For any non-positive natural number `N`, define
its predecessor, `Pred<N>`, a negative number.

By the definitions of `Pred` and `Succ`, we disallow them to be used together.

Conceptually, we now have access to all integers. In practice, we have access to
the numbers from -63 to 63 unless we use the `#![recursion_limit="N"]` lint to increase
the allowed number of embedded traits.

These numbers are used to track powers of units in **dimensioned**.

In addition to the traits created here, the traits `Add`, `Sub`, `Mul`, `Div`, and `Neg`
are all implemented for Peano numbers. Note that these traits are used here very
differently than is typical. The functions that come with them are not used at all (and
will throw an error if you try). Instead, we are using them as operators on *types*,
with the associated type acting as the result of the computation.

# Example
```
use dimensioned::peano::{P2, P3, P4, ToInt};
# use std::ops::{Add, Div};
// 2 + 3 == 5
assert_eq!( 5, <<P2 as Add<P3>>::Output as ToInt>::to_int() );

// 4 / 2 == 2
assert_eq!( 2, <<P4 as Div<P2>>::Output as ToInt>::to_int() );
```

Note that the `ToInt` trait here is only used to get an integer output; it is the only
runtime operation defined for Peano numbers, and exists primarily for debugging
purposes. What is important and generally used is the associated type `Output`.
 */
use std::marker::PhantomData;
use std::ops::{Add, Sub, Mul, Div, Neg};

/// The type corresponding to the Peano number 0.
#[derive(Copy, Clone)]
pub struct Zero;

/// For any non-negative Peano number `N`, we define its successor, `Succ<N>`.
///
/// This gives us positive Peano numbers.
#[derive(Copy, Clone)]
pub struct Succ<N: NonNeg> {
    _marker: PhantomData<N>
}
/// For any non-positive Peano number `N`, we define its predecessor, `Pred<N>`.
///
/// This gives us negative Peano numbers.
#[derive(Copy, Clone)]
pub struct Pred<N: NonPos> {
    _marker: PhantomData<N>
}


/// The Peano number +1; `P1 = Succ<Zero>;`
pub type P1 = Succ<Zero>;
/// The Peano number +2; `P2 = Succ<P1>;`
pub type P2 = Succ<P1>;
/// The Peano number +3; `P3 = Succ<P2>;`
pub type P3 = Succ<P2>;
/// The Peano number +4; `P4 = Succ<P3>;`
pub type P4 = Succ<P3>;
/// The Peano number +5; `P5 = Succ<P4>;`
pub type P5 = Succ<P4>;
/// The Peano number +6; `P6 = Succ<P5>;`
pub type P6 = Succ<P5>;
/// The Peano number +7; `P7 = Succ<P6>;`
pub type P7 = Succ<P6>;
/// The Peano number +8; `P8 = Succ<P7>;`
pub type P8 = Succ<P7>;
/// The Peano number +9; `P9 = Succ<P8>;`
pub type P9 = Succ<P8>;

/// The Peano number -1; `N1 = Pred<Zero>;`
pub type N1 = Pred<Zero>;
/// The Peano number -2; `N2 = Pred<N1>;`
pub type N2 = Pred<N1>;
/// The Peano number -3; `N3 = Pred<N2>;`
pub type N3 = Pred<N2>;
/// The Peano number -4; `N4 = Pred<N3>;`
pub type N4 = Pred<N3>;
/// The Peano number -5; `N5 = Pred<N4>;`
pub type N5 = Pred<N4>;
/// The Peano number -6; `N6 = Pred<N5>;`
pub type N6 = Pred<N5>;
/// The Peano number -7; `N7 = Pred<N6>;`
pub type N7 = Pred<N6>;
/// The Peano number -8; `N8 = Pred<N7>;`
pub type N8 = Pred<N7>;
/// The Peano number -9; `N9 = Pred<N8>;`
pub type N9 = Pred<N8>;


/// All numbers defined in this module belong to the **Peano** trait. It should not be
/// implemented for anything else.
pub trait Peano {}
/// Implemented for all Peano numbers of the form `Succ<M: NonNeg>` or `Pred<N: NonPos>`.
pub trait NonZero: Peano {}
/// Implemented for `Zero` and all numbers of the form `Succ<N: NonNeg>`.
pub trait NonNeg: Peano {}
/// Implemented for `Zero` and all numbers of the form `Pred<N: NonPos>`.
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
// subtraction, we can be sure to only support division between divisible numbers by
// terminating when the numerator is Zero and not allowing it to become negative. We can
// support negative numerators by mapping e.g. -4 / 2 to 4 / -2, which is what is done
// in the implementation of `Div`
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
        type Output = <P1 as Add<<<Succ<Lhs> as Sub<Succ<Rhs>>>::Output as DivPrivate<Succ<Rhs>>>::Output>>::Output;
}
// Dividing a positive integer by a negative integer (e.g. 4 / -2)
impl<Lhs, Rhs> DivPrivate<Pred<Rhs>> for Succ<Lhs>
    where Lhs: NonNeg, Succ<Lhs>: DivPrivate<Pred<Rhs>> + Add<Pred<Rhs>>, Rhs: NonPos {
        // Lhs / Rhs = -1 + (Lhs + Rhs) / Rhs
        type Output = <N1 as Add<<<Succ<Lhs> as Add<Pred<Rhs>>>::Output as DivPrivate<Pred<Rhs>>>::Output>>::Output;
    }

/** `Same` is used to ensure that two types are the same. Its `Output` should be that type.

# Example:
```
use dimensioned::peano::{Same, Succ, P1, P2, ToInt};

assert_eq!(2, <<Succ<P1> as Same<P2>>::Output as ToInt>::to_int());
```
*/
pub trait Same<Rhs = Self> {
    /// `Output` should always be `Self`
    type Output = Self;
}
impl<N> Same<N> for N where N: Peano {
    type Output = N;
}



// ToInt
// -------------------------------------------------------------------------------------

/**
Convert a Peano number to the integer it represents.

Once Rust implements associated constants, an associated constant will replace the
function `to_int()`.

# Example:
```
use dimensioned::peano::{P2, ToInt};

assert_eq!(2, <P2 as ToInt>::to_int());
```
*/
pub trait ToInt: Peano {
    #[allow(missing_docs)]
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
