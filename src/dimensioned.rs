// use si::SI;
//use peano::*;
use std::ops::{Add, Sub, Mul, Div};


pub trait Dim {}

// fixme
pub struct Dimensioned<T: Dim, V: Add + Sub + Mul + Div>(pub V);

impl<T, Vl, Vr> Add<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
    where T: Dim, Vl: Add + Sub + Mul + Div, Vr: Add + Sub + Mul + Div, <Vl as Add<Vr>>::Output: Add + Sub + Mul + Div {
        type Output = Dimensioned<T, <Vl as Add<Vr>>::Output>;
        fn add(self, rhs: Dimensioned<T, Vr>) -> Dimensioned<T, <Vl as Add<Vr>>::Output> {
            Dimensioned(self.0 + rhs.0)
        }
}

// impl<T, V> Sub<Self> for Dimensioned<T, V>
//     where T: Dim, V: Add + Sub + Mul + Div, <V as Sub>::Output: Add + Sub + Mul + Div {
//         type Output = Dimensioned<T, <V as Sub>::Output>;
//         fn sub(self, rhs: Self) -> Dimensioned<T, <V as Sub>::Output> {
//             Dimensioned(self.0 - rhs.0)
//         }
// }

// impl<T1, V1, T2, V2> Mul<Self> for Dimensioned<T, V>
//     where T: Dim, V: Add + Sub + Mul + Div, <V as Add>::Output: Add + Sub + Mul + Div {
//         type Output = Dimensioned<T, <V as Add>::Output>;
//         fn add(self, rhs: Self) -> Dimensioned<T, <V as Add>::Output> {
//             Dimensioned(self.0 + rhs.0)
//         }
// }


// fixme: figure out how to do this
// impl<T: Dim, Rhs: Dim> AddDim<Rhs> for T {
//     type Result =
// }


// impl<T: Dim> Copy for Dimensioned<T> {}

// impl<T: Dim> Add for Dimensioned<T> {
//     type Output = Dimensioned<T>;
//     fn add(self, rhs: Dimensioned<T>) -> Dimensioned<T> {
//         Dimensioned(self.0 + rhs.0)
//     }
// }

// impl<T: Dim> Sub for Dimensioned<T> {
//     type Output = Dimensioned<T>;
//     fn sub(self, rhs: Dimensioned<T>) -> Dimensioned<T> {
//         Dimensioned(self.0 - rhs.0)
//     }
// }

// impl<T: Dim, U: Dim> Mul<Dimensioned<U>> for Dimensioned<T> {
//     type Output = Dimensioned<<T as AddDim<U>>::Result>;
//     fn mul(self, rhs: Dimensioned<U>) -> Dimensioned<<T as AddDim<U>>::Result> {
//         Dimensioned(self.0 * rhs.0)
//     }
// }


// fixme: pick more unique names? Preface all constants with a unit system?
