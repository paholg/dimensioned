
pub trait Dimension { }

pub trait Dimensionless: Dimension {}

macro_rules! impl_unary {
    ($Type:ty, $Trait:ident, $fun:ident) => (
        impl $Trait for $Type {
            type Output = $Type;
            fn $fun(self) -> Self::Output { self.$fun() }
        }
    );
}

use reexported::fmt;
pub trait FmtDim: Dimension {
    /// Gives a human friendly representation of a `Dimension` type.
    fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
}


/// `Recip` is used for implementing a `recip()` member for types that are not preserved under reciprocal.
pub trait Recip {
    type Output;
    fn recip(self) -> Self::Output;
}

impl_unary!(f32, Recip, recip);
impl_unary!(f64, Recip, recip);


/// `Pow<Base>` is used for implementing general integer powers for types that are not preserved
/// under exponentiation.
///
/// It uses type numbers to specify the degree.
///
/// The syntax is a little bit weird and may be subject to change.
pub trait Pow<Base> {
    type Output;
    fn pow(base: Base) -> Self::Output;
}

use typenum::Integer;
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


/// `Root<Radicand>` is used for implementing general integer roots for types that aren't preserved
/// under root.
///
/// It uses type numbers to specify the degree.
///
/// The syntax is a little bit weird and may be subject to change.
pub trait Root<Radicand> {
    type Output;
    fn root(radicand: Radicand) -> Self::Output;
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


/// `Sqrt` provides a `sqrt` member function for types that are not preserved under square root.
///
/// It is automagically implemented for all types `T` for which `P2::Root<T>` is implemented.
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

use typenum::P2;
impl<T> Sqrt for T where P2: Root<T> {
    type Output = <P2 as Root<T>>::Output;
    fn sqrt(self) -> Self::Output {
        P2::root(self)
    }
}


/// `Sqrt` provides a `sqrt` member function for types that are not preserved under square root.
///
/// It is automagically implemented for all types `T` for which `P3::Root<T>` is implemented.
pub trait Cbrt {
    type Output;
    fn cbrt(self) -> Self::Output;
}

use typenum::P3;
impl<T> Cbrt for T where P3: Root<T> {
    type Output = <P3 as Root<T>>::Output;
    fn cbrt(self) -> Self::Output {
        P3::root(self)
    }
}
