#[macro_use]
extern crate dimensioned;
extern crate num;

use dimensioned::Sqrt;
use dimensioned::si::{one, m, kg, s};

use num::traits::Float;
use std::ops::{Add, Sub, Mul, Div};
use std::fmt::{self, Display};


fn main() {
    let xhat = Vector2d::new(one, 0.0*one);
    let yhat = Vector2d::new(0.0*one, one);

    // fixable?
    // m * xhat;

    let start = -13.0 * xhat * m + 33.0 * yhat * m;
    let end = 26.0 * xhat * m - 19.0 * yhat * m;

    let displace = end - start;
    let time = 26.0 * s;
    let vel = displace/time;


    let speed = vel.norm2(); // fixme: This should use sqrt() but there's an error
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}!", start, end, displace, time, vel, speed);

    let center = 24.0 * xhat * m - 17.0 * yhat * m;
    let force = 500.0 * xhat * kg*m/s/s;
    let r = end - center;
    println!("
Now, she's standing next to a merry-go-round, centered at {}.
That is {} away from her. She decides to spin it, pushing with a force of {}.
That's a torque of {}!", center, r.norm2(), force, r.cross(force)); // fixme: same error with sqrt()
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector2d<N> {
    x: N,
    y: N,
}

impl<N> Vector2d<N> {
    #[inline]
    fn new(x: N, y: N) -> Vector2d<N> { Vector2d{ x: x, y: y} }
}

pub trait Dot<N = Self> {
    type Output;
    fn dot(self, rhs: N) -> Self::Output;
}

impl<M, N> Dot<Vector2d<N>> for Vector2d<M> where M: Mul<N>, <M as Mul<N>>::Output: Add {
    type Output = <<M as Mul<N>>::Output as Add>::Output;
    #[inline]
    fn dot(self, rhs: Vector2d<N>) -> Self::Output { self.x*rhs.x + self.y*rhs.y }
}

pub trait Norm2 {
    type Output;
    fn norm2(self) -> Self::Output;
}

impl<N> Norm2 for Vector2d<N> where Vector2d<N>: Dot + Copy {
    type Output = <Vector2d<N> as Dot>::Output;
    #[inline]
    fn norm2(self) -> Self::Output { self.dot(self) }
}

pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

impl<N> Norm for Vector2d<N> where Vector2d<N>: Norm2, <Vector2d<N> as Norm2>::Output: Float {
    type Output = <Vector2d<N> as Norm2>::Output;
    #[inline]
    fn norm(self) -> Self::Output { self.norm2().sqrt() }
}

pub trait Cross<N> {
    type Output;
    fn cross(self, rhs: N) -> Self::Output;
}

impl<M, N> Cross<Vector2d<N>> for Vector2d<M> where M: Mul<N>, <M as Mul<N>>::Output: Sub<<M as Mul<N>>::Output> {
    type Output = <<M as Mul<N>>::Output as Sub<<M as Mul<N>>::Output>>::Output;
    #[inline]
    fn cross(self, rhs: Vector2d<N>) -> Self::Output { self.x*rhs.y - self.y*rhs.x }
}

impl<M, N> Add<Vector2d<N>> for Vector2d<M> where M: Add<N> {
    type Output = Vector2d<<M as Add<N>>::Output>;
    #[inline]
    fn add(self, rhs: Vector2d<N>) -> Self::Output { Vector2d{ x: self.x + rhs.x, y: self.y + rhs.y } }
}

impl<M, N> Sub<Vector2d<N>> for Vector2d<M> where M: Sub<N> {
    type Output = Vector2d<<M as Sub<N>>::Output>;
    #[inline]
    fn sub(self, rhs: Vector2d<N>) -> Self::Output { Vector2d{ x: self.x - rhs.x, y: self.y - rhs.y } }
}

/// Scalar multiplication
impl<N, T> Mul<T> for Vector2d<N> where N: Mul<T>, T: Copy {
    type Output = Vector2d<<N as Mul<T>>::Output>;
    #[inline]
    fn mul(self, rhs: T) -> Self::Output { Vector2d{ x: self.x * rhs, y: self.y * rhs } }
}

/// Scalar multiplication with the scalar on the left. As far as I know, there is not a generic way to do this.
impl<N: Mul<f64>> Mul<Vector2d<N>> for f64 {
    type Output = Vector2d<<N as Mul<f64>>::Output>;
    #[inline]
    fn mul(self, rhs: Vector2d<N>) -> Self::Output { rhs * self }
}

/// Scalar division
impl<N, T> Div<T> for Vector2d<N> where N: Div<T>, T: Copy {
    type Output = Vector2d<<N as Div<T>>::Output>;
    #[inline]
    fn div(self, rhs: T) -> Self::Output { Vector2d{ x: self.x / rhs, y: self.y / rhs } }
}

impl<N: Display> Display for Vector2d<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)
    }
}
