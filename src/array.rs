//! Some tools for converting from type arrays of type numbers to generic arrays.
//!
//! This module is implemented very hackily and will be replaced with something better in the
//! future.
//!
//! Don't rely on or use directly anything here.

use typenum::*;

use generic_array::{GenericArray, ArrayLength};

use reexported::ops::Add;

pub trait ToGA {
    type Output;
    fn to_ga() -> Self::Output;
}


impl ToGA for ATerm {
    type Output = GenericArray<isize, U0>;
    fn to_ga() -> Self::Output {
        GenericArray::new()
    }
}


impl<V, A> ToGA for TArr<V, A> where
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

pub trait AppendFront<T> {
    type Output;
    fn append_front(self, element: T) -> Self::Output;
}

impl<T, N> AppendFront<T> for GenericArray<T, N> where
    T: Default,
    N: Add<B1> + ArrayLength<T>,
    Add1<N>: ArrayLength<T>
{
    type Output = GenericArray<T, Add1<N>>;
    fn append_front(self, element: T) -> Self::Output {
        let mut a = GenericArray::new();
        a[0] = element;
        for (i, el) in self.into_iter().enumerate() {
            a[i + 1] = el;
        }
        a
    }
}

#[test]
fn test() {
    type A = tarr![P1, N3, P4];
    let a = A::to_ga();

    for i in a.iter() {
        println!("{}", i);
    }

    assert_eq!(a, arr![isize; 1, -3, 4]);
}
