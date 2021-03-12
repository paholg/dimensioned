//! Traits for working generically with dimensioned
//!
//! Unless specified otherwise, all of these traits are implemented for the unit systems that come
//! with dimensioned and with any created by the `make_units!` macro.

/// Allows one to refer to quantities generically.
///
/// It is not recommened to implement this for anything outside this this crate.
pub trait Dimensioned {
    /// The type of the value of a quantity. E.g. For `si::Meter<f64>`, `Value` is `f64`.
    type Value;

    /// The units of a quanitity. This will be a type-array of type-numbers. E.g. For
    /// `si::Meter<f64>`, `Units` is `tarr![P1, Z0, Z0, Z0, Z0, Z0, Z0]`.
    type Units;

    /// Construct a new quantity.
    fn new(val: Self::Value) -> Self;

    /// Extract the value from a quantity. As this ignores the units completely, it is
    /// dimensionally unsafe.
    fn value_unsafe(&self) -> &Self::Value;
}

/// This trait is implemented for all quantities with no units. The unit systems that come with
/// dimensioned use `Unitless<V>` for that type.
pub trait Dimensionless: Dimensioned {
    /// Extract the value from a quantity with no units. As there are no units to ignore, it is
    /// dimensionally safe.
    fn value(&self) -> &Self::Value;
}

/// Perform an operation on a quantity.
///
/// Use of this function is discouraged except when necessary, as the operation may be one that
/// does not perserve units, and this function has no way to protect against that. If you do use
/// it, consider placing it in a trait or function that you can verify is dimensionally safe.
///
/// If associated type constructors or higher kinded types are implemented, then this trait should
/// no longer be necessary and may become deprecated.
///
/// # Example
///
/// Let's say we have a function that, when given a quantity with value type `Value` and unit type
/// `Units`, has output with value type `(Value, Value)` and squares the units. Then, we could
/// generically implement it for `Dimensioned` as follows:
///
/// ```rust
/// extern crate dimensioned as dim;
///
/// use dim::typenum::{Prod, P2};
/// use dim::{Dimensioned, MapUnsafe};
/// use std::ops::Mul;
///
/// pub trait Weird {
///     type Output;
///     fn weird(self) -> Self::Output;
/// }
///
/// impl<D, Value, Units> Weird for D
/// where
///     Value: Clone,
///     Units: Mul<P2>,
///     D: Dimensioned<Value = Value, Units = Units> + MapUnsafe<(Value, Value), Prod<Units, P2>>,
/// {
///     type Output = <D as MapUnsafe<(Value, Value), Prod<Units, P2>>>::Output;
///     fn weird(self) -> Self::Output {
///         self.map_unsafe(|v| (v.clone(), v))
///     }
/// }
///
/// fn main() {
///     use dim::si;
///     let x = 3.0 * si::M;
///     let w = x.weird();
///
///     assert_eq!(w, si::Meter2::new((3.0, 3.0)));
///
///     println!("w: {:?}", w);
///     // prints: w: (3, 3) m^2
/// }
/// ```
pub trait MapUnsafe<ValueOut, UnitsOut>: Dimensioned {
    /// The type to which the input is mapped
    type Output;

    /// Perform the map
    fn map_unsafe<F: FnOnce(Self::Value) -> ValueOut>(self, f: F) -> Self::Output;
}

/// Perform an operation on the contained value.
///
/// This trait is only defined for unitless types, and it keeps them unitless, so it is
/// perfectly safe to use.
///
/// It can be used similarly to `MapUnsafe`, but only for `Dimensionless` quantities, and it cannot
/// make them non-`Dimensionless`.
///
/// # Example
///
/// ```rust
/// extern crate dimensioned as dim;
///
/// fn main() {
///     use dim::si;
///     let x1 = 2.0 * si::ONE;
///     let x2 = 2.0f64;
///
///     use dim::Map;
///     assert_eq!(x1.map(|v| v.sin()), x2.sin() * si::ONE);
/// }
/// ```
pub trait Map<ValueOut>: Dimensionless {
    /// The type to which the input is mapped
    type Output;

    /// Perform the map
    fn map<F: FnOnce(Self::Value) -> ValueOut>(self, f: F) -> Self::Output;
}

#[cfg(feature = "oibit")]
/// Everything that is not a quantity implements this trait
pub auto trait NotDim {}

macro_rules! impl_unary {
    ($Type:ty, $Trait:ident, $fun:ident) => {
        impl $Trait for $Type {
            type Output = $Type;
            fn $fun(self) -> Self::Output {
                self.$fun()
            }
        }
    };
}

/// `Recip` is used for implementing a `recip()` member for types that are not preserved under
/// reciprocal.
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
/// use dim::si;
///
/// fn main() {
///     let t = 2.0 * si::S;
///     let f = 0.5 * si::HZ;
///
///     use dim::Recip;
///     assert_eq!(t.recip(), f);
/// }
/// ```
pub trait Recip {
    /// The resulting type after taking the reciprocal
    type Output;

    /// The method for taking the reciprocal
    fn recip(self) -> Self::Output;
}

impl_unary!(f32, Recip, recip);
impl_unary!(f64, Recip, recip);

/// `Abs` is used for implementing an `abs()` member for types that
/// can have their absolute value taken (which is most of them).
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
/// use dim::si;
///
/// fn main() {
///     let t = -2.0 * si::S;
///     let abst = 2.0 * si::S;
///
///     use dim::Abs;
///     assert_eq!(t.abs(), abst);
/// }
/// ```
pub trait Abs {
    /// The method for taking the absolute value
    fn abs(self) -> Self;
}

macro_rules! impl_abs {
    ($t:ty) => {
        impl Abs for $t {
            fn abs(self) -> Self {
                ::num_traits::sign::abs(self)
            }
        }
    };
}
impl_abs!(f32);
impl_abs!(f64);
impl_abs!(i8);
impl_abs!(i16);
impl_abs!(i32);
impl_abs!(i64);
impl_abs!(isize);

/// `Root` is used for implementing general integer roots for types that aren't necessarily
/// preserved under root.
///
/// It uses instantiated type numbers to specify the degree, as you can see in the example below.
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
///
/// fn main() {
/// # #[cfg(any(feature = "std", feature = "nightly"))]
/// # {
///     use dim::typenum::P2;
///     use dim::Root;
///     let x = 4.0.root(P2::new());
///     let y = 2.0;
///
///     assert_eq!(x, y);
/// # }
/// }
/// ```
pub trait Root<Index> {
    /// The resulting type after taking the `Index` root
    type Output;

    /// The method for taking the `idx` root
    fn root(self, idx: Index) -> Self::Output;
}

#[cfg(any(feature = "std", feature = "nightly"))]
use typenum::Integer;

macro_rules! impl_root {
    ($t:ty, $f:ident) => {
        #[cfg(any(feature = "std", feature = "nightly"))]
        impl<Index: Integer> Root<Index> for $t {
            type Output = $t;

            fn root(self, _: Index) -> Self::Output {
                #[allow(clippy::cast_lossless)]
                let exp = (Index::to_i32() as $t).recip();
                #[cfg(feature = "std")]
                return self.powf(exp);
                #[cfg(not(feature = "std"))]
                return unsafe { core::intrinsics::$f(self, exp) };
            }
        }
    };
}

impl_root!(f32, powf32);
impl_root!(f64, powf64);

#[cfg(any(feature = "std", feature = "nightly"))]
#[test]
fn test_root() {
    use typenum::consts::*;
    let radicands: &[f32] = &[0.0, 0.5, 1.0, 2.0];

    for &r in radicands {
        assert_eq!(r, r.root(P1::new()));
        assert_eq!(r, (r * r).root(P2::new()));
        assert_eq!(r, (r * r * r).root(P3::new()));
        assert_eq!(r, (r * r * r * r * r).root(P5::new()));
    }
}

/// `Sqrt` provides a `sqrt` member function for types that are not necessarily preserved under
/// square root.
///
/// # Example
///
/// ```rust
/// extern crate dimensioned as dim;
///
/// fn main() {
/// # #[cfg(any(feature = "std", feature = "nightly"))]
/// # {
///     use dim::si;
///     let x = 2.0 * si::M;
///     let a = 4.0 * si::M2;
///
///     use dim::Sqrt;
///     assert_eq!(a.sqrt(), x);
/// # }
/// }
/// ```
pub trait Sqrt {
    /// The resulting type after taking the square root
    type Output;

    /// The method for taking the square root
    fn sqrt(self) -> Self::Output;
}

/// `Cbrt` provides a `cbrt` member function for types that are not necessarily preserved under
/// cube root.
///
/// This is not currently implemented for built-in floating point types unless you are using `std`.
///
/// # Example
///
/// ```rust
/// extern crate dimensioned as dim;
///
/// fn main() {
///     use dim::si;
///     let x = 2.0 * si::M;
///     let v = 8.0 * si::M3;
///
///     use dim::Cbrt;
///     #[cfg(feature = "std")]
///     assert_eq!(v.cbrt(), x);
/// }
/// ```
pub trait Cbrt {
    /// The resulting type after taking the cube root
    type Output;

    /// The method for taking the cube root
    fn cbrt(self) -> Self::Output;
}

macro_rules! impl_sqcbroot {
    ($t:ty, $f:ident, $nan:path) => {
        #[cfg(any(feature = "std", feature = "nightly"))]
        impl Sqrt for $t {
            type Output = $t;
            fn sqrt(self) -> Self::Output {
                #[cfg(feature = "std")]
                return self.sqrt();
                #[cfg(not(feature = "std"))]
                {
                    if self < 0.0 {
                        $nan
                    } else {
                        unsafe { core::intrinsics::$f(self) }
                    }
                }
            }
        }

        #[cfg(feature = "std")]
        impl Cbrt for $t {
            type Output = $t;
            fn cbrt(self) -> Self::Output {
                self.cbrt()
            }
        }
    };
}

impl_sqcbroot!(f32, sqrtf32, core::f32::NAN);
impl_sqcbroot!(f64, sqrtf64, core::f64::NAN);
