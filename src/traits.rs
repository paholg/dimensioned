//! Traits that are useful.

pub trait Dimensioned<V> {
    type Array;

    fn new(val: V) -> Self;

    fn value(&self) -> &V;
}

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

/// `Recip` is used for implementing a `recip()` member for types that are not preserved under reciprocal.
pub trait Recip {
    type Output;
    fn recip(self) -> Self::Output;
}

impl_unary!(f32, Recip, recip);
impl_unary!(f64, Recip, recip);


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

use typenum::Integer;
macro_rules! impl_root {
    ($t: ty, $f: ident) => (
        impl<Index: Integer> Root<Index> for $t {
            type Output = $t;
            #[cfg(feature = "std")]
            fn root(self, _: Index) -> Self::Output {
                let exp = (Index::to_i32() as $t).recip();
                self.powf(exp)
            }

            #[cfg(not(feature = "std"))]
            fn root(self, _: Index) -> Self::Output {
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
impl<T> Sqrt for T
    where T: Root<P2>
{
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
impl<T> Cbrt for T
    where T: Root<P3>
{
    type Output = <T as Root<P3>>::Output;
    fn cbrt(self) -> Self::Output {
        self.root(P3::new())
    }
}
