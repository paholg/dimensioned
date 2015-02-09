use std::ops::*;
use std::num::{ToPrimitive, NumCast, Float};
use std::cmp::*;
//use std::string::String;
use std::fmt;

pub use peano::*;

pub trait Dimension {}
pub trait Dimensionless: Dimension {}

pub trait AddDim<RHS = Self>: Dimension {
    type Output;
}
pub trait SubDim<RHS = Self>: Dimension {
    type Output;
}
pub trait MulDim<RHS>: Dimension {
    type Output;
}
pub trait DivDim<RHS>: Dimension {
    type Output;
}
pub trait KeepDim<RHS = Self>: Dimension {
    type Output;
}
pub trait DimToString: Dimension {
    fn to_string() -> String;
}

pub trait Scalar {}
impl Scalar for f64 {}
impl Scalar for f32 {}

pub struct Dim<T: Dimension, V>(pub V);

pub trait Wrap<B> {
    type Output;
    fn wrap(&self, b: B) -> <Self as Wrap<B>>::Output;
}
impl<T, A, B> Wrap<B> for Dim<T, A>
    where T: Dimension {
        type Output = Dim<T, B>;
        fn wrap(&self, b: B) -> Dim<T, B> {
            Dim(b)
        }
}


pub trait Sqrt {
    type Output;
    fn sqrt(self) -> <Self as Sqrt>::Output;
}
impl<T, V> Sqrt for Dim<T, V>
    where T:  DivDim<Two>, V: Float, <T as DivDim<Two>>::Output: Dimension {
        type Output = Dim<<T as DivDim<Two>>::Output, V>;
        fn sqrt(self) -> <Self as Sqrt>::Output { Dim( (self.0).sqrt()) }
    }

// pub trait PowI<RHS> {
//     type Output;
//     fn powi(self) -> <Self as PowI<RHS>>::Output;
// }
// impl<T, V, RHS> PowI<RHS> for Dim<T, V>
//     where T: MulDim<RHS>, V: Float, RHS: PInt, <T as MulDim<RHS>>::Output: Dimension {
//         type Output = Dim<<T as MulDim<RHS>>::Output, V>;
//         fn powi(self) -> <Self as PowI<RHS>>::Output {
//             Dim( (self.0).powi( <RHS as ToInt>::to_int() ) )
//         }
//     }


pub trait Sqr {
    type Output;
    fn sqr(self) -> <Self as Sqr>::Output;
}
impl<T, V> Sqr for Dim<T, V>
    where T: AddDim<T>, V: Copy + Mul, <T as AddDim<T>>::Output: Dimension {
        type Output = Dim<<T as AddDim<T>>::Output, <V as Mul<V>>::Output>;
        fn sqr(self) -> <Self as Sqr>::Output {
            Dim( (self.0)*(self.0) )
        }
    }

// pub trait PowI<Exp> {
//     type Output;
//     fn powi(self, n: i32) -> <Self as PowI<Exp>>::Output;
// }
// impl<T, V, Exp> PowI<Exp> for Dim<T, V>
//     where T: MulDim<Exp>, V: Float, Exp: PInt, <T as MulDim<Exp>>::Output: Dimension {
//         type Output = Dim<<T as MulDim<Exp>>::Output, V>;
//         fn powi(self) -> <Self as PowI<Exp>>::Output {
//             Dim( (self.0).powi( <Exp as ToInt>::to_int() ) )
//         }
// }



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
#[macro_export]
#[macro_use(dim_unary)]
macro_rules! dim_unary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<T, V> $Trait for Dim<T, V>
            where T: $op<T>, V: $Trait, <T as $op<T>>::Output: Dimension {
                type Output = Dim<<T as $op<T>>::Output, <V as $Trait>::Output>;
                $(fn $fun(self) -> Dim<<T as $op<T>>::Output, <V as $Trait>::Output> {
                    Dim( (self.0).$fun() )
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
        impl<Tl, Vl, Tr, Vr> $Trait<Dim<Tr, Vr>> for Dim<Tl, Vl>
            where Tl: $op<Tr>, Tr: Dimension, Vl: $Trait<Vr>, <Tl as $op<Tr>>::Output: Dimension {
                type Output = Dim<<Tl as $op<Tr>>::Output, <Vl as $Trait<Vr>>::Output>;
                $(fn $fun(self, rhs: Dim<Tr, Vr>) -> Dim<<Tl as $op<Tr>>::Output, <Vl as $Trait<Vr>>::Output> {
                    Dim( (self.0).$fun(rhs.0) )
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

macro_rules! dim_cast_fun {
    ($fun:ident, $prim:ident) => (
        fn $fun(&self) -> Option<$prim> {
            (self.0).$fun()
        }
        )
}
//------------------------------------------------------------------------------
// ToPrimitive
impl<T, V> ToPrimitive for Dim<T, V> where T: Dimensionless, V: ToPrimitive {
    dim_cast_fun!(to_i64, i64);
    dim_cast_fun!(to_u64, u64);
    dim_cast_fun!(to_int, isize);
    dim_cast_fun!(to_i8, i8);
    dim_cast_fun!(to_i16, i16);
    dim_cast_fun!(to_i32, i32);
    dim_cast_fun!(to_uint, usize);
    dim_cast_fun!(to_u8, u8);
    dim_cast_fun!(to_u16, u16);
    dim_cast_fun!(to_u32, u32);
    dim_cast_fun!(to_f32, f32);
    dim_cast_fun!(to_f64, f64);
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
macro_rules! dim_unary_float {
    ($fun:ident, $returns:ty) => (
        fn $fun(self) -> $returns { Dim( (self.0).$fun()) }
        )
}

// impl<T, V> Float for Dim<T, V>
//     where T: Dimensionless, V: Float {
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
