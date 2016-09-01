/*!
This module allows dimensioned to be very flexible. It creates the `Quantity<D, V>` type,
which is the type that will be used for all dimensioned objects. It then implements as
many traits from `std` as generically as possible.

Among the included traits in **dimensioned**, there are a few that are used solely to
aid in generic programming and should not be implemented for anything outside this
module. They are `Dimension`, `Dimensionless`, and `QuantityToString`.
 */

use {Same, Integer, P2, P3};

use std::marker::PhantomData;

use std::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, FnOnce, Not, Rem, Shl, Shr};
use num::traits::{Float, FromPrimitive, ToPrimitive, NumCast};
use std::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};
use std::fmt;

/**
All types created for a unit system will implement this trait. No other types should
implement it. The struct `Quantity<D, V>` requires that `D` implement `Dimension`.
 */
pub trait Dimension {}

/**
The only types that implement this trait are the `Unitless` types that exist in each
unit system. It allows more flexibility when handling specifically objects without
dimension.

No other types should implement it.
*/
pub trait Dimensionless: Dimension {}

/**
This trait allows human-friendly printing of dimensioned objects. It is used to
implement the traits in **std::fmt**.

All types created for a unit system will implement this trait. No other types should
implement it.
*/
pub trait QuantityToString: Dimension {
    /// Gives a human friendly `String` representation of a `Dimension` type.
    fn to_string() -> String;
}

/// This is the primary struct that users of this library will interact with.
pub struct Quantity<D, V>(pub V, pub PhantomData<D>);

use std::clone::Clone;
use std::marker::Copy;
impl<D, V: Clone> Clone for Quantity<D, V> {
    fn clone(&self) -> Self { Quantity::new(self.0.clone()) }
}
impl<D, V: Copy> Copy for Quantity<D, V> {}

impl<D, V> Quantity<D, V> {
    /**
    Construct a new `Quantity` object.

    It is recommened to use this only where necessary, and to generally use the
    constants that ship with unit systems to create `Quantity` objects.

    # Example
    ```
    use dimensioned::Quantity;
    use dimensioned::si::{m, Meter};

    let x: Quantity<Meter, f64> = Quantity::new(3.0);
    let y = 3.0*m;
    assert_eq!(x, y);
    ```

     */
    pub const fn new(v: V) -> Quantity<D, V> {
        Quantity(v, PhantomData)
    }
    /**
    Map a `Quantity<D, V>` to `Quantity<D, O>` by applying function `f` to the contained value
    # Example
    ```
    # extern crate dimensioned;
    # extern crate num;

    use num::traits::Float;
    use dimensioned::si::m;

    # fn main() {
    let x = 3.5*m;
    assert_eq!(3.0*m, x.map(Float::trunc) );
    # }
    ```
     */
    pub fn map<O, F: FnOnce(V) -> O>(self, f: F) -> Quantity<D, O> {
        Quantity(f(self.0), PhantomData)
    }
}

/**
This trait is implemented by default for everything that is not Quantity<D, V>. It allows a
 greater level of generic operator overloading than would be possible otherwise.
*/
#[doc(hidden)]
pub trait NotQuantity {}
impl NotQuantity for .. {}
impl<D, V> !NotQuantity for Quantity<D, V> {}

/// **Sqrt** is used for implementing a `sqrt()` member for types that don't `impl Float`.
pub trait Sqrt {
    #[allow(missing_docs)]
    type Output;
    /**
    Take a square root.
    # Example
    ```
    use dimensioned::si::m;
    use dimensioned::Sqrt;

    let x = 2.0*m;
    let y = 4.0*m*m;
    assert_eq!(x, y.sqrt());
    ```
     */
    fn sqrt(self) -> Self::Output;
}

impl<D, V> Sqrt for Quantity<D, V> where D: Root<P2>, V: Float, <D as Root<P2>>::Output: Dimension {
    type Output = Quantity<<D as Root<P2>>::Output, V>;
    #[inline]
    fn sqrt(self) -> Self::Output { Quantity( (self.0).sqrt(), PhantomData) }
}

/// **Cbrt** is used for implementing a `cbrt()` member for types that don't `impl Float`.
pub trait Cbrt {
    #[allow(missing_docs)]
    type Output;
    /**
    Take a cube root.
    # Example
    ```
    use dimensioned::si::m;
    use dimensioned::Cbrt;

    let x = 2.0*m;
    let y = 8.0*m*m*m;
    assert_eq!(x, y.cbrt());
    ```
     */
    fn cbrt(self) -> Self::Output;
}

impl<D, V> Cbrt for Quantity<D, V> where D: Root<P3>, V: Float, <D as Root<P3>>::Output: Dimension {
    type Output = Quantity<<D as Root<P3>>::Output, V>;
    #[inline]
    fn cbrt(self) -> Self::Output { Quantity( (self.0).cbrt(), PhantomData) }
}

/**
**Root<Radicand>** is used for implementing general integer roots for types that don't
`impl Float` and whose type signature changes when taking a root, such as `Quantity<D, V>`.

It uses type numbers to specify the degree.

The syntax is a little bit weird and may be subject to change.
*/
pub trait Root<Radicand> {
    #[allow(missing_docs)]
    type Output;

    /**
    # Example
    ```
    use dimensioned::si::m;
    use dimensioned::Root;
    use dimensioned::P4;

    let x = 2.0*m;
    let y = 16.0*m*m*m*m;
    assert_eq!(x, P4::root(x*x*x*x));
    ```
    */
    fn root(radicand: Radicand) -> Self::Output;
}
impl<D, V, Degree> Root<Quantity<D, V>> for Degree where D: Root<Degree>, V: Float, Degree: Integer, <D as Root<Degree>>::Output: Dimension {
    type Output = Quantity<<D as Root<Degree>>::Output, V>;
    fn root(base: Quantity<D, V>) -> Self::Output {
        let x: V = NumCast::from(Degree::to_i32()).expect("Attempted to take nth root of a Quantity<D, V>, but could not convert from i32 to V to compute n.");
        Quantity::new( (base.0).powf(x.recip()) )
    }
}

/**
**Pow<Base>** is used for implementing general integer powers for types that don't `impl
Float` and whose type signature changes when multiplying, such as `Quantity<D, V>`.

It uses type numbers to specify the degree.

The syntax is a little bit weird and may be subject to change.
*/
pub trait Pow<Base> {
    #[allow(missing_docs)]
    type Output;
    /**
    # Example
    ```
    use dimensioned::si::m;
    use dimensioned::Pow;
    use dimensioned::P3;

    let x = 2.0*m;
    let y = 8.0*m*m*m;
    assert_eq!(P3::pow(x), y);
    ```
    */
    fn pow(base: Base) -> Self::Output;
}
impl<D, V, Exp> Pow<Quantity<D, V>> for Exp where D: Pow<Exp>, V: Float, Exp: Integer, <D as Pow<Exp>>::Output: Dimension {
    type Output = Quantity<<D as Pow<Exp>>::Output, V>;
    fn pow(base: Quantity<D, V>) -> Self::Output {
        Quantity::new( (base.0).powi(Exp::to_i32()) )
    }
}

/// **Recip** is used for implementing a `recip()` member for types that don't `impl Float`.
pub trait Recip {
    #[allow(missing_docs)]
    type Output;
    /**
    # Example
    ```
    use dimensioned::si::s;
    use dimensioned::Recip;

    let x = 4.0*s;
    let y = 0.25/s;
    assert_eq!(x, y.recip())
    ```
     */
    fn recip(self) -> Self::Output;
}
impl<D, V> Recip for Quantity<D, V> where D: Recip, V: Float, <D as Recip>::Output: Dimension {
    type Output = Quantity<<D as Recip>::Output, V>;
    fn recip(self) -> Self::Output {
        Quantity::new( (self.0).recip() )
    }
}

// /**
// **Convert** provides a useful trait for allowing unit conversions. The trait `std::convert::From` can't be used because it doesn't have an associated type.

// # Example
// ```
// #[macro_use]
// extern crate dimensioned as dim;
// extern crate typenum;

// use dim::{Quantity, FromQuantity};
// use dim::cgs::{self, CGS};
// use typenum::int::Integer;
// use typenum::consts::P2;

// use std::ops::{Mul, Div};

// type Quot<A, B> = <A as Div<B>>::Output;

// mod mks {
// make_units_adv! {
// CGS, Unitless, one, f64, 1.0;
// base {
// P2, Centimeter, cm, cm;
// P2, Gram, g, g;
// P1, Second, s, s;
//     }
//     derived {
//     }
// }
// }
// use mks::MKS;

// impl<V, CM, G, S> FromQuantity<Quantity<CGS<CM, G, S>, V>> for MKS<Quot<CM, P2>, Quot<G, P2>, S>
//     where V: Mul<f64, Output=V>, CM: Integer + Div<P2>, G: Integer + Div<P2>, S: Integer,
//           Quot<CM, P2>: Integer, Quot<G, P2>: Integer,
// {
//     type Output = Quantity<Self, V>;
//     fn from_dim(origin: Quantity<CGS<CM, G, S>, V>) -> Self::Output {
//         Quantity::new( origin.0 * 0.01f64.powi(CM::to_i32()) * 0.001f64.powi(G::to_i32()) )
//     }
// }


// fn main() {
//     let speed = 35.0 * cgs::cm / cgs::s;

//     println!("I was going {}, which is {}.", speed, MKS::from_dim(speed));
// }

// ```
// */
// pub trait ConvertFrom<D, V> where Self: Sized {
//     fn convert_from(from: Quantity<D, V>) -> Quantity<Self, V>;
// }
// pub trait ConvertTo<D> {
//     type Output;
//     fn convert_to(self) -> Self::Output;
// }


/**
Used for implementing unary members of `V` for `Quantity<D, V>`

Assume you have some type `V` with a member function `fun` that takes no arguments
and has output of type `Out`.

Then, you can implement `fun` as a member for `Quantity<D, V>` with the macro invocation:

```ignore
dim_impl_unary!(Trait, fun, Op, V => Out);
```

where `Trait` is the name of the trait that you want to put this member in; it can be
any available name.

Finally, `Op` determines how the dimensions should change when calling `fun()` and is
one of:

* `Same`: Keeps the dimensions the same.
* `Mul`: Multiplies `Self` by `Self`. The same as `Pow<P2>`.
* `Div`: Divides `Self` by `Self`. The same as `Pow<Zero>`.
* `Recip`: Gives the reciprocal of `Self`.
* `Pow<N>`: Raises `Self` to the exponent `N` where `N` is a type number.
* `Root<N>`: Takes the `N`th root of `Self` where `N` is a type number.
* `Sqrt`: Takes the square root of `Self`. The same as `Root<P2>`.
* `Cbrt`: Takes the cube root of `Self`. The same as `Root<P3>`.

Note: This macro requires that `Quantity` and `Dimension` be imported.

# Example
```rust
#[macro_use]
extern crate dimensioned;

use dimensioned::Same;
use dimensioned::{Quantity, Dimension};
use dimensioned::si::m;
use std::ops::Mul;

pub struct Vector2 {
    x: f64,
    y: f64
}
impl Vector2 {
    fn norm(self) -> f64 {
        (self.x*self.x + self.y*self.y).sqrt()
    }
}
impl Mul<Vector2> for f64 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 { Vector2{x: self*rhs.x, y: self*rhs.y} }
}

dim_impl_unary!(Norm, norm, Same, Vector2 => f64);

fn main() {
    let v = m * Vector2{ x: 3.0, y: 4.0 };
    assert_eq!(5.0*m, v.norm());
}
```
*/
#[macro_export]
macro_rules! dim_impl_unary { ($Trait:ident, $fun:ident, $op:ident, $In:ty => $Out:ty) => (
    pub trait $Trait {
        type Output;
        fn $fun(self) -> Self::Output;
    }
    impl<D> $Trait for Quantity<D, $In> where D: $op, <D as $op>::Output: Dimension {
        type Output = Quantity<<D as $op>::Output, $Out>;
        fn $fun(self) -> Self::Output { Quantity::new( (self.0).$fun() ) }
    }
    );
}

/**
Used for implementing binary members of `V` for `Quantity<D, V>`.

Assume you have some type `V` with a member function `fun` that takes one argument also
of type `V` and has output type `Out`.

Then, you can implement `fun` as a member for `Quantity<D, V>` with the macro invocation:

```ignore
dim_impl_binary!(Trait, fun, Op, V => Out);
```

where `Trait` is the name of the trait that you want to put this member in; it can be
any available name.

Finally, `Op` determines how the dimensions should change when calling `fun()` and is
one of:

* `Same<RHS>`: Ensures that `Self` has the same dimensions as `RHS` but doesn't change them.
* `Mul<RHS>`: Multiplies `Self` by `RHS`.
* `Div<RHS>`: Divides `Self` by `RHS`.

Note: This macro requires that `Quantity` and `Dimension` be imported.

# Example
```rust
#[macro_use]
extern crate dimensioned;
use dimensioned::{Quantity, Dimension};
use dimensioned::si::m;
use std::ops::Mul;

pub struct Vector2 {
    x: f64,
    y: f64
}
impl Vector2 {
    fn dot(self, rhs: Vector2) -> f64 { self.x*rhs.x + self.y*rhs.y }
}
impl Mul<Vector2> for f64 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 { Vector2{ x: self*rhs.x, y: self*rhs.y } }
}

dim_impl_binary!(Dot, dot, Mul, Vector2 => f64);

fn main() {
    let v1 = m * Vector2{ x: 1.0, y: 2.0 };
    let v2 = m * Vector2{ x: 3.0, y: 5.0 };
    assert_eq!(13.0*m*m, v1.dot(v2));
}
```
*/
#[macro_export]
macro_rules! dim_impl_binary { ($Trait:ident, $fun:ident, $op:ident, $In:ty => $Out:ty) => (
    pub trait $Trait<RHS> {
        type Output;
        fn $fun(self, rhs: RHS) -> Self::Output;
    }
    impl<Dl, Dr> $Trait<Quantity<Dr, $In>> for Quantity<Dl, $In> where Dl: Dimension + $op<Dr>, Dr: Dimension, <Dl as $op<Dr>>::Output: Dimension {
        type Output = Quantity<<Dl as $op<Dr>>::Output, $Out>;
        fn $fun(self, rhs: Quantity<Dr, $In>) -> Self::Output { Quantity::new( (self.0).$fun(rhs.0) ) }
    }
    );
}

//------------------------------------------------------------------------------
// Traits from std::fmt
//------------------------------------------------------------------------------
macro_rules! dim_fmt {
    ($Trait:ident, $str:expr) => (
        impl<D, V> fmt::$Trait for Quantity<D, V> where D: QuantityToString, V: fmt::$Trait {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, $str, self.0, <D as QuantityToString>::to_string())
            }
        }
        );
}
dim_fmt!(Display, "{} {}");
dim_fmt!(Debug, "{:?} {}");
dim_fmt!(Octal, "{:o} {}");
dim_fmt!(LowerHex, "{:x} {}");
dim_fmt!(UpperHex, "{:X} {}");
dim_fmt!(Pointer, "{:p} {}");
dim_fmt!(Binary, "{:b} {}");
dim_fmt!(LowerExp, "{:e} {}");
dim_fmt!(UpperExp, "{:E} {}");
//------------------------------------------------------------------------------
// Traits from std::cmp
//------------------------------------------------------------------------------
impl<Dl, Dr, Vl, Vr> PartialEq<Quantity<Dr, Vr>> for Quantity<Dl, Vl> where Dl: Same<Dr>, Vl: PartialEq<Vr> {
    fn eq(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).eq(&(other.0))
    }
    fn ne(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).ne(&(other.0))
    }
}
impl<D: Same, V: Eq> Eq for Quantity<D, V> {}

impl<Dl, Dr, Vl, Vr> PartialOrd<Quantity<Dr, Vr>> for Quantity<Dl, Vl> where Dl: Same<Dr>, Vl: PartialOrd<Vr> {
    fn partial_cmp(&self, other: &Quantity<Dr, Vr>) -> Option<Ordering> {
        (self.0).partial_cmp(&(other.0))
    }
    fn lt(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).lt(&(other.0))
    }
    fn le(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).le(&(other.0))
    }
    fn gt(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).gt(&(other.0))
    }
    fn ge(&self, other: &Quantity<Dr, Vr>) -> bool {
        (self.0).ge(&(other.0))
    }
}
impl<D: Same, V: Ord> Ord for Quantity<D, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0).cmp(&(other.0))
    }
}
//------------------------------------------------------------------------------
// Traits from std::ops
//------------------------------------------------------------------------------

/// Multiplying!
impl<Dl, Dr, Vl, Vr> Mul<Quantity<Dr, Vr>> for Quantity<Dl, Vl>
    where Dl: Dimension + Mul<Dr>, Dr: Dimension, Vl: Mul<Vr>, <Dl as Mul<Dr>>::Output: Dimension
{
    type Output = Quantity<<Dl as Mul<Dr>>::Output, <Vl as Mul<Vr>>::Output>;

    #[inline]
    fn mul(self, rhs: Quantity<Dr, Vr>) -> Self::Output {
        Quantity::new(self.0 * rhs.0)
    }
}

/// Scalar multiplication (with scalar on RHS)!
impl<D, V, RHS> Mul<RHS> for Quantity<D, V> where V: Mul<RHS>, RHS: NotQuantity {
    type Output = Quantity<D, <V as Mul<RHS>>::Output>;
    #[inline]
    fn mul(self, rhs: RHS) -> Quantity<D, <V as Mul<RHS>>::Output> {
        Quantity(self.0 * rhs, PhantomData)
    }
}

/// Scalar multiplication (with scalar on LHS)!
macro_rules! dim_lhs_mult {
    ($t: ty) => (
        impl<D, V> Mul<Quantity<D, V>> for $t where $t: Mul<V> {
            type Output = Quantity<D, <$t as Mul<V>>::Output>;
            #[inline]
            fn mul(self, rhs: Quantity<D, V>) -> Self::Output {
                Quantity( self * rhs.0, PhantomData )
            }
        }
        );
}
dim_lhs_mult!(f32);
dim_lhs_mult!(f64);
dim_lhs_mult!(i8);
dim_lhs_mult!(i16);
dim_lhs_mult!(i32);
dim_lhs_mult!(i64);
dim_lhs_mult!(isize);
dim_lhs_mult!(u8);
dim_lhs_mult!(u16);
dim_lhs_mult!(u32);
dim_lhs_mult!(u64);
dim_lhs_mult!(usize);


/// Dividing!
impl<Dl, Dr, Vl, Vr> Div<Quantity<Dr, Vr>> for Quantity<Dl, Vl>
    where Dl: Dimension + Div<Dr>, Dr: Dimension, Vl: Div<Vr>, <Dl as Div<Dr>>::Output: Dimension
{
    type Output = Quantity<<Dl as Div<Dr>>::Output, <Vl as Div<Vr>>::Output>;
    #[inline]
    fn div(self, rhs: Quantity<Dr, Vr>) -> Quantity<<Dl as Div<Dr>>::Output, <Vl as Div<Vr>>::Output> {
        Quantity(self.0 / rhs.0, PhantomData)
    }
}

/// Scalar division (with scalar on RHS)!
impl<D, V, RHS> Div<RHS> for Quantity<D, V> where V: Div<RHS>, RHS: NotQuantity {
    type Output = Quantity<D, <V as Div<RHS>>::Output>;
    #[inline]
    fn div(self, rhs: RHS) -> Quantity<D, <V as Div<RHS>>::Output> {
        Quantity(self.0 / rhs, PhantomData)
    }
}

/// Scalar division (with scalar on LHS)!
macro_rules! dim_lhs_div {
    ($t: ty) => (
        impl<D, V> Div<Quantity<D, V>> for $t where D: Recip, <D as Recip>::Output: Dimension, $t: Div<V> {
            type Output = Quantity<<D as Recip>::Output, <$t as Div<V>>::Output>;
            #[inline]
            fn div(self, rhs: Quantity<D, V>) -> Self::Output {
                Quantity( self / rhs.0, PhantomData )
            }
        }
        );
}
dim_lhs_div!(f32);
dim_lhs_div!(f64);
dim_lhs_div!(i8);
dim_lhs_div!(i16);
dim_lhs_div!(i32);
dim_lhs_div!(i64);
dim_lhs_div!(isize);
dim_lhs_div!(u8);
dim_lhs_div!(u16);
dim_lhs_div!(u32);
dim_lhs_div!(u64);
dim_lhs_div!(usize);


// Unary operators:
macro_rules! dim_unary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<D, V> $Trait for Quantity<D, V>
            where D:$op<D>, V: $Trait, <D as $op<D>>::Output: Dimension {
                type Output = Quantity<<D as $op<D>>::Output, <V as $Trait>::Output>;
                #[inline]
                $(fn $fun(self) -> Quantity<<D as $op<D>>::Output, <V as $Trait>::Output> {
                    Quantity( (self.0).$fun(), PhantomData )
                })*
            }
        )
}
dim_unary!(Neg, Same, neg);
dim_unary!(Not, Same, not);

// Binary operators:
macro_rules! dim_binary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<Dl, Vl, Dr, Vr> $Trait<Quantity<Dr, Vr>> for Quantity<Dl, Vl>
            where Dl: Dimension + $op<Dr>, Dr: Dimension, Vl: $Trait<Vr>, <Dl as $op<Dr>>::Output: Dimension {
                type Output = Quantity<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output>;
                #[inline]
                $(fn $fun(self, rhs: Quantity<Dr, Vr>) -> Quantity<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output> {
                    Quantity( (self.0).$fun(rhs.0), PhantomData )
                })*
            }
        )
}
dim_binary!(Add, Same, add);
dim_binary!(BitAnd, Same, bitand);
dim_binary!(BitOr, Same, bitor);
dim_binary!(BitXor, Same, bitxor);
dim_binary!(Rem, Same, rem);
dim_binary!(Shl, Same, shl);
dim_binary!(Shr, Same, shr);
dim_binary!(Sub, Same, sub);

// fixme: figure this out
// impl<D, V, Idx> Index<Idx> for Quantity<D, V> where D: Dimension, V: Index<Idx>, <V as Index<Idx>>::Output: Sized {
//     type Output = Quantity<D, <V as Index<Idx>>::Output>;
//     fn index<'a>(&'a self, index: Idx) -> &'a Self::Output {
//         &Quantity::new((self.0)[index])
//     }
// }

//------------------------------------------------------------------------------
// Casting
macro_rules! cast_from {
    ($fun:ident, $prim:ident) => (
        #[inline]
        fn $fun(n: $prim) -> Option<Self> {
            match FromPrimitive::$fun(n) {
                Some(v) => Some( Quantity(v, PhantomData) ),
                None => None
            }
        }
        );
}

impl<D, V> FromPrimitive for Quantity<D, V> where V: FromPrimitive {
    cast_from!(from_i64, i64);
    cast_from!(from_u64, u64);
    cast_from!(from_isize, isize);
    cast_from!(from_i8, i8);
    cast_from!(from_i16, i16);
    cast_from!(from_i32, i32);
    cast_from!(from_usize, usize);
    cast_from!(from_u8, u8);
    cast_from!(from_u32, u32);
    cast_from!(from_f32, f32);
    cast_from!(from_f64, f64);
}

macro_rules! cast_to {
    ($fun:ident, $prim:ident) => (
        #[inline]
        fn $fun(&self) -> Option<$prim> {
            (self.0).$fun()
        }
        );
}

impl<D, V> ToPrimitive for Quantity<D, V> where V: ToPrimitive {
    cast_to!(to_i64, i64);
    cast_to!(to_u64, u64);
    cast_to!(to_isize, isize);
    cast_to!(to_i8, i8);
    cast_to!(to_i16, i16);
    cast_to!(to_i32, i32);
    cast_to!(to_usize, usize);
    cast_to!(to_u8, u8);
    cast_to!(to_u16, u16);
    cast_to!(to_u32, u32);
    cast_to!(to_f32, f32);
    cast_to!(to_f64, f64);
}

impl<D, V> NumCast for Quantity<D, V> where V: NumCast {
    #[inline]
    fn from<N>(n: N) -> Option<Self> where N: ToPrimitive {
        match NumCast::from(n) {
            Some(v) => Some(Quantity(v, PhantomData)),
            None => None
        }
    }
}

//------------------------------------------------------------------------------
// impl<D, V> ::std::num::Zero for Quantity<D, V> where V: ::std::num::Zero {
//     fn zero() -> Self {
//         Quantity::new(V::zero())
//     }
// }

//------------------------------------------------------------------------------
// DIMENSIONLESS THINGS HERE
//------------------------------------------------------------------------------
// impl<D, V> ::std::num::One for Quantity<D, V> where D: Dimensionless + Mul<D>, V: ::std::num::One + Mul {
//     fn one() -> Self {
//         Quantity::new(V::one())
//     }
// }

//------------------------------------------------------------------------------
// Num
// impl<D, V> Num for Quantity<D, V>
//     where D: Dimensionless + Same<D>, V: Float, <D as Same<D>>::Output: Dimensionless {
//         type FromStrRadixErr = Quantity<D, <V as Num>::FromStrRadixErr>;
//         fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
//             Quantity( <V as Num>::from_str_radix(str, radix));
//         }
//     }
//------------------------------------------------------------------------------
// Float
// macro_rules! dim_unary_float {
//     ($fun:ident, $returns:ty) => (
//         fn $fun(self) -> $returns { Quantity( (self.0).$fun(), PhantomData) }
//         )
// }

// impl<D, V> Float for Quantity<D, V>
//     where D: Dimensionless + Same<D>, V: Float, <D as Same<D>>::Output: Dimensionless {
//         // fn nan(self) -> Quantity<D, V> {Quantity ( (self.0).nan() )}
//         dim_unary_float!(nan, Self);
//         dim_unary_float!(infinity, Self);
//         dim_unary_float!(neg_infinity, Self);
//         dim_unary_float!(neg_zero, Self);
//         dim_unary_float!(min_value, Self);
//         //dim_unary_float!(min_positive_value, Self);
//         dim_unary_float!(max_value, Self);
//         dim_unary_float!(is_nan, bool);
//         dim_unary_float!(is_infinite, bool);
//         dim_unary_float!(is_finite, bool);
//         dim_unary_float!(is_normal, bool);
//         // dim_unary_float!(classify, FpCategory);
//         dim_unary_float!(floor, Self);
//         dim_unary_float!(ceil, Self);
//         dim_unary_float!(round, Self);
//         dim_unary_float!(trunc, Self);
//         dim_unary_float!(fract, Self);
//         dim_unary_float!(abs, Self);
//         dim_unary_float!(signum, Self);
//         dim_unary_float!(is_sign_positive, bool);
//         dim_unary_float!(is_sign_negative, bool);
//         // dim_unary_float!(mul_add, bool); BINARY

//         dim_unary_float!(recip, Self);
//         // powi
//         // powf
//         dim_unary_float!(sqrt, Self);
//         dim_unary_float!(exp, Self);
//         dim_unary_float!(exp2, Self);
//         dim_unary_float!(ln, Self);
//         dim_unary_float!(log, Self);
//         dim_unary_float!(log2, Self);
//         dim_unary_float!(log10, Self);
//         dim_unary_float!(max, Self);
//         dim_unary_float!(min, Self);
//         // abs_sub
//         dim_unary_float!(cbrt, Self);
//         dim_unary_float!(hypot, Self);
//         dim_unary_float!(sin, Self);
//         dim_unary_float!(cos, Self);
//         dim_unary_float!(tan, Self);
//         dim_unary_float!(asin, Self);
//         dim_unary_float!(acos, Self);
//         dim_unary_float!(atan, Self);
//         dim_unary_float!(atan2, Self);
//         dim_unary_float!(sin_cos, (Self, Self));
//         dim_unary_float!(exp_m1, Self);
//         dim_unary_float!(ln_1p, Self);
//         dim_unary_float!(sinh, Self);
//         dim_unary_float!(cosh, Self);
//         dim_unary_float!(tanh, Self);
//         dim_unary_float!(asinh, Self);
//         dim_unary_float!(acosh, Self);
//         dim_unary_float!(atanh, Self);
//         dim_unary_float!(integer_decode, (u64, i16, i8));

//     }


