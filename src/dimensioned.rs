pub use peano::*;
pub use std::marker::PhantomData;

use std::ops::*;
use num::traits::{FromPrimitive, ToPrimitive, NumCast, Float};
use std::cmp::*;
use std::fmt;

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
pub trait NegDim: Dimension {
    type Output;
}

pub trait DimToString: Dimension {
    fn to_string() -> String;
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dim<D: Dimension, V>(pub V, pub PhantomData<D>);

impl<D: Dimension, V> Dim<D, V> {
    pub fn new(v: V) -> Dim<D, V> {
        Dim(v, PhantomData)
    }
}

pub trait NotDim {}
impl NotDim for .. {}
impl<D: Dimension, V> !NotDim for Dim<D, V> {}

pub trait Wrap<B> {
    type Output;
    fn wrap(&self, b: B) -> <Self as Wrap<B>>::Output;
}
impl<D, A, B> Wrap<B> for Dim<D, A>
    where D: Dimension {
        type Output = Dim<D, B>;
        #[inline]
        fn wrap(&self, b: B) -> Dim<D, B> { Dim(b, PhantomData) }
}

pub trait Sqrt {
    type Output;
    fn sqrt(self) -> <Self as Sqrt>::Output;
}

impl<T> Sqrt for T where T: Float {
    type Output = T;
    #[inline]
    fn sqrt(self) -> Self::Output { self.sqrt() }
}

impl<D, V> Sqrt for Dim<D, V> where D:  DivDim<Two>, V: Float, <D as DivDim<Two>>::Output: Dimension {
    type Output = Dim<<D as DivDim<Two>>::Output, V>;
    #[inline]
    fn sqrt(self) -> <Self as Sqrt>::Output { Dim( (self.0).sqrt(), PhantomData) }
}

pub trait Sqr {
    type Output;
    fn sqr(self) -> <Self as Sqr>::Output;
}
impl<D, V> Sqr for Dim<D, V> where D: MulDim<Two>, V: Copy + Mul, <D as MulDim<Two>>::Output: Dimension {
    type Output = Dim<<D as MulDim<Two>>::Output, <V as Mul<V>>::Output>;

    #[inline]
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

        #[inline]
        fn mul(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as AddDim<Dr>>::Output, <Vl as Mul<Vr>>::Output> {
            Dim(self.0 * rhs.0, PhantomData)
        }
}

/// Scalar multiplication (with scalar on RHS)!
impl<D, V, RHS> Mul<RHS> for Dim<D, V>
    where D: Dimension, V: Mul<RHS>, RHS: NotDim {
        type Output = Dim<D, <V as Mul<RHS>>::Output>;
        #[inline]
        fn mul(self, rhs: RHS) -> Dim<D, <V as Mul<RHS>>::Output> {
            Dim(self.0 * rhs, PhantomData)
        }
    }

// fixme: make more generic if possible
/// Scalar multiplication (with scalar on LHS)!
#[macro_export]
macro_rules! dim_lhs_mult {
    ($t: ty) => (
        impl<D> Mul<Dim<D, $t>> for $t
            where D: Dimension {
                type Output = Dim<D, <$t as Mul>::Output>;
                #[inline]
                fn mul(self, rhs: Dim<D, $t>) -> Self::Output {
                    Dim( self * rhs.0, PhantomData )
                }
            }
        );
}
dim_lhs_mult!(f64);
dim_lhs_mult!(f32);


/// Dividing! Dimensions must be able to subtract.
impl<Dl, Dr, Vl, Vr> Div<Dim<Dr, Vr>> for Dim<Dl, Vl>
    where Dl: Dimension + SubDim<Dr>, Dr: Dimension, Vl: Div<Vr>, <Dl as SubDim<Dr>>::Output: Dimension {
        type Output = Dim<<Dl as SubDim<Dr>>::Output, <Vl as Div<Vr>>::Output>;
        #[inline]
        fn div(self, rhs: Dim<Dr, Vr>) -> Dim<<Dl as SubDim<Dr>>::Output, <Vl as Div<Vr>>::Output> {
            Dim(self.0 / rhs.0, PhantomData)
        }
    }

/// Scalar division (with scalar on RHS)!
impl<D, V, RHS> Div<RHS> for Dim<D, V>
    where D: Dimension, V: Div<RHS>, RHS: NotDim {
        type Output = Dim<D, <V as Div<RHS>>::Output>;
        #[inline]
        fn div(self, rhs: RHS) -> Dim<D, <V as Div<RHS>>::Output> {
            Dim(self.0 / rhs, PhantomData)
        }
    }

/// Scalar division (with scalar on LHS)!
#[macro_export]
macro_rules! dim_lhs_div {
    ($t: ty) => (
        impl<D> Div<Dim<D, $t>> for $t
            where D: Dimension + NegDim, <D as NegDim>::Output: Dimension {
                type Output = Dim<<D as NegDim>::Output, <$t as Div>::Output>;
                #[inline]
                fn div(self, rhs: Dim<D, $t>) -> Self::Output {
                    Dim( self / rhs.0, PhantomData )
                }
            }
        );
}
dim_lhs_div!(f64);
dim_lhs_div!(f32);


// Unary operators:
#[macro_export]
#[macro_use(dim_unary)]
macro_rules! dim_unary {
    ($Trait:ident, $op: ident, $($fun:ident),*) => (
        impl<D, V> $Trait for Dim<D, V>
            where D: $op<D>, V: $Trait, <D as $op<D>>::Output: Dimension {
                type Output = Dim<<D as $op<D>>::Output, <V as $Trait>::Output>;
                #[inline]
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
                #[inline]
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

// fixme: figure this out
// impl<'a, D, V, I> Index<I> for Dim<D, V>
//     where D: Dimension, V: Index<I> + 'a, <V as Index<I>>::Output: Sized {
//         type Output = Dim<D, &'a <V as Index<I>>::Output>;
//         fn index<'b>(&'b self, _index: &I) -> &'b Dim<D, &'a <V as Index<I>>::Output> {
//             &Dim((self.0).index(_index))
//         }
//     }

//------------------------------------------------------------------------------
// Casting
macro_rules! cast_from {
    ($fun:ident, $prim:ident) => (
        #[inline]
        fn $fun(n: $prim) -> Option<Self> {
            match FromPrimitive::$fun(n) {
                Some(v) => Some( Dim(v, PhantomData) ),
                None => None
            }
        }
        );
}

impl<D, V> FromPrimitive for Dim<D, V> where D: Dimension, V: FromPrimitive {
    cast_from!(from_i64, i64);
    cast_from!(from_u64, u64);
    cast_from!(from_isize, isize);
    cast_from!(from_i8, i8);
    cast_from!(from_i16, i16);
    cast_from!(from_i32, i32);
    cast_from!(from_usize, usize);
    cast_from!(from_u8, u8);
    cast_from!(from_u32, u32);
    cast_from!(from_f32, f32);
    cast_from!(from_f64, f64);
}

macro_rules! cast_to {
    ($fun:ident, $prim:ident) => (
        #[inline]
        fn $fun(&self) -> Option<$prim> {
            (self.0).$fun()
        }
        );
}

impl<D, V> ToPrimitive for Dim<D, V> where D: Dimension, V: ToPrimitive {
    cast_to!(to_i64, i64);
    cast_to!(to_u64, u64);
    cast_to!(to_isize, isize);
    cast_to!(to_i8, i8);
    cast_to!(to_i16, i16);
    cast_to!(to_i32, i32);
    cast_to!(to_usize, usize);
    cast_to!(to_u8, u8);
    cast_to!(to_u16, u16);
    cast_to!(to_u32, u32);
    cast_to!(to_f32, f32);
    cast_to!(to_f64, f64);
}

impl<D, V> NumCast for Dim<D, V> where D: Dimension, V: NumCast {
    #[inline]
    fn from<N>(n: N) -> Option<Self> where N: ToPrimitive {
        match NumCast::from(n) {
            Some(v) => Some(Dim(v, PhantomData)),
            None => None
        }
    }
}

//------------------------------------------------------------------------------
// DIMENSIONLESS THINGS HERE
//------------------------------------------------------------------------------


//------------------------------------------------------------------------------
// Zero and One
// impl<D, V> ::num::traits::Zero for Dim<D, V> where D: Dimension + KeepDim<D>, V: ::num::traits::Zero, <D as KeepDim<D>>::Output: Dimension {
//     fn zero() -> Self {
//         Dim(V::zero())
//     }
// }


//------------------------------------------------------------------------------
// Num
// impl<D, V> Num for Dim<D, V>
//     where D: Dimensionless + KeepDim<D>, V: Float, <D as KeepDim<D>>::Output: Dimensionless {
//         type FromStrRadixErr = Dim<D, <V as Num>::FromStrRadixErr>;
//         fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
//             Dim( <V as Num>::from_str_radix(str, radix));
//         }
//     }
//------------------------------------------------------------------------------
// Float
// macro_rules! dim_unary_float {
//     ($fun:ident, $returns:ty) => (
//         fn $fun(self) -> $returns { Dim( (self.0).$fun(), PhantomData) }
//         )
// }

// impl<D, V> Float for Dim<D, V>
//     where D: Dimensionless + KeepDim<D>, V: Float, <D as KeepDim<D>>::Output: Dimensionless {
//         // fn nan(self) -> Dim<D, V> {Dim ( (self.0).nan() )}
//         dim_unary_float!(nan, Self);
//         dim_unary_float!(infinity, Self);
//         dim_unary_float!(neg_infinity, Self);
//         dim_unary_float!(neg_zero, Self);
//         dim_unary_float!(min_value, Self);
//         //dim_unary_float!(min_positive_value, Self);
//         dim_unary_float!(max_value, Self);
//         dim_unary_float!(is_nan, bool);
//         dim_unary_float!(is_infinite, bool);
//         dim_unary_float!(is_finite, bool);
//         dim_unary_float!(is_normal, bool);
//         // dim_unary_float!(classify, FpCategory);
//         dim_unary_float!(floor, Self);
//         dim_unary_float!(ceil, Self);
//         dim_unary_float!(round, Self);
//         dim_unary_float!(trunc, Self);
//         dim_unary_float!(fract, Self);
//         dim_unary_float!(abs, Self);
//         dim_unary_float!(signum, Self);
//         dim_unary_float!(is_sign_positive, bool);
//         dim_unary_float!(is_sign_negative, bool);
//         // dim_unary_float!(mul_add, bool); BINARY

//         dim_unary_float!(recip, Self);
//         // powi
//         // powf
//         dim_unary_float!(sqrt, Self);
//         dim_unary_float!(exp, Self);
//         dim_unary_float!(exp2, Self);
//         dim_unary_float!(ln, Self);
//         dim_unary_float!(log, Self);
//         dim_unary_float!(log2, Self);
//         dim_unary_float!(log10, Self);
//         dim_unary_float!(max, Self);
//         dim_unary_float!(min, Self);
//         // abs_sub
//         dim_unary_float!(cbrt, Self);
//         dim_unary_float!(hypot, Self);
//         dim_unary_float!(sin, Self);
//         dim_unary_float!(cos, Self);
//         dim_unary_float!(tan, Self);
//         dim_unary_float!(asin, Self);
//         dim_unary_float!(acos, Self);
//         dim_unary_float!(atan, Self);
//         dim_unary_float!(atan2, Self);
//         dim_unary_float!(sin_cos, (Self, Self));
//         dim_unary_float!(exp_m1, Self);
//         dim_unary_float!(ln_1p, Self);
//         dim_unary_float!(sinh, Self);
//         dim_unary_float!(cosh, Self);
//         dim_unary_float!(tanh, Self);
//         dim_unary_float!(asinh, Self);
//         dim_unary_float!(acosh, Self);
//         dim_unary_float!(atanh, Self);
//         dim_unary_float!(integer_decode, (u64, i16, i8));

//     }
