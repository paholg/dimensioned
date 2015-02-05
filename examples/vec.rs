extern crate dimensioned;

use dimensioned::Scalar;
use dimensioned::si::*;
use std::ops::*;
use std::fmt;
use std::num::Float;

pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

impl Vector2d {
    /// Standard dot product.
    pub fn dot(&self, v: Vector2d) -> f64 { self.x*v.x + self.y*v.y }
    /// The wedge product and cross product are identical for 2d vectors.
    pub fn wedge(&self, v: Vector2d) -> f64 { self.x*v.y - self.y*v.x }
    /// Standard cross product.
    pub fn cross(&self, v: Vector2d) -> f64 { self.wedge(v) }
    /// Returns the squared norm.
    pub fn norm2(&self) -> f64 { self.x*self.x + self.y*self.y }
    /// Returns the norm.
    pub fn norm(&self) -> f64 { self.norm2().sqrt() }
    /// Returns a normalized 2d vector parallel to self.
    pub fn normalized(&self) -> Vector2d {
        let n = self.norm();
        Vector2d{x: self.x/n, y: self.y/n}
    }
}
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

impl<T> Mul<T> for Vector2d {
    type Output = <Vector2d as Mul<T>>::Output;
    fn mul(self, scalar: T) -> <Vector2d as Mul<T>>::Output {
        Vector2d{x: scalar*self.x, y: scalar*self.y}
    }
}


impl Div<f64> for Vector2d {
    type Output = Vector2d;
    fn div(self, scalar: f64) -> Vector2d { Vector2d{x: self.x/scalar, y: self.y/scalar} }
}
impl fmt::Display for Vector2d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "({}, {})", self.x, self.y) }
}

impl Scalar for Vector2d {}

fn main() {
    let pos = Vector2d{x: 1.3, y: 2.5}*m;
}
