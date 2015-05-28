pub use peano::*;
pub use std::marker::PhantomData;

use std::ops::*;
use num::traits::{ToPrimitive, NumCast};
use std::cmp::*;
use std::fmt;

use num::Float;

pub trait Dimension {}

pub trait Dimensionless: Dimension {}

pub trait AddDim<RHS>: Dimension {
    type Output;
}
pub trait SubDim<RHS>: Dimension {
    type Output;
}
pub trait MulDim<RHS>: Dimension {
    type Output;
}
pub trait DivDim<RHS>: Dimension {
    type Output;
}
pub trait KeepDim<RHS>: Dimension {
    type Output;
}
pub trait DimToString: Dimension {
    fn to_string() -> String;
}

pub trait Scalar {}
impl Scalar for f64 {}
impl Scalar for f32 {}

pub struct Dim<D: Dimension, V>(pub V, pub PhantomData<D>);
impl<D: Dimension, V> Dim<D, V> {
    pub fn new(v: V) -> Dim<D, V> {
        Dim(v, PhantomData)
    }
}
impl<D: Dimension, V: Copy> Copy for Dim<D, V> {}

pub trait Wrap<B> {
    type Output;
    fn wrap(&self, b: B) -> <Self as Wrap<B>>::Output;
}
impl<D, A, B> Wrap<B> for Dim<D, A>
    where D: Dimension {
        type Output = Dim<D, B>;
        fn wrap(&self, b: B) -> Dim<D, B> { Dim(b, PhantomData) }
}

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> <Self as Sqrt>::Output;
}
impl<D, V> Sqrt for Dim<D, V>
    where D:  DivDim<Two>, V: Float, <D as DivDim<Two>>::Output: Dimension {
        type Output = Dim<<D as DivDim<Two>>::Output, V>;

        fn sqrt(self) -> <Self as Sqrt>::Output { Dim( (self.0).sqrt(), PhantomData) }
}

pub trait Sqr {
    type Output;
    fn sqr(self) -> <Self as Sqr>::Output;
}
impl<D, V> Sqr for Dim<D, V> where D: MulDim<Two>, V: Copy + Mul, <D as MulDim<Two>>::Output: Dimension {
    type Output = Dim<<D as MulDim<Two>>::Output, <V as Mul<V>>::Output>;

    fn sqr(self) -> <Self as Sqr>::Output {
        Dim( (self.0)*(self.0), PhantomData )
    }
}

// pub trait PowI<Exp> {
//     type Output;
//     fn powi(self) -> <Self as PowI<Exp>>::Output;
// }
// impl<D, V, Exp> PowI<Exp> for Dim<D, V>
//     where D: MulDim<Exp>, V: Float, Exp: ToInt, <D as MulDim<Exp>>::Output: Dimension {
//         type Output = Dim<<D as MulDim<Exp>>::Output, V>;
//         fn powi(self) -> <Self as PowI<Exp>>::Output {
//             Dim( (self.0).powi( <Exp as ToInt>::to_int() ) )
//         }
// }

// pub trait PowI<Exp> {
//     type Output;
//     fn powi(self) -> <Self as PowI<Exp>>::Output;
// }
// impl<D, V, Exp> PowI<Exp> for Dim<D, V>
//     where D: MulDim<Exp>, V: Float, Exp: ToInt, <D as MulDim<Exp>>::Output: Dimension {
//         type Output = Dim<<D as MulDim<Exp>>::Output, V>;
//         fn powi(self) -> <Self as PowI<Exp>>::Output {
//             Dim( (self.0).powi( <Exp as ToInt>::to_int() ) )
//         }
// }
// macro_rules! pow {
//     ($Exp:ident, $nexp: expr) => (
//         pub trait Pow$nexp {
//             type Output;
//             fn pow$nexp(self) -> <Self as Pow$nexp>::Output;
//         }
//         impl<D, V> Pow$nexp for Dim<D, V>
//             where D: MulDim<$Exp>, V: Float, <D as MulDim<$Exp>>::Output: Dimension {
//                 type Output = Dim<<D as MulDim<$Exp>>::Output, V>;
//                 fn pow$nexp(self) -> <Self as Pow$nexp>::Output {
//                     Dim( (self.0).powi($nexp) )
//                 }
//             }
//         )
// }
// pow!(Two, 2);


//------------------------------------------------------------------------------
// Clone
//------------------------------------------------------------------------------
impl<D, V> Clone for Dim<D, V> where D: Dimension, V: Clone {
    fn clone(&self) -> Self {
        Dim((self.0).clone(), PhantomData)
    }
    fn clone_from(&mut self, source: &Self) {
        (self.0).clone_from(&source.0);
    }
}

//------------------------------------------------------------------------------
// Traits from std::fmt
//------------------------------------------------------------------------------
impl<D, V> fmt::Display for Dim<D, V> where D: DimToString, V: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, <D as DimToString>::to_string())
    }
}
//------------------------------------------------------------------------------
// Traits from std::ops
//------------------------------------------------------------------------------

/// Multiplying! Dimensions must be able to add.
impl<Dl, Dr, Vl, Vr> Mul<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Dimension + AddDim<Dr>, Dr: Dimension, Vl: Mul<Vr>, <Dl as AddDim<Dr>>::Output: Dimension {
        type Output = Dim<<Dl as AddDim<Dr>>::Output, <Vl as Mul<Vr>>::Output>;

        fn mul(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as AddDim<Dr>>::Output, <Vl as Mul<Vr>>::Output> {
            Dim(self.0 * rhs.0, PhantomData)
        }
}

/// Scalar multiplication (with scalar on RHS)!
impl<D, V, RHS> Mul<RHS> for Dim<D, V>
    where D: Dimension, V: Mul<RHS>, RHS: Scalar {
        type Output = Dim<D, <V as Mul<RHS>>::Output>;
        fn mul(self, rhs: RHS) -> Dim<D, <V as Mul<RHS>>::Output> {
            Dim(self.0 * rhs, PhantomData)
        }
    }

// fixme: Waiting on Rust changes I believe
// /// Scalar multiplication (with scalar on LHS)!
// impl<D, V, Num> Mul<Dim<D, V>> for Num
//     where D: Dimension, Num: Mul<V> {
//         type Output = Dim<D, <Num as Mul<V>>::Output>;
//         fn mul(self, rhs: Dim<D, V>) -> Dim<D, <Num as Mul<V>>::Output> {
//             Dim( self * rhs.0 )
//         }
//     }

/// Dividing! Dimensions must be able to subtract.
impl<Dl, Dr, Vl, Vr> Div<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Dimension + SubDim<Dr>, Dr: Dimension, Vl: Div<Vr>, <Dl as SubDim<Dr>>::Output: Dimension {
        type Output = Dim<<Dl as SubDim<Dr>>::Output, <Vl as Div<Vr>>::Output>;
        fn div(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as SubDim<Dr>>::Output, <Vl as Div<Vr>>::Output> {
            Dim(self.0 / rhs.0, PhantomData)
        }
    }

/// Scalar division (with scalar on RHS)!
impl<D, V, RHS> Div<RHS> for Dim<D, V>
    where D: Dimension, V: Div<RHS>, RHS: Scalar {
        type Output = Dim<D, <V as Div<RHS>>::Output>;
        fn div(self, rhs: RHS) -> Dim<D, <V as Div<RHS>>::Output> {
            Dim(self.0 / rhs, PhantomData)
        }
    }

// fixme: waiting on Rust changes
// /// Scalar division (with scalar on LHS)!


// Unary operators:
#[macro_export]
#[macro_use(dim_unary)]
macro_rules! dim_unary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<D, V> $Trait for Dim<D, V>
            where D: $op<D>, V: $Trait, <D as $op<D>>::Output: Dimension {
                type Output = Dim<<D as $op<D>>::Output, <V as $Trait>::Output>;
                $(fn $fun(self) -> Dim<<D as $op<D>>::Output, <V as $Trait>::Output> {
                    Dim( (self.0).$fun(), PhantomData )
                })*
            }
        )
}
dim_unary!(Neg, KeepDim, neg);
dim_unary!(Not, KeepDim, not);

// Binary operators:
#[macro_export]
#[macro_use(dim_binary)]
macro_rules! dim_binary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<Dl, Vl, Dr, Vr> $Trait<Dim<Dr, Vr>> for Dim<Dl, Vl>
            where Dl: $op<Dr>, Dr: Dimension, Vl: $Trait<Vr>, <Dl as $op<Dr>>::Output: Dimension {
                type Output = Dim<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output>;
                $(fn $fun(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as $op<Dr>>::Output, <Vl as $Trait<Vr>>::Output> {
                    Dim( (self.0).$fun(rhs.0), PhantomData )
                })*
            }
        )
}
dim_binary!(Add, KeepDim, add);
dim_binary!(BitAnd, KeepDim, bitand);
dim_binary!(BitOr, KeepDim, bitor);
dim_binary!(BitXor, KeepDim, bitxor);
dim_binary!(Rem, KeepDim, rem);
dim_binary!(Shl, KeepDim, shl);
dim_binary!(Shr, KeepDim, shr);
dim_binary!(Sub, KeepDim, sub);

//------------------------------------------------------------------------------
// Traits from core::cmp
//------------------------------------------------------------------------------
impl<D, Vl, Vr> PartialEq<Dim<D, Vr>> for Dim<D, Vl>
    where D: Dimension, Vl: PartialEq<Vr> {
        fn eq(&self, rhs: &Dim<D, Vr>) -> bool { self.0 == rhs.0 }
        fn ne(&self, rhs: &Dim<D, Vr>) -> bool { self.0 != rhs.0 }
    }
impl<D, V> Eq for Dim<D, V> where D: Dimension, V: Eq { }
impl<D, Vl, Vr> PartialOrd<Dim<D, Vr>> for Dim<D, Vl>
    where D: Dimension, Vl: PartialOrd<Vr> {
        fn partial_cmp(&self, rhs: &Dim<D, Vr>) -> Option<Ordering> { (self.0).partial_cmp(&rhs.0) }

        fn lt(&self, rhs: &Dim<D, Vr>) -> bool { self.0 <  rhs.0 }
        fn le(&self, rhs: &Dim<D, Vr>) -> bool { self.0 <= rhs.0 }
        fn gt(&self, rhs: &Dim<D, Vr>) -> bool { self.0 >  rhs.0 }
        fn ge(&self, rhs: &Dim<D, Vr>) -> bool { self.0 >= rhs.0 }
    }
impl<D, V> Ord for Dim<D, V> where D: Dimension, V: Ord {
    fn cmp(&self, rhs: &Dim<D, V>) -> Ordering { (self.0).cmp(&rhs.0) }
}




// fixme: figure this out
// impl<'a, D, V, I> Index<I> for Dim<D, V>
//     where D: Dimension, V: Index<I> + 'a, <V as Index<I>>::Output: Sized {
//         type Output = Dim<D, &'a <V as Index<I>>::Output>;
//         fn index<'b>(&'b self, _index: &I) -> &'b Dim<D, &'a <V as Index<I>>::Output> {
//             &Dim((self.0).index(_index))
//         }
//     }

//------------------------------------------------------------------------------
// DIMENSIONLESS THINGS HERE
//------------------------------------------------------------------------------

//------------------------------------------------------------------------------
// Dimensionless multiplication and division
// fixme
// impl<D, V> Mul for Dim<D, V>
//     where D: Dimensionless<D = NoDim>, V: Mul {
//         type Output = Dim<D, <V as Mul>::Output>;
//         fn mul(self, rhs: Self) -> Dim<D, <V as Mul>::Output> {
//             Dim(self.0 * rhs.0)
//         }
//     }

macro_rules! dim_cast_fun {
    ($fun:ident, $prim:ident) => (
        fn $fun(&self) -> Option<$prim> {
            (self.0).$fun()
        }
        )
}
//------------------------------------------------------------------------------
// ToPrimitive
impl<D, V> ToPrimitive for Dim<D, V> where D: Dimensionless, V: ToPrimitive {
    dim_cast_fun!(to_i64, i64);
    dim_cast_fun!(to_u64, u64);
    dim_cast_fun!(to_isize, isize);
    dim_cast_fun!(to_i8, i8);
    dim_cast_fun!(to_i16, i16);
    dim_cast_fun!(to_i32, i32);
    dim_cast_fun!(to_usize, usize);
    dim_cast_fun!(to_u8, u8);
    dim_cast_fun!(to_u16, u16);
    dim_cast_fun!(to_u32, u32);
    dim_cast_fun!(to_f32, f32);
    dim_cast_fun!(to_f64, f64);
}

//------------------------------------------------------------------------------
// NumCast
impl<D, V> NumCast for Dim<D, V> where D: Dimensionless, V: NumCast {
    fn from<N>(n: N) -> Option<Self> where N: ToPrimitive {
        match NumCast::from(n) {
            Some(v) => Some(Dim(v, PhantomData)),
            None => None
        }
    }
}

//------------------------------------------------------------------------------
// Float
macro_rules! dim_unary_float {
    ($fun:ident, $returns:ty) => (
        fn $fun(self) -> $returns { Dim( (self.0).$fun(), PhantomData) }
        )
}

// impl<D, V> Float for Dim<D, V>
//     where D: Dimensionless + KeepDim<D>, V: Float, <D as KeepDim<D>>::Output: Dimensionless {
//         // fn nan(self) -> Dim<D, V> {Dim ( (self.0).nan() )}
//         dim_unary_float!(nan, Self);
//         dim_unary_float!(infinity, Self);
//         dim_unary_float!(neg_infinity, Self);
//         dim_unary_float!(zero, Self);
//         dim_unary_float!(neg_zero, Self);
//         dim_unary_float!(one, Self);
//         dim_unary_float!(epsilon, Self);
//         dim_unary_float!(min_value, Self);
//         dim_unary_float!(max_value, Self);
//         dim_unary_float!(is_nan, bool);
//         dim_unary_float!(is_infinite, bool);
//         dim_unary_float!(is_finite, bool);
//         dim_unary_float!(is_normal, bool);
//         // dim_unary_float!(classify, FpCategory);
//         dim_unary_float!(integer_decode, (u64, i16, i8));
//         dim_unary_float!(floor, Self);
//         dim_unary_float!(ceil, Self);
//         dim_unary_float!(round, Self);
//         dim_unary_float!(trunc, Self);
//         dim_unary_float!(fract, Self);
//         dim_unary_float!(abs, Self);
//         dim_unary_float!(signum, Self);
//         dim_unary_float!(is_positive, bool);
//         dim_unary_float!(is_negative, bool);
//         dim_unary_float!(recip, Self);
//         dim_unary_float!(sqrt, Self);
//         dim_unary_float!(rsqrt, Self);
//         dim_unary_float!(exp, Self);
//         dim_unary_float!(exp2, Self);
//         dim_unary_float!(ln, Self);
//         dim_unary_float!(log, Self);
//         dim_unary_float!(log2, Self);
//         dim_unary_float!(log10, Self);
//         dim_unary_float!(to_degrees, Self);
//         dim_unary_float!(to_radians, Self);
//         dim_unary_float!(cbrt, Self);
//         dim_unary_float!(sin, Self);
//         dim_unary_float!(cos, Self);
//         dim_unary_float!(tan, Self);
//         dim_unary_float!(asin, Self);
//         dim_unary_float!(acos, Self);
//         dim_unary_float!(atan, Self);
//         dim_unary_float!(sin_cos, (Self, Self));
//         dim_unary_float!(exp_m1, Self);
//         dim_unary_float!(ln_1p, Self);
//         dim_unary_float!(sinh, Self);
//         dim_unary_float!(cosh, Self);
//         dim_unary_float!(tanh, Self);
//         dim_unary_float!(asinh, Self);
//         dim_unary_float!(acosh, Self);
//         dim_unary_float!(atanh, Self);
//     }
