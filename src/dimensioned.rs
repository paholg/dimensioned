use std::ops::*;
//use std::num::Float;
use core::cmp::*;

pub trait AddDim<RHS = Self> {
    type Output;
}

pub trait SubDim<RHS = Self> {
    type Output;
}

pub trait Dim: AddDim + SubDim {}
pub trait Dimensionless: Dim {}

pub trait Scalar {}
impl Scalar for f64 {}
impl Scalar for f32 {}

pub struct Dimensioned<T: Dim, V>(pub V);
impl<T: Dim, V: Copy> Copy for Dimensioned<T, V> {}

// Implementing traits from std::ops:

/// Multiplying! Dimensions must be able to add.
impl<Tl, Tr, Vl, Vr> Mul<Dimensioned<Tr, Vr>> for Dimensioned<Tl, Vl>
    where Tl: Dim + AddDim<Tr>, Tr: Dim, Vl: Mul<Vr>, <Tl as AddDim<Tr>>::Output: Dim {
        type Output = Dimensioned<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output>;
        fn mul(self, rhs: Dimensioned<Tr, Vr>) -> Dimensioned<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output> {
            Dimensioned(self.0 * rhs.0)
        }
    }

/// Scalar multiplication (with scalar on RHS)!
impl<T, V, RHS> Mul<RHS> for Dimensioned<T, V>
    where T: Dim, V: Mul<RHS>, RHS: Scalar {
        type Output = Dimensioned<T, <V as Mul<RHS>>::Output>;
        fn mul(self, rhs: RHS) -> Dimensioned<T, <V as Mul<RHS>>::Output> {
            Dimensioned(self.0 * rhs)
        }
    }

// fixme: Waiting on Rust changes I believe
// /// Scalar multiplication (with scalar on LHS)!
// impl<T, V, Num> Mul<Dimensioned<T, V>> for Num
//     where T: Dim, Num: Mul<V> {
//         type Output = Dimensioned<T, <Num as Mul<V>>::Output>;
//         fn mul(self, rhs: Dimensioned<T, V>) -> Dimensioned<T, <Num as Mul<V>>::Output> {
//             Dimensioned( self * rhs.0 )
//         }
//     }

/// Dividing! Dimensions must be able to subtract.
impl<Tl, Tr, Vl, Vr> Div<Dimensioned<Tr, Vr>> for Dimensioned<Tl, Vl>
    where Tl: Dim + SubDim<Tr>, Tr: Dim, Vl: Div<Vr>, <Tl as SubDim<Tr>>::Output: Dim {
        type Output = Dimensioned<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output>;
        fn div(self, rhs: Dimensioned<Tr, Vr>) -> Dimensioned<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output> {
            Dimensioned(self.0 / rhs.0)
        }
    }

/// Scalar division (with scalar on RHS)!
impl<T, V, RHS> Div<RHS> for Dimensioned<T, V>
    where T: Dim, V: Div<RHS>, RHS: Scalar {
        type Output = Dimensioned<T, <V as Div<RHS>>::Output>;
        fn div(self, rhs: RHS) -> Dimensioned<T, <V as Div<RHS>>::Output> {
            Dimensioned(self.0 / rhs)
        }
    }

// fixme: waiting on Rust changes
// /// Scalar division (with scalar on LHS)!


// Unary operators:
macro_rules! define_unary_op {
    ($Trait:ident, $fun:ident) => (
        impl<T, V> $Trait for Dimensioned<T, V>
            where T: Dim, V: $Trait {
                type Output = Dimensioned<T, <V as $Trait>::Output>;
                fn $fun(self) -> Dimensioned<T, <V as $Trait>::Output> {
                    Dimensioned( (self.0).$fun() )
                }
            }
        )
}
define_unary_op!(Neg, neg);
define_unary_op!(Not, not);


// Binary operators that require type unchanged:
macro_rules! define_binary_op {
    ($Trait:ident, $fun:ident) => (
        impl<T, Vl, Vr> $Trait<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
            where T: Dim, Vl: $Trait<Vr> {
                type Output = Dimensioned<T, <Vl as $Trait<Vr>>::Output>;
                fn $fun(self, rhs: Dimensioned<T, Vr>) -> Dimensioned<T, <Vl as $Trait<Vr>>::Output> {
                    Dimensioned( (self.0).$fun(rhs.0) )
                }
            }
        )
}
define_binary_op!(Add, add);
define_binary_op!(BitAnd, bitand);
define_binary_op!(BitOr, bitor);
define_binary_op!(BitXor, bitxor);
define_binary_op!(Rem, rem);
define_binary_op!(Shl, shl);
define_binary_op!(Shr, shr);
define_binary_op!(Sub, sub);

// Implementing traits from core::cmp:
impl<T, Vl, Vr> PartialEq<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
    where T: Dim, Vl: PartialEq<Vr> {
        fn eq(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 == rhs.0 }
        fn ne(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 != rhs.0 }
    }
impl<T, V> Eq for Dimensioned<T, V> where T: Dim, V: Eq { }
impl<T, Vl, Vr> PartialOrd<Dimensioned<T, Vr>> for Dimensioned<T, Vl>
    where T: Dim, Vl: PartialOrd<Vr> {
        fn partial_cmp(&self, rhs: &Dimensioned<T, Vr>) -> Option<Ordering> { (self.0).partial_cmp(&rhs.0) }

        fn lt(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 <  rhs.0 }
        fn le(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 <= rhs.0 }
        fn gt(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 >  rhs.0 }
        fn ge(&self, rhs: &Dimensioned<T, Vr>) -> bool { self.0 >= rhs.0 }
    }
impl<T, V> Ord for Dimensioned<T, V> where T: Dim, V: Ord {
    fn cmp(&self, rhs: &Dimensioned<T, V>) -> Ordering { (self.0).cmp(&rhs.0) }
}




// fixme: figure this out
// impl<'a, T, V, I> Index<I> for Dimensioned<T, V>
//     where T: Dim, V: Index<I> + 'a, <V as Index<I>>::Output: Sized {
//         type Output = Dimensioned<T, &'a <V as Index<I>>::Output>;
//         fn index<'b>(&'b self, _index: &I) -> &'b Dimensioned<T, &'a <V as Index<I>>::Output> {
//             &Dimensioned((self.0).index(_index))
//         }
//     }


// Implementing Float for Dimensionless types

// macro_rules! define_unary_float {
//     ($fun:ident, $returns:ty) => (
//         fn $fun(self) -> $returns { Dimensioned( (self.0).$fun()) }
//         )
// }

// impl<T, V> Float for Dimensioned<T, V>
//     where T: Dimensionless, V: Float {
//         define_unary_float!(nan, Self);
//         define_unary_float!(infinity, Self);
//         define_unary_float!(neg_infinity, Self);
//         define_unary_float!(zero, Self);
//         define_unary_float!(neg_zero, Self);
//         define_unary_float!(one, Self);
//         define_unary_float!(epsilon, Self);
//         define_unary_float!(min_value, Self);
//         define_unary_float!(max_value, Self);
//         define_unary_float!(is_nan, bool);
//         define_unary_float!(is_infinite, bool);
//         define_unary_float!(is_finite, bool);
//         define_unary_float!(is_normal, bool);
//         // define_unary_float!(classify, FpCategory);
//         define_unary_float!(integer_decode, (u64, i16, i8));
//         define_unary_float!(floor, Self);
//         define_unary_float!(ceil, Self);
//         define_unary_float!(round, Self);
//         define_unary_float!(trunc, Self);
//         define_unary_float!(fract, Self);
//         define_unary_float!(abs, Self);
//         define_unary_float!(signum, Self);
//         define_unary_float!(is_positive, bool);
//         define_unary_float!(is_negative, bool);
//         define_unary_float!(recip, Self);
//         define_unary_float!(sqrt, Self);
//         define_unary_float!(rsqrt, Self);
//         define_unary_float!(exp, Self);
//         define_unary_float!(exp2, Self);
//         define_unary_float!(ln, Self);
//         define_unary_float!(log, Self);
//         define_unary_float!(log2, Self);
//         define_unary_float!(log10, Self);
//         define_unary_float!(to_degrees, Self);
//         define_unary_float!(to_radians, Self);
//         define_unary_float!(cbrt, Self);
//         define_unary_float!(sin, Self);
//         define_unary_float!(cos, Self);
//         define_unary_float!(tan, Self);
//         define_unary_float!(asin, Self);
//         define_unary_float!(acos, Self);
//         define_unary_float!(atan, Self);
//         define_unary_float!(sin_cos, (Self, Self));
//         define_unary_float!(exp_m1, Self);
//         define_unary_float!(ln_1p, Self);
//         define_unary_float!(sinh, Self);
//         define_unary_float!(cosh, Self);
//         define_unary_float!(tanh, Self);
//         define_unary_float!(asinh, Self);
//         define_unary_float!(acosh, Self);
//         define_unary_float!(atanh, Self);
//     }
