pub trait Dimension<T> {}

pub trait Dimensionless {}

#[cfg(feature = "oibit")]
pub trait NotDim {}
#[cfg(feature = "oibit")]
impl NotDim for .. {}

macro_rules! impl_unary {
    ($Type:ty, $Trait:ident, $fun:ident) => (
        impl $Trait for $Type {
            type Output = $Type;
            fn $fun(self) -> Self::Output { self.$fun() }
        }
    );
}

// use reexported::fmt;
// pub trait FmtDim: Dimension {
//     /// Gives a human friendly representation of a `Dimension` type.
//     fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error>;
// }


/// `Recip` is used for implementing a `recip()` member for types that are not preserved under reciprocal.
pub trait Recip {
    type Output;
    fn recip(self) -> Self::Output;
}

impl_unary!(f32, Recip, recip);
impl_unary!(f64, Recip, recip);


/// `Pow<Exp> for Base` is used for implementing general integer powers for types that are not necessarily
/// preserved under exponentiation.
///
/// It uses instantiated type numbers to specify the degree, as you can see in the example below.
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
/// use dim::Pow;
/// use dim::typenum::P2;
///
/// # fn main() {
/// let x = 2.0.pow(P2::new());
/// let y = 4.0;
///
/// assert_eq!(x, y);
/// # }
pub trait Pow<Exp> {
    type Output;
    fn pow(self, exp: Exp) -> Self::Output;
}

use typenum::Integer;
macro_rules! impl_pow {
    ($t: ty) => (
        impl<Exp: Integer> Pow<Exp> for $t {
            type Output = $t;
            fn pow(self, exp: Exp) -> Self::Output {
                self.powi(Exp::to_i32())
            }
        }
    );
}

impl_pow!(f32);
impl_pow!(f64);

/// `Root<Index> for Radicand` is used for implementing general integer roots for types that aren't necessarily preserved under root.
///
/// It uses instantiated type numbers to specify the degree, as you can see in the example below.
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
/// use dim::Root;
/// use dim::typenum::P2;
///
/// # fn main() {
/// let x = 4.0.root(P2::new());
/// let y = 2.0;
///
/// assert_eq!(x, y);
/// # }
pub trait Root<Index> {
    type Output;
    fn root(self, idx: Index) -> Self::Output;
}

macro_rules! impl_root {
    ($t: ty, $f: ident) => (
        impl<Index: Integer> Root<Index> for $t {
            type Output = $t;
            #[cfg(feature = "std")]
            fn root(self, idx: Index) -> Self::Output {
                let exp = (Index::to_i32() as $t).recip();
                self.powf(exp)
            }

            #[cfg(not(feature = "std"))]
            fn root(self, idx: Index) -> Self::Output {
                let exp = (Index::to_i32() as $t).recip();
                unsafe { ::core::intrinsics::$f(self, exp) }
            }
        }
    );
}

impl_root!(f32, powf32);
impl_root!(f64, powf64);


/// `Sqrt` provides a `sqrt` member function for types that are not preserved under square root.
///
/// It is automatically implemented for all types `T` for which `P2::Root<T>` is implemented.
pub trait Sqrt {
    type Output;
    fn sqrt(self) -> Self::Output;
}

use typenum::P2;
impl<T> Sqrt for T where T: Root<P2> {
    type Output = <T as Root<P2>>::Output;
    fn sqrt(self) -> Self::Output {
        self.root(P2::new())
    }
}


/// `Cbrt` provides a `cbrt` member function for types that are not preserved under square root.
///
/// It is automatically implemented for all types `T` for which `P3::Root<T>` is implemented.
pub trait Cbrt {
    type Output;
    fn cbrt(self) -> Self::Output;
}

use typenum::P3;
impl<T> Cbrt for T where T: Root<P3> {
    type Output = <T as Root<P3>>::Output;
    fn cbrt(self) -> Self::Output {
        self.root(P3::new())
    }
}
