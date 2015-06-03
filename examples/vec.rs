#[macro_use]
extern crate dimensioned;
extern crate nalgebra;

use dimensioned::dimensioned::*;

use std::ops::{Add, Sub, Mul, Div};
use nalgebra::Vec2;

make_units! {
    SI, One;
    base {
        Meter, m, m;
        Kilogram, kg, kg;
        Second, s, s;
        Ampere, amp, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
        Newton(newton) = Kilogram * Meter / Second / Second;
    }
}

// trait Boob {}

// impl<D: Dimension, V> Boob for Dim<D, V> {}

// impl<D: Boob> Div<Dim<D, f64>> for Vec2<f64> {
//     type Output = Vec2<<f64 as Div<Dim<D, f64>>>::Output>;

//     #[inline]
//     fn div(self, right: Dim<D, f64>) -> Self::Output {
//         Vec2::new(self.x / right, self.y / right)
//     }
// }


fn main() {
    let x = Vec2::new(1.0*m, 3.0*m);

    let t = 5.0*s;

    // let v = x / t;

    // v / second;

    // println!("dot: {}, norm2: {}", x.dot(y), x.norm2());
    // let xhat = Vec2::new(1.0, 0.0)*meter;

//     let xhat: Dim<Unitless, Vec2<f64>> = Dim::new(Vec2{x: 1.0, y: 0.0});
//     let yhat: Dim<Unitless, Vec2<f64>> = Dim::new(Vec2{x: 1.0, y: 0.0});

//     let start = -xhat*13.0*meter + yhat*33.0*meter;
//     let end = xhat*26.0*meter - yhat*19.0*meter;

//     let displace = end - start;
//     let time = second*26.0;
//     let vel = displace/time;
//     // Because we put norm() in a trait and implemented it for both Vector2d and Dim,
//     // calling vel.norm() works as we want it to (returning Dim<Meter, ff64>). This is
//     // the recommended way of accessing values inside a Dim.
//     let speed = vel.norm();
//     // Had we been unable or unwilling to implement norm() inside a trait, we could have
//     // achieved the same behavior using the wrap() function, as follows:
//     let speed2 = vel.wrap((vel.0).norm());
//     println!("
// A physicist was standing at {}.
// Then she walked to {}, for a displacement of {}.
// The walk took her {}, so she must have had a velocity of {}.
// That's a speed of {}! Again, that's {}!", start, end, displace, time, vel, speed, speed2);

//     let center = xhat*meter*24.0 - yhat*meter*17.0;
//     let force = xhat*500.0*kilogram*meter/second/second;
//     let r = end-center;
//     println!("
// Now, she's standing next to a merry-go-round, centered at {}.
// That is {} away from her. She decides to spin it, pushing with a force of {}.
// That's a torque of {}!", center, r.norm(), force, r.cross(force));
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Vector2d<N> {
    x: N,
    y: N,
}

pub trait Dot<N = Self> {
    type Output;
    fn dot(self, rhs: N) -> Self::Output;
}

impl<N, M> Dot<Vector2d<N>> for Vector2d<M> where M: Mul<N>, <M as Mul<N>>::Output: Add {
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

// pub trait Norm {
//     type Output;
//     fn norm(self) -> Self::Output;
// }

// impl<N> Norm for Vector2d<N> where N: Norm2 {
//     type Output = <Vector2d<N> as Norm2>::Output;
//     #[inline]
//     fn norm(self) -> Self::Output { self.norm2().sqrt() }
// }


// impl<N: Mul<M>, M> Vector2d<N> {
//     pub fn dot(self, rhs: Vector2d<M>) -> <N as Mul<M>>::Output { self.x*rhs.x }
// }
