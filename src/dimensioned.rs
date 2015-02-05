use std::ops::*;
use std::num::{ToPrimitive, NumCast, Float};
use std::cmp::*;
//use std::string::String;
use std::fmt;

pub trait Dimension {}
pub trait Dimensionless: Dimension {}

pub trait AddDim<RHS = Self>: Dimension {
    type Output;
}
pub trait SubDim<RHS = Self>: Dimension {
    type Output;
}
pub trait MulDim<RHS = Self>: Dimension {
    type Output;
}
pub trait DivDim<RHS = Self>: Dimension {
    type Output;
}
pub trait DimToString: Dimension {
    fn to_string() -> String;
}

pub trait Scalar {}
impl Scalar for f64 {}
impl Scalar for f32 {}

pub struct Dim<T: Dimension, V>(pub V);

// trait PowI {
//     type Output;
// }
// impl<T, V> PowI for Dim<T, V>
//     where T: MulDim, V: Float {
//         type Output = Dim<>
//             fn powi(self, n: i32) ->
//     }

impl<T: Dimension, V: Copy> Copy for Dim<T, V> {}



//------------------------------------------------------------------------------
// Clone
//------------------------------------------------------------------------------
impl<T, V> Clone for Dim<T, V> where T: Dimension, V: Clone {
    fn clone(&self) -> Self {
        Dim((self.0).clone())
    }
    fn clone_from(&mut self, source: &Self) {
        (self.0).clone_from(&source.0);
    }
}

//------------------------------------------------------------------------------
// Traits from std::fmt
//------------------------------------------------------------------------------
impl<T, V> fmt::Display for Dim<T, V> where T: DimToString, V: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, <T as DimToString>::to_string())
    }
}
//------------------------------------------------------------------------------
// Traits from std::ops
//------------------------------------------------------------------------------

/// Multiplying! Dimensions must be able to add.
impl<Tl, Tr, Vl, Vr> Mul<Dim<Tr, Vr>> for Dim<Tl, Vl>
    where Tl: Dimension + AddDim<Tr>, Tr: Dimension, Vl: Mul<Vr>, <Tl as AddDim<Tr>>::Output: Dimension {
        type Output = Dim<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output>;
        fn mul(self, rhs: Dim<Tr, Vr>) -> Dim<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output> {
            Dim(self.0 * rhs.0)
        }
    }

/// Scalar multiplication (with scalar on RHS)!
impl<T, V, RHS> Mul<RHS> for Dim<T, V>
    where T: Dimension, V: Mul<RHS>, RHS: Scalar {
        type Output = Dim<T, <V as Mul<RHS>>::Output>;
        fn mul(self, rhs: RHS) -> Dim<T, <V as Mul<RHS>>::Output> {
            Dim(self.0 * rhs)
        }
    }

// fixme: Waiting on Rust changes I believe
// /// Scalar multiplication (with scalar on LHS)!
// impl<T, V, Num> Mul<Dim<T, V>> for Num
//     where T: Dimension, Num: Mul<V> {
//         type Output = Dim<T, <Num as Mul<V>>::Output>;
//         fn mul(self, rhs: Dim<T, V>) -> Dim<T, <Num as Mul<V>>::Output> {
//             Dim( self * rhs.0 )
//         }
//     }

/// Dividing! Dimensions must be able to subtract.
impl<Tl, Tr, Vl, Vr> Div<Dim<Tr, Vr>> for Dim<Tl, Vl>
    where Tl: Dimension + SubDim<Tr>, Tr: Dimension, Vl: Div<Vr>, <Tl as SubDim<Tr>>::Output: Dimension {
        type Output = Dim<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output>;
        fn div(self, rhs: Dim<Tr, Vr>) -> Dim<<Tl as SubDim<Tr>>::Output, <Vl as Div<Vr>>::Output> {
            Dim(self.0 / rhs.0)
        }
    }

/// Scalar division (with scalar on RHS)!
impl<T, V, RHS> Div<RHS> for Dim<T, V>
    where T: Dimension, V: Div<RHS>, RHS: Scalar {
        type Output = Dim<T, <V as Div<RHS>>::Output>;
        fn div(self, rhs: RHS) -> Dim<T, <V as Div<RHS>>::Output> {
            Dim(self.0 / rhs)
        }
    }

// fixme: waiting on Rust changes
// /// Scalar division (with scalar on LHS)!


// Unary operators:
macro_rules! define_unary_op {
    ($Trait:ident, $fun:ident) => (
        impl<T, V> $Trait for Dim<T, V>
            where T: Dimension, V: $Trait {
                type Output = Dim<T, <V as $Trait>::Output>;
                fn $fun(self) -> Dim<T, <V as $Trait>::Output> {
                    Dim( (self.0).$fun() )
                }
            }
        )
}
define_unary_op!(Neg, neg);
define_unary_op!(Not, not);


// Binary operators that require type unchanged:
macro_rules! define_binary_op {
    ($Trait:ident, $fun:ident) => (
        impl<T, Vl, Vr> $Trait<Dim<T, Vr>> for Dim<T, Vl>
            where T: Dimension, Vl: $Trait<Vr> {
                type Output = Dim<T, <Vl as $Trait<Vr>>::Output>;
                fn $fun(self, rhs: Dim<T, Vr>) -> Dim<T, <Vl as $Trait<Vr>>::Output> {
                    Dim( (self.0).$fun(rhs.0) )
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

impl<T, V> Deref for Dim<T, V> {
    type Target = V;
    fn deref<'a>(&'a self) -> &'a V { &self.0 }
}
impl<T, V> DerefMut for Dim<T, V> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut V { &mut self.0 }
}

//------------------------------------------------------------------------------
// Traits from core::cmp
//------------------------------------------------------------------------------
impl<T, Vl, Vr> PartialEq<Dim<T, Vr>> for Dim<T, Vl>
    where T: Dimension, Vl: PartialEq<Vr> {
        fn eq(&self, rhs: &Dim<T, Vr>) -> bool { self.0 == rhs.0 }
        fn ne(&self, rhs: &Dim<T, Vr>) -> bool { self.0 != rhs.0 }
    }
impl<T, V> Eq for Dim<T, V> where T: Dimension, V: Eq { }
impl<T, Vl, Vr> PartialOrd<Dim<T, Vr>> for Dim<T, Vl>
    where T: Dimension, Vl: PartialOrd<Vr> {
        fn partial_cmp(&self, rhs: &Dim<T, Vr>) -> Option<Ordering> { (self.0).partial_cmp(&rhs.0) }

        fn lt(&self, rhs: &Dim<T, Vr>) -> bool { self.0 <  rhs.0 }
        fn le(&self, rhs: &Dim<T, Vr>) -> bool { self.0 <= rhs.0 }
        fn gt(&self, rhs: &Dim<T, Vr>) -> bool { self.0 >  rhs.0 }
        fn ge(&self, rhs: &Dim<T, Vr>) -> bool { self.0 >= rhs.0 }
    }
impl<T, V> Ord for Dim<T, V> where T: Dimension, V: Ord {
    fn cmp(&self, rhs: &Dim<T, V>) -> Ordering { (self.0).cmp(&rhs.0) }
}




// fixme: figure this out
// impl<'a, T, V, I> Index<I> for Dim<T, V>
//     where T: Dimension, V: Index<I> + 'a, <V as Index<I>>::Output: Sized {
//         type Output = Dim<T, &'a <V as Index<I>>::Output>;
//         fn index<'b>(&'b self, _index: &I) -> &'b Dim<T, &'a <V as Index<I>>::Output> {
//             &Dim((self.0).index(_index))
//         }
//     }

//------------------------------------------------------------------------------
// DIMENSIONLESS THINGS HERE
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Dimensionless multiplication and division
// fixme
// impl<T, V> Mul for Dim<T, V>
//     where T: Dimensionless, V: Mul {
//         type Output = Dim<T, <V as Mul>::Output>;
//         fn mul(self, rhs: Self) -> Dim<T, <V as Mul>::Output> {
//             Dim(self.0 * rhs.0)
//         }
//     }

macro_rules! define_cast_fun {
    ($fun:ident, $prim:ident) => (
        fn $fun(&self) -> Option<$prim> {
            (self.0).$fun()
        }
        )
}
//------------------------------------------------------------------------------
// ToPrimitive
impl<T, V> ToPrimitive for Dim<T, V> where T: Dimensionless, V: ToPrimitive {
    define_cast_fun!(to_i64, i64);
    define_cast_fun!(to_u64, u64);
    define_cast_fun!(to_int, isize);
    define_cast_fun!(to_i8, i8);
    define_cast_fun!(to_i16, i16);
    define_cast_fun!(to_i32, i32);
    define_cast_fun!(to_uint, usize);
    define_cast_fun!(to_u8, u8);
    define_cast_fun!(to_u16, u16);
    define_cast_fun!(to_u32, u32);
    define_cast_fun!(to_f32, f32);
    define_cast_fun!(to_f64, f64);
}

//------------------------------------------------------------------------------
// NumCast
impl<T, V> NumCast for Dim<T, V> where T: Dimensionless, V: NumCast {
    fn from<N>(n: N) -> Option<Self> where N: ToPrimitive {
        match NumCast::from(n) {
            Some(v) => Some(Dim(v)),
            None => None
        }
    }
}

//------------------------------------------------------------------------------
// Float
macro_rules! define_unary_float {
    ($fun:ident, $returns:ty) => (
        fn $fun(self) -> $returns { Dim( (self.0).$fun()) }
        )
}

// impl<T, V> Float for Dim<T, V>
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
