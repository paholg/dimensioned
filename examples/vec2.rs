#[macro_use]
extern crate dimensioned;
// use dimensioned::{Dim, Wrap, Dimension};
use dimensioned::dimensioned::*;
//use dimensioned::*;
use dimensioned::u::*;
use std::ops::*;
use std::fmt;
use std::num::Float;
//use peano::*;

pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    fn norm(self) -> f64 { self.norm2().sqrt() }
    fn norm2(self) -> f64 { self.dot(self) }
    fn normalize(self) -> Vector2d { self / self.norm() }
    fn cross(self, rhs: Vector2d) -> f64 { self.x*rhs.y - self.y*rhs.x }
    fn dot(self, rhs: Vector2d) -> f64 { self.x*rhs.x + self.y*rhs.y }
}

trait VectorNorm {
    type Output;
    fn norm(self) -> <Self as VectorNorm>::Output;
}

impl<T: Dimension> VectorNorm for Dim<T, Vector2d> {
    type Output = Dim<T, f64>;
    fn norm(self) -> Dim<T, f64> { self.wrap( Vector2d::norm(self.0) ) }
}

macro_rules! wrap_member {
    ($Trait:ident, $DimOp:ident, $V:ident, $Out:ident, $fun:ident(self, $($arg:ident),*) ) => (
        pub trait $Trait {
            type Output;
            fn $fun(self $(, $arg)*) -> <Self as $Trait>::Output;
        }
        impl<T: Dimension> $Trait for Dim<T, $V> {
            type Output = Dim<>
        }
        )
}


// dim_unary!(VectorNorm, KeepDim, norm);
//wrap_binary!(Trait, DimOp, Self, Output, flist)

// macro_rules! wrap_binary {
//     ($Trait:ident, $DimOp:ident, $V:ident, $Out:ident, $($fun:ident),*) => (
//         pub trait $Trait<RHS = Self> {
//             type Output;
//             $(fn $fun(self, rhs: RHS) -> <Self as $Trait<RHS>>::Output;)*
//         }
//         impl<RHS> $Trait<RHS> for $V {
//             type Output = <$V as $Trait<RHS>>::Output;
//             $(fn $fun(self, rhs: RHS) -> <Self as $Trait<RHS>>::Output { self.fun(rhs) } )*
//         }
//         dim_binary!($Trait, $DimOp $(, $fun)*);
//         )
// }

// macro_rules! wrapper {
// pub trait $
//     fn $fun -> $Output;
// }
//wrap_binary!(VectorProd, AddDim, Vector2d, f64, dot, cross);
// dim_binary!(VectorProd, AddDim, cross, dot);

// macro_rules! dim_unary {
//     ($Trait: ident, $op: ident, $($fun: ident),*) => (
//         impl<T, V> $Trait for Dim<T, V>
//             where T: $op<T>, V: $Trait, <T as $op<T>>::Output: Dimension {
//                 type Output = Dim<<T as $op<T>>::Output, <V as $Trait>::Output>;
//                 $(fn $fun(self) -> Dim<<T as $op<T>>::Output, <V as $Trait>::Output> {
//                     Dim( (self.0).$fun() )
//                 })*
//             }
//         )
// }





impl Copy for Vector2d {}
impl Add for Vector2d {
    type Output = Vector2d;
    fn add(self, v: Vector2d) -> Vector2d { Vector2d{x: self.x + v.x, y: self.y + v.y} }
}
impl Sub for Vector2d {
    type Output = Vector2d;
    fn sub(self, v: Vector2d) -> Vector2d { Vector2d{x: self.x - v.x, y: self.y - v.y} }
}
impl Neg for Vector2d {
    type Output = Vector2d;
    fn neg(self) -> Vector2d { Vector2d{x: -self.x, y: -self.y} }
}

impl Mul<f64> for Vector2d {
    type Output = Vector2d;
    fn mul(self, rhs: f64) -> Vector2d {
        Vector2d{x: rhs*self.x, y: rhs*self.y}
    }
}

impl Div<f64> for Vector2d {
    type Output = Vector2d;
    fn div(self, scalar: f64) -> Vector2d { Vector2d{x: self.x/scalar, y: self.y/scalar} }
}
impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}


fn main() {
    let xhat: Dim<Unitless, Vector2d> = Dim::new(Vector2d{x: 1.0, y: 0.0});
    let yhat: Dim<Unitless, Vector2d> = Dim::new(Vector2d{x: 1.0, y: 0.0});

    let v = Vector2d{x: 1.1, y: 1.1};
    v.norm();

    let x = unit;

    println!("x: {}", x );
    // println!("square? {}", x.pow2() );

    xhat.norm();
}
