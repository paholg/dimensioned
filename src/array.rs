//! Tools for converting from type arrays of type numbers to generic arrays.
//!
//! This module may change, and will likely be removed someday if this functionality is implemented into typenum.
//!
//! Consider this module **unstable**.

use typenum::{ATerm, Add1, Integer, Len, Length, TArr, Unsigned, B1, U0};

use generic_array::{ArrayLength, GenericArray};

use core::ops::Add;

/// Implemented for `TArr` (a type-level array of type numbers), this gives the equivalent `GenericArray`.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
/// #[macro_use]
/// extern crate generic_array;
///
/// use dim::typenum::consts::*;
/// type TArr = tarr![P3, P2, N5, N8, P2];
///
/// fn main() {
///     use dim::array::ToGA;
///     let x = TArr::to_ga();
///     let y = arr![isize; 3, 2, -5, -8, 2];
///
///     assert_eq!(x, y);
/// }
/// ```
pub trait ToGA {
    /// The type of the `GenericArray` to which we've converted
    type Output;

    /// Create a `GenericArray` of integers from a `TArr` of type numbers.
    fn to_ga() -> Self::Output;
}

impl ToGA for ATerm {
    type Output = GenericArray<isize, U0>;
    fn to_ga() -> Self::Output {
        GenericArray::default()
    }
}

impl<V, A> ToGA for TArr<V, A>
where
    V: Integer,
    A: Len + ToGA,
    <A as ToGA>::Output: AppendFront<isize>,
    Length<A>: Add<B1>,
    Add1<Length<A>>: Unsigned + ArrayLength<isize>,
{
    type Output = <<A as ToGA>::Output as AppendFront<isize>>::Output;
    fn to_ga() -> Self::Output {
        A::to_ga().append_front(V::to_isize())
    }
}

/// Implemented for `GenericArray`, this allows growable `GenericArray`s by appending elements to the front.
///
/// # Example
/// ```rust
/// extern crate dimensioned as dim;
/// #[macro_use]
/// extern crate generic_array;
///
/// use dim::array::AppendFront;
///
/// fn main() {
///     let a = arr![u32; 1, 2];
///     let a2 = arr![u32; 0, 1, 2];
///
///     assert_eq!(a.append_front(0), a2);
/// }

pub trait AppendFront<T> {
    /// The resulting type after performing the append
    type Output;

    /// Append `element` to the front of `self`.
    fn append_front(self, element: T) -> Self::Output;
}

impl<T, N> AppendFront<T> for GenericArray<T, N>
where
    T: Default,
    N: Add<B1> + ArrayLength<T>,
    Add1<N>: ArrayLength<T>,
{
    type Output = GenericArray<T, Add1<N>>;
    fn append_front(self, element: T) -> Self::Output {
        let mut a = GenericArray::default();
        a[0] = element;
        for (i, el) in self.into_iter().enumerate() {
            a[i + 1] = el;
        }
        a
    }
}

#[test]
fn test_array() {
    use typenum::consts::*;
    type A = tarr![P1, N3, P4];
    let a = A::to_ga();

    assert_eq!(a, arr![isize; 1, -3, 4]);
}
