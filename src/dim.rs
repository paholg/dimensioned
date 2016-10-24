//! This module allows dimensioned to be very flexible. It creates the `Dim<D, V>` type,
//! which is the type that will be used for all dimensioned objects. It then implements as
//! many traits from `std` as generically as possible.
//!
//! Among the included traits in **dimensioned**, there are a few that are used solely to
//! aid in generic programming and should not be implemented for anything outside this
//! module. They are `Dimension`, `Dimensionless`, and `DimToString`.
//!

use {Same, Integer, P2, P3};

use reexported::marker::PhantomData;

use reexported::ops::{Add, Sub, Mul, Div, Neg, BitAnd, BitOr, BitXor, FnOnce, Not, Rem, Shl, Shr};
use reexported::cmp::{Eq, PartialEq, Ord, PartialOrd, Ordering};
use reexported::fmt;

#[allow(unused_imports)]
// needed for some reason
#[cfg(not(feature="std"))]
use core::num::Float;

/**
All types created for a unit system will implement this trait. No other types should
implement it. The struct `Dim<D, V>` requires that `D` implement `Dimension`.
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
implement the traits in `std::fmt`.

All types created for a unit system will implement this trait. No other types should
implement it.
*/
pub trait FmtDim: Dimension {
    /// Gives a human friendly representation of a `Dimension` type.
    fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
}

/// This is the primary struct that users of this library will interact with.
pub struct Dim<D, V>(pub V, pub PhantomData<D>);

use reexported::clone::Clone;
use reexported::marker::Copy;
impl<D, V: Clone> Clone for Dim<D, V> {
    fn clone(&self) -> Self {
        Dim::new(self.0.clone())
    }
}
impl<D, V: Copy> Copy for Dim<D, V> {}

impl<D, V> Dim<D, V> {
    /**
    Construct a new `Dim` object.

    It is recommened to use this only where necessary, and to generally use the
    constants that ship with unit systems to create `Dim` objects.

    # Example
    ```
    use dimensioned::si::{m, Meter};

    let x = Meter::new(3.0);
    let y = 3.0*m;
    assert_eq!(x, y);
    ```

     */
    pub fn new(v: V) -> Dim<D, V> {
        Dim(v, PhantomData)
    }
    /**
    Map a `Dim<D, V>` to `Dim<D, O>` by applying function `f` to the contained value
    # Example
    ```
    # extern crate dimensioned;

    use dimensioned::si::m;

    # fn main() {
    let x = 4.0*m;
    assert_eq!(2.0*m, x.map(|x| x.sqrt()) );
    # }
    ```
     */
    pub fn map<O, F: FnOnce(V) -> O>(self, f: F) -> Dim<D, O> {
        Dim(f(self.0), PhantomData)
    }
}

/**
This trait is implemented by default for everything that is not Dim<D, V>. It allows a
 greater level of generic operator overloading than would be possible otherwise.
*/
#[doc(hidden)]
#[cfg(feature = "nightly")]
pub trait NotDim {}
#[cfg(feature = "nightly")]
impl NotDim for .. {}
#[cfg(feature = "nightly")]
impl<D, V> !NotDim for Dim<D, V> {}

/// **Sqrt** provides a `sqrt` member function.
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

impl<D, V> Sqrt for Dim<D, V>
    where D: Root<P2>,
          V: Sqrt,
          <D as Root<P2>>::Output: Dimension
{
    type Output = Dim<<D as Root<P2>>::Output, <V as Sqrt>::Output>;
    #[inline]
    fn sqrt(self) -> Self::Output {
        Dim((self.0).sqrt(), PhantomData)
    }
}

macro_rules! impl_sqrt {
    ($t: ty) => (
        impl<D> Sqrt for Dim<D, $t> where D: Root<P2>, <D as Root<P2>>::Output: Dimension {
            type Output = Dim<<D as Root<P2>>::Output, $t>;
            #[inline]
            fn sqrt(self) -> Self::Output { Dim( (self.0).sqrt(), PhantomData) }
        }
    );
}

#[cfg(feature="std")]
impl_sqrt!(f32);
#[cfg(feature="std")]
impl_sqrt!(f64);

#[cfg(not(feature="std"))]
impl Sqrt for f32 {
    type Output = f32;
    fn sqrt(self) -> f32 {
        if self < 0.0 {
            ::core::f32::NAN
        } else {
            unsafe { ::core::intrinsics::sqrtf32(self) }
        }
    }
}

#[cfg(not(feature="std"))]
impl Sqrt for f64 {
    type Output = f64;
    fn sqrt(self) -> f64 {
        if self < 0.0 {
            ::core::f64::NAN
        } else {
            unsafe { ::core::intrinsics::sqrtf64(self) }
        }
    }
}

/// **Cbrt** provides a `cbrt` member function.
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

impl<D, V> Cbrt for Dim<D, V>
    where D: Root<P3>,
          V: Cbrt,
          <D as Root<P3>>::Output: Dimension
{
    type Output = Dim<<D as Root<P3>>::Output, <V as Cbrt>::Output>;
    #[inline]
    fn cbrt(self) -> Self::Output {
        Dim((self.0).cbrt(), PhantomData)
    }
}

macro_rules! impl_cbrt {
    ($t: ty) => (
        impl<D> Cbrt for Dim<D, $t> where D: Root<P3>, <D as Root<P3>>::Output: Dimension {
            type Output = Dim<<D as Root<P3>>::Output, $t>;
            #[inline]
            #[cfg(feature="std")]
            fn cbrt(self) -> Self::Output { Dim( (self.0).cbrt(), PhantomData) }
            #[inline]
            #[cfg(not(feature="std"))]
            fn cbrt(self) -> Self::Output { P3::root(self) }
        }
    );
}


impl_cbrt!(f32);
impl_cbrt!(f64);

/**
**Root<Radicand>** is used for implementing general integer roots for types that don't
`impl Float` and whose type signature changes when taking a root, such as `Dim<D, V>`.

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

impl<D, V, Index> Root<Dim<D, V>> for Index
    where D: Root<Index>,
          Index: Integer + Root<V>
{
    type Output = Dim<<D as Root<Index>>::Output, <Index as Root<V>>::Output>;
    fn root(radicand: Dim<D, V>) -> Self::Output {
        Dim::new(Index::root(radicand.0))
    }
}

macro_rules! impl_root {
    ($t: ty, $f: ident) => (
        impl<Index: Integer> Root<$t> for Index {
            type Output = $t;
            #[cfg(feature = "std")]
            fn root(radicand: $t) -> Self::Output {
                let exp = (Index::to_i32() as $t).recip();
                radicand.powf(exp)
            }
            #[cfg(not(feature = "std"))]
            fn root(radicand: $t) -> Self::Output {
                let exp = (Index::to_i32() as $t).recip();
                unsafe { ::core::intrinsics::$f(radicand, exp) }
            }
        }
    );
}

impl_root!(f32, powf32);
impl_root!(f64, powf64);

/// **Pow<Base>** is used for implementing general integer powers for types that don't `impl
/// Float` and whose type signature changes when multiplying, such as `Dim<D, V>`.
///
/// It uses type numbers to specify the degree.
///
/// The syntax is a little bit weird and may be subject to change.
pub trait Pow<Base> {
    #[allow(missing_docs)]
    type Output;
    /// #[allow(missing_docs)]
    /// type Output;
    /// # Example
    /// ```
    /// use dimensioned::si::m;
    /// use dimensioned::Pow;
    /// use dimensioned::P3;
    ///
    /// let x = 2.0*m;
    /// let y = 8.0*m*m*m;
    /// assert_eq!(P3::pow(x), y);
    /// ```
    fn pow(base: Base) -> Self::Output;
}

impl<D, V, Exp> Pow<Dim<D, V>> for Exp
    where D: Pow<Exp>,
          Exp: Integer + Pow<V>
{
    type Output = Dim<<D as Pow<Exp>>::Output, <Exp as Pow<V>>::Output>;
    fn pow(base: Dim<D, V>) -> Self::Output {
        Dim::new(Exp::pow(base.0))
    }
}

macro_rules! impl_pow {
    ($t: ty) => (
        impl<Exp: Integer> Pow<$t> for Exp {
            type Output = $t;
            fn pow(base: $t) -> Self::Output {
                base.powi(Exp::to_i32())
            }
        }
    );
}

impl_pow!(f32);
impl_pow!(f64);

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
impl<D, V> Recip for Dim<D, V>
    where D: Recip,
          V: Recip,
          <D as Recip>::Output: Dimension
{
    type Output = Dim<<D as Recip>::Output, <V as Recip>::Output>;
    fn recip(self) -> Self::Output {
        Dim::new((self.0).recip())
    }
}

macro_rules! impl_recip {
    ($t: ty) => (
        impl<D> Recip for Dim<D, $t> where D: Dimension + Recip, <D as Recip>::Output: Dimension {
            type Output = Dim<<D as Recip>::Output, $t>;
            fn recip(self) -> Self::Output {
                Dim::new( (self.0).recip() )
            }
        }
    );
}

impl_recip!(f32);
impl_recip!(f64);


// /** **Convert** provides a useful trait for allowing unit conversions. The trait
// `std::convert::From` can't be used because it doesn't have an associated type.

// # Example
// ```
// #[macro_use]
// extern crate dimensioned as dim;
// extern crate typenum;

// use dim::{Dim, FromDim};
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

// impl<V, CM, G, S> FromDim<Dim<CGS<CM, G, S>, V>> for MKS<Quot<CM, P2>, Quot<G, P2>, S>
//     where V: Mul<f64, Output=V>, CM: Integer + Div<P2>, G: Integer + Div<P2>, S: Integer,
//           Quot<CM, P2>: Integer, Quot<G, P2>: Integer,
// {
//     type Output = Dim<Self, V>;
//     fn from_dim(origin: Dim<CGS<CM, G, S>, V>) -> Self::Output {
//         Dim::new( origin.0 * 0.01f64.powi(CM::to_i32()) * 0.001f64.powi(G::to_i32()) )
//     }
// }


// fn main() {
//     let speed = 35.0 * cgs::cm / cgs::s;

//     println!("I was going {}, which is {}.", speed, MKS::from_dim(speed));
// }

// ```
// */
// pub trait ConvertFrom<D, V> where Self: Sized {
//     fn convert_from(from: Dim<D, V>) -> Dim<Self, V>;
// }
// pub trait ConvertTo<D> {
//     type Output;
//     fn convert_to(self) -> Self::Output;
// }

// fixme: re-enable test when nightly flag is not needed for multiplication
/**
Used for implementing unary members of `V` for `Dim<D, V>`

Assume you have some type `V` with a member function `fun` that takes no arguments
and has output of type `Out`.

Then, you can implement `fun` as a member for `Dim<D, V>` with the macro invocation:

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

Note: This macro requires that `Dim` and `Dimension` be imported.

# Example
```ignore
#[macro_use]
extern crate dimensioned;

use dimensioned::Same;
use dimensioned::{Dim, Dimension};
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
    impl<D> $Trait for Dim<D, $In> where D: $op, <D as $op>::Output: Dimension {
        type Output = Dim<<D as $op>::Output, $Out>;
        fn $fun(self) -> Self::Output { Dim::new( (self.0).$fun() ) }
    }
    );
}
// fixme: re-enable test when nightly flag is not needed for multiplication
/**
Used for implementing binary members of `V` for `Dim<D, V>`.

Assume you have some type `V` with a member function `fun` that takes one argument also
of type `V` and has output type `Out`.

Then, you can implement `fun` as a member for `Dim<D, V>` with the macro invocation:

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

Note: This macro requires that `Dim` and `Dimension` be imported.

# Example
```ignore
#[macro_use]
extern crate dimensioned;
use dimensioned::{Dim, Dimension};
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
    impl<Dl, Dr> $Trait<Dim<Dr, $In>> for Dim<Dl, $In>
        where Dl: Dimension + $op<Dr>,
              Dr: Dimension,
              <Dl as $op<Dr>>::Output: Dimension
    {
        type Output = Dim<<Dl as $op<Dr>>::Output, $Out>;
        fn $fun(self, rhs: Dim<Dr, $In>) -> Self::Output { Dim::new( (self.0).$fun(rhs.0) ) }
    }
    );
}

// ------------------------------------------------------------------------------
// Traits from std::fmt
// ------------------------------------------------------------------------------
macro_rules! dim_fmt {
    ($Trait:ident, $str:expr) => (
        impl<D, V> fmt::$Trait for Dim<D, V> where D: FmtDim, V: fmt::$Trait {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                try!(write!(f, $str, self.0));
                try!(D::fmt(f));
                Ok(())
            }
        }
        );
}
dim_fmt!(Display, "{} ");
dim_fmt!(Debug, "{:?} ");
dim_fmt!(Octal, "{:o} ");
dim_fmt!(LowerHex, "{:x} ");
dim_fmt!(UpperHex, "{:X} ");
dim_fmt!(Pointer, "{:p} ");
dim_fmt!(Binary, "{:b} ");
dim_fmt!(LowerExp, "{:e} ");
dim_fmt!(UpperExp, "{:E} ");
// ------------------------------------------------------------------------------
// Traits from std::cmp
// ------------------------------------------------------------------------------
impl<Dl, Dr, Vl, Vr> PartialEq<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Same<Dr>,
          Vl: PartialEq<Vr>
{
    fn eq(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).eq(&(other.0))
    }
    fn ne(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).ne(&(other.0))
    }
}
impl<D: Same, V: Eq> Eq for Dim<D, V> {}

impl<Dl, Dr, Vl, Vr> PartialOrd<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Same<Dr>,
          Vl: PartialOrd<Vr>
{
    fn partial_cmp(&self, other: &Dim<Dr, Vr>) -> Option<Ordering> {
        (self.0).partial_cmp(&(other.0))
    }
    fn lt(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).lt(&(other.0))
    }
    fn le(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).le(&(other.0))
    }
    fn gt(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).gt(&(other.0))
    }
    fn ge(&self, other: &Dim<Dr, Vr>) -> bool {
        (self.0).ge(&(other.0))
    }
}
impl<D: Same, V: Ord> Ord for Dim<D, V> {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.0).cmp(&(other.0))
    }
}
// ------------------------------------------------------------------------------
// Traits from std::ops
// ------------------------------------------------------------------------------

/// Scalar multiplication with scalar on RHS
#[cfg(feature = "nightly")]
impl<D, V, RHS> Mul<RHS> for Dim<D, V>
    where V: Mul<RHS>,
          RHS: NotDim
{
    type Output = Dim<D, <V as Mul<RHS>>::Output>;
    #[inline]
    fn mul(self, rhs: RHS) -> Self::Output {
        Dim(self.0 * rhs, PhantomData)
    }
}

/// Multiplying
impl<Dl, Dr, Vl, Vr> Mul<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Dimension + Mul<Dr>,
          Dr: Dimension,
          Vl: Mul<Vr>,
          <Dl as Mul<Dr>>::Output: Dimension
{
    type Output = Dim<<Dl as Mul<Dr>>::Output, <Vl as Mul<Vr>>::Output>;
    #[inline]
    fn mul(self, rhs: Dim<Dr, Vr>) -> Self::Output {
        Dim::new(self.0 * rhs.0)
    }
}

macro_rules! dim_scalar_mult {
    ($t: ty) => (
        /// Scalar multiplication with scalar on LHS
        impl<D, V> Mul<Dim<D, V>> for $t where $t: Mul<V> {
            type Output = Dim<D, <$t as Mul<V>>::Output>;
            #[inline]
            fn mul(self, rhs: Dim<D, V>) -> Self::Output {
                Dim( self * rhs.0, PhantomData )
            }
        }

        /// Scalar multiplication with scalar on RHS
        #[cfg(not(feature = "nightly"))]
        impl<D, V> Mul<$t> for Dim<D, V> where V: Mul<$t> {
            type Output = Dim<D, <V as Mul<$t>>::Output>;
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Dim( self.0 * rhs, PhantomData )
            }
        }

    );
}

dim_scalar_mult!(f32);
dim_scalar_mult!(f64);
dim_scalar_mult!(i8);
dim_scalar_mult!(i16);
dim_scalar_mult!(i32);
dim_scalar_mult!(i64);
dim_scalar_mult!(isize);
dim_scalar_mult!(u8);
dim_scalar_mult!(u16);
dim_scalar_mult!(u32);
dim_scalar_mult!(u64);
dim_scalar_mult!(usize);


/// Dividing
impl<Dl, Dr, Vl, Vr> Div<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Dimension + Div<Dr>,
          Dr: Dimension,
          Vl: Div<Vr>,
          <Dl as Div<Dr>>::Output: Dimension
{
    type Output = Dim<<Dl as Div<Dr>>::Output, <Vl as Div<Vr>>::Output>;
    #[inline]
    fn div(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as Div<Dr>>::Output, <Vl as Div<Vr>>::Output> {
        Dim(self.0 / rhs.0, PhantomData)
    }
}

/// Scalar division with scalar on RHS
#[cfg(feature = "nightly")]
impl<D, V, RHS> Div<RHS> for Dim<D, V>
    where V: Div<RHS>,
          RHS: NotDim
{
    type Output = Dim<D, <V as Div<RHS>>::Output>;
    #[inline]
    fn div(self, rhs: RHS) -> Dim<D, <V as Div<RHS>>::Output> {
        Dim(self.0 / rhs, PhantomData)
    }
}

macro_rules! dim_scalar_div {
    ($t: ty) => (
        /// Scalar division with scalar on LHS
        impl<D, V> Div<Dim<D, V>> for $t
            where D: Recip,
                  <D as Recip>::Output: Dimension,
                  $t: Div<V>
        {
            type Output = Dim<<D as Recip>::Output, <$t as Div<V>>::Output>;
            #[inline]
            fn div(self, rhs: Dim<D, V>) -> Self::Output {
                Dim( self / rhs.0, PhantomData )
            }
        }

        /// Scalar division with scalar on RHS
        #[cfg(not(feature = "nightly"))]
        impl<D, V> Div<$t> for Dim<D, V> where V: Div<$t> {
            type Output = Dim<D, <V as Div<$t>>::Output>;
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Dim( self.0 / rhs, PhantomData )
            }
        }

        );
}
dim_scalar_div!(f32);
dim_scalar_div!(f64);
dim_scalar_div!(i8);
dim_scalar_div!(i16);
dim_scalar_div!(i32);
dim_scalar_div!(i64);
dim_scalar_div!(isize);
dim_scalar_div!(u8);
dim_scalar_div!(u16);
dim_scalar_div!(u32);
dim_scalar_div!(u64);
dim_scalar_div!(usize);


// Unary operators:
macro_rules! dim_unary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<D, V> $Trait for Dim<D, V>
            where D:$op<D>, V: $Trait, <D as $op<D>>::Output: Dimension {
                type Output = Dim<<D as $op<D>>::Output, <V as $Trait>::Output>;
                #[inline]
                $(fn $fun(self) -> Dim<<D as $op<D>>::Output, <V as $Trait>::Output> {
                    Dim( (self.0).$fun(), PhantomData )
                })*
            }
        )
}
dim_unary!(Neg, Same, neg);
dim_unary!(Not, Same, not);

// Binary operators:
macro_rules! dim_binary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<Dl, Vl, Dr, Vr> $Trait<Dim<Dr, Vr>> for Dim<Dl, Vl>
            where Dl: Dimension + $op<Dr>,
                  Dr: Dimension, Vl: $Trait<Vr>,
                  <Dl as $op<Dr>>::Output: Dimension
        {
            type Output = Dim<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output>;
            #[inline]
            $(fn $fun(self, rhs: Dim<Dr, Vr>)
                      -> Dim<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output>
              {
                  Dim( (self.0).$fun(rhs.0), PhantomData )
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
// impl<D, V, Idx> Index<Idx> for Dim<D, V>
//     where D: Dimension, V: Index<Idx>, <V as Index<Idx>>::Output: Sized
// {
//     type Output = Dim<D, <V as Index<Idx>>::Output>;
//     fn index<'a>(&'a self, index: Idx) -> &'a Self::Output {
//         &Dim::new((self.0)[index])
//     }
// }

// {

//     trait Sqr {
//         fn sqr(self) -> Self;
//     }

//     macro_rules! impl_sqr {
//         ($t:ty) => (
//             impl Sqr for $t {
//                 fn sqr(self) -> Self {
//                     self * self
//                 }
//             }
//         );
//     }
// }
