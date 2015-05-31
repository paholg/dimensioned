#[macro_use]
extern crate dimensioned;

use dimensioned::dimensioned::*;
use std::ops::*;
use std::fmt;

make_units! {
    SI, One;
    base {
        Meter, meter, m;
        Kilogram, kilogram, kg;
        Second, second, s;
        Ampere, ampere, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
    }
}

#[derive(Copy, Clone)]
pub struct Vector2d {
    pub x: f64,
    pub y: f64,
}

// Defining traits for vector functions, so we can use them inside a Dim nicely:
pub trait VectorNorm {
    type Output;
    fn norm (self) -> <Self as VectorNorm>::Output;
}
// Note that norm2 needs to be in a separate trait from norm() because they will return
// objects with different dimensions
pub trait VectorNorm2 {
    type Output;
    fn norm2 (self) -> <Self as VectorNorm2>::Output;
}
// In 2d, cross and dot have the same type signature, so we can put them in the same trait:
pub trait VectorProd<Rhs> {
    type Output;
    fn cross (self, rhs: Rhs) -> <Self as VectorProd<Rhs>>::Output;
    fn dot (self, rhs: Rhs) -> <Self as VectorProd<Rhs>>::Output;
}
pub trait VectorNormalize {
    type Output;
    fn normalize(self) -> <Self as VectorNormalize>::Output;
}
// Implementing vector functions:
impl VectorNorm for Vector2d {
    type Output = f64;
    fn norm(self) -> f64 { (self.x*self.x + self.y*self.y).sqrt() }
}
impl VectorNorm2 for Vector2d {
    type Output = f64;
    fn norm2(self) -> f64 { self.x*self.x + self.y*self.y }
}
impl VectorNormalize for Vector2d {
    type Output = Vector2d;
    fn normalize(self) -> Vector2d { self / self.norm() }
}
impl VectorProd<Vector2d> for Vector2d {
    type Output = f64;
    fn cross (self, rhs: Vector2d) -> f64 { self.x*rhs.y -self.y*rhs.x }
    fn dot (self, rhs: Vector2d) -> f64 { self.x*rhs.x + self.y*rhs.y }
}
// Using our helper macros to implement the traits for Dim. Note that these macros
// require the trait to have an associated Output type, and you get to specify how the
// dimensions interact.
// For unary functions, the Dimension operator (KeepDim, AddDim, etc.) acts on its caller
// KeepDim requires both operands to be the same dimensions, and doesn't change them
// AddDim sums the dimensional powers (m AddDim m/s == m^2/s)
// SubDim take the difference of dimensional powers (m SubDim m/s == s)
// The macro takes, in order: Trait name, Dimensional operation, list of functions.
dim_unary!(VectorNorm, KeepDim, norm);
dim_unary!(VectorNorm2, AddDim, norm2);
dim_unary!(VectorNormalize, SubDim, normalize);
dim_binary!(VectorProd, AddDim, cross, dot);

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

    let start = -xhat*13.0*meter + yhat*33.0*meter;
    let end = xhat*26.0*meter - yhat*19.0*meter;

    let displace = end - start;
    let time = second*26.0;
    let vel = displace/time;
    // Because we put norm() in a trait and implemented it for both Vector2d and Dim,
    // calling vel.norm() works as we want it to (returning Dim<Meter, ff64>). This is
    // the recommended way of accessing values inside a Dim.
    let speed = vel.norm();
    // Had we been unable or unwilling to implement norm() inside a trait, we could have
    // achieved the same behavior using the wrap() function, as follows:
    let speed2 = vel.wrap((vel.0).norm());
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}! Again, that's {}!", start, end, displace, time, vel, speed, speed2);

    let center = xhat*meter*24.0 - yhat*meter*17.0;
    let force = xhat*500.0*kilogram*meter/second/second;
    let r = end-center;
    println!("
Now, she's standing next to a merry-go-round, centered at {}.
That is {} away from her. She decides to spin it, pushing with a force of {}.
That's a torque of {}!", center, r.norm(), force, r.cross(force));
}
