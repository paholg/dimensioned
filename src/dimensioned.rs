use std::ops::{Add, Sub, Mul, Div};

pub trait AddDim<Rhs = Self> {
    type Output;
}

pub trait SubDim<Rhs = Self> {
    type Output;
}

pub trait Dim: AddDim + SubDim {}

pub struct Dimensioned<T: Dim, V: Add + Sub + Mul + Div>(pub V);

/// Adding! Dimensions must be the same (although value types can differ)
impl<T, Vl, Vr> Add<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
    where T: Dim, Vl: Add<Vr> + Add + Sub + Mul + Div, Vr: Add + Sub + Mul + Div, <Vl as Add<Vr>>::Output: Add + Sub + Mul + Div {
        type Output = Dimensioned<T, <Vl as Add<Vr>>::Output>;
        fn add(self, rhs: Dimensioned<T, Vr>) -> Dimensioned<T, <Vl as Add<Vr>>::Output> {
            Dimensioned(self.0 + rhs.0)
        }
}

/// Subtracting! Dimensions must be the same (although value types can differ)
impl<T, Vl, Vr> Sub<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
    where T: Dim, Vl: Sub<Vr> + Add + Sub + Mul + Div, Vr: Add + Sub + Mul + Div, <Vl as Sub<Vr>>::Output: Add + Sub + Mul + Div {
        type Output = Dimensioned<T, <Vl as Sub<Vr>>::Output>;
        fn sub(self, rhs: Dimensioned<T, Vr>) -> Dimensioned<T, <Vl as Sub<Vr>>::Output> {
            Dimensioned(self.0 - rhs.0)
        }
}

/// Multiplying! Dimensions must be able to add.
impl<Tl, Tr, Vl, Vr> Mul<Dimensioned<Tr, Vr>> for Dimensioned<Tl, Vl>
    where Tl: Dim + AddDim<Tr>, Tr: Dim, Vl: Mul<Vr> + Add + Sub + Mul + Div, Vr: Add + Sub + Mul + Div, <Vl as Mul<Vr>>::Output: Add + Sub + Mul + Div, <Tl as AddDim<Tr>>::Output: Dim {
        type Output = Dimensioned<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output>;
        fn mul(self, rhs: Dimensioned<Tr, Vr>) -> Dimensioned<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output> {
            Dimensioned(self.0 * rhs.0)
        }
}

/// Dividing! Dimensions must be able to subtract.
impl<Tl, Tr, Vl, Vr> Div<Dimensioned<Tr, Vr>> for Dimensioned<Tl, Vl>
    where Tl: Dim + SubDim<Tr>, Tr: Dim, Vl: Div<Vr> + Add + Sub + Mul + Div, Vr: Add + Sub + Mul + Div, <Vl as Div<Vr>>::Output: Add + Sub + Mul + Div, <Tl as SubDim<Tr>>::Output: Dim {
        type Output = Dimensioned<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output>;
        fn div(self, rhs: Dimensioned<Tr, Vr>) -> Dimensioned<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output> {
            Dimensioned(self.0 / rhs.0)
        }
}
