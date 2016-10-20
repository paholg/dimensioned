extern crate typenum;
#[macro_use]
extern crate dimensioned;

use vector3a::Vector3;

use typenum::Same;
use std::ops::{Mul};
use dimensioned::si::{one, m, kg, s};
use dimensioned::{Dim, Dimension};

dim_impl_unary!(Norm, norm, Same, Vector3 => f64);
dim_impl_unary!(Norm2, norm2, Mul, Vector3 => f64);

dim_impl_binary!(Dot, dot, Mul, Vector3 => f64);
dim_impl_binary!(Cross, cross, Mul, Vector3 => Vector3);

#[cfg(feature = "nightly")]
fn main() {
    let xhat = one * Vector3::new(1.0, 0.0, 0.0);
    let yhat = one * Vector3::new(0.0, 1.0, 0.0);
    let zhat = one * Vector3::new(0.0, 0.0, 1.0);


    let start = -22.0*xhat*m + 5.0*yhat*m + 6.0*zhat*m;
    println!("A physicist was standing on a hill at position {}.", start);

    let end = 26.0*xhat*m - 19.0*yhat*m;
    println!("Then she walked down the hill to {}.", end);

    let displace = end - start;
    println!("So, her displacement vector was {}.", displace);

    let time = 30.0*s;
    println!("The walk took her {}.", time);

    let velocity = displace/time;
    println!("She must have had an average velocity of {}.", velocity);

    let speed = velocity.norm();
    println!("So, her average speed was {}.", speed);

    let center = 28.0*xhat*m - 21.0*yhat*m;
    println!("Now, she's standing next to a merry-go-round, centered at {}.", center);

    let force = 500.0*xhat*kg*m/s/s;
    println!("She decides to spin it, pushing with a force of {}", force);

    let r = end - center;
    let torque = r.cross(force);
    println!("That's a torque of {}!", torque);
}

#[cfg(not(feature = "nightly"))]
fn main() {
}




mod vector3a {
    use std::ops::{Add, Sub, Mul, Div};
    use std::fmt::{self, Display};


    #[derive(Copy, Clone, PartialEq, PartialOrd)]
    pub struct Vector3 {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vector3 {
        #[inline]
        pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
            Vector3{ x: x, y: y, z: z}
        }

        #[inline]
        pub fn cross(self, rhs: Vector3) -> Vector3 {
            Vector3{ x: self.y*rhs.z - self.z*rhs.y, y: self.z*rhs.x - self.x*rhs.z, z: self.x*rhs.y - self.y*rhs.x }
        }
        #[inline]
        pub fn dot(self, rhs: Vector3) -> f64 {
            self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
        }
        #[inline]
        pub fn norm2(self) -> f64 {
            self.dot(self)
        }
        #[inline]
        pub fn norm(self) -> f64 {
            self.norm2().sqrt()
        }
    }

    impl Add<Vector3> for Vector3 {
        type Output = Vector3;
        #[inline]
        fn add(self, rhs: Vector3) -> Self::Output {
            Vector3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
        }
    }
    impl Sub<Vector3> for Vector3 {
        type Output = Vector3;
        #[inline]
        fn sub(self, rhs: Vector3) -> Self::Output {
            Vector3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
        }
    }
    impl Mul<f64> for Vector3 {
        type Output = Vector3;
        #[inline]
        fn mul(self, rhs: f64) -> Self::Output {
            Vector3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
        }
    }
    impl Mul<Vector3> for f64 {
        type Output = Vector3;
        #[inline]
        fn mul(self, rhs: Vector3) -> Self::Output {
            Vector3{ x: self * rhs.x, y: self * rhs.y, z: self * rhs.z }
        }
    }
    impl Div<f64> for Vector3 {
        type Output = Vector3;
        #[inline]
        fn div(self, rhs: f64) -> Self::Output {
            Vector3{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
        }
    }

    impl Display for Vector3 {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }
}

