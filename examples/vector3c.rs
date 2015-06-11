extern crate dimensioned;

use dimensioned::si::{Unitless, one, m, kg, s};
use vector3c::*;

fn main() {
    let xhat: Vector3<Unitless> = Vector3::new(1.0, 0.0, 0.0);
    let yhat: Vector3<Unitless> = Vector3::new(0.0, 1.0, 0.0);
    let zhat: Vector3<Unitless> = Vector3::new(0.0, 0.0, 1.0);


    // let start = -22.0*xhat*m + 5.0*yhat*m + 6.0*zhat*m;
    // println!("A physicist was standing on a hill at position {}.", start);

    // let end = 26.0*xhat*m - 19.0*yhat*m;
    // println!("Then she walked down the hill to {}.", end);

    // let displace = end - start;
    // println!("So, her displacement vector was {}.", displace);

    // let time = 30.0*s;
    // println!("The walk took her {}.", time);

    // let velocity = displace/time;
    // println!("She must have had an average velocity of {}.", velocity);

    // let speed = velocity.norm();
    // println!("So, her average speed was {}.", speed);

    // let center = 28.0*xhat*m - 21.0*yhat*m;
    // println!("Now, she's standing next to a merry-go-round, centered at {}.", center);

    // let force = 500.0*xhat*kg*m/s/s;
    // println!("She decides to spin it, pushing with a force of {}", force);

    // let r = end - center;
    // let torque = r.cross(force);
    // println!("That's a torque of {}!", torque);
}


mod vector3c {
    use std::ops::{Add, Sub, Mul, Div};
    use std::fmt::{self, Display};
    use std::marker::PhantomData;

    use dimensioned::{Dim, Dimension, MulDim, Sqrt, DimToString};

    #[derive(Copy, Clone)]
    pub struct Vector3<D: Dimension> {
        x: f64,
        y: f64,
        z: f64,
        p: PhantomData<D>
    }

    impl<D: Dimension> Vector3<D> {
        #[inline]
        pub fn new(x: f64, y: f64, z: f64) -> Vector3<D> {
            Vector3 { x: x, y: y, z: z, p: PhantomData }
        }
    }

    pub trait Cross<RHS = Self> {
        type Output;
        fn cross(self, rhs: RHS) -> Self::Output;
    }
    impl<Dl, Dr> Cross<Vector3<Dr>> for Vector3<Dl> where Dl: Dimension + MulDim<Dr>, Dr: Dimension, <Dl as MulDim<Dr>>::Output: Dimension {
        type Output = Vector3<<Dl as MulDim<Dr>>::Output>;
        #[inline]
        fn cross(self, rhs: Vector3<Dr>) -> Self::Output {
            Vector3::new(self.y*rhs.z - self.z*rhs.y,
                         self.z*rhs.x - self.x*rhs.z,
                         self.x*rhs.y - self.y*rhs.x)
        }
    }

    pub trait Dot<RHS = Self> {
        type Output;
        fn dot(self, rhs: RHS) -> Self::Output;
    }
    impl<Dl, Dr> Dot<Vector3<Dr>> for Vector3<Dl> where Dl: Dimension + MulDim<Dr>, Dr: Dimension, <Dl as MulDim<Dr>>::Output: Dimension {
        type Output = Dim<<Dl as MulDim<Dr>>::Output, f64>;
        #[inline]
        fn dot(self, rhs: Vector3<Dr>) -> Self::Output {
            Dim::new( self.x*rhs.x + self.y*rhs.y + self.z*rhs.z )
        }
    }

    pub trait Norm2: Dot {
        type Output;
        fn norm2(self) -> <Self as Norm2>::Output;
    }
    impl<D> Norm2 for Vector3<D> where Vector3<D>: Copy, D: Dimension + MulDim, <D as MulDim>::Output: Dimension {
        type Output = Dim<<D as MulDim>::Output, f64>;
        #[inline]
        fn norm2(self) -> <Self as Norm2>::Output {
            self.dot(self)
        }
    }

    pub trait Norm: Norm2 {
        type Output;
        fn norm(self) -> <Self as Norm>::Output;
    }
    impl<D> Norm for Vector3<D> where D: Dimension + MulDim + Copy, <D as MulDim>::Output: Dimension, Dim<<D as MulDim>::Output, f64>: Sqrt {
        type Output = <Dim<<D as MulDim>::Output, f64> as Sqrt>::Output;
        #[inline]
        fn norm(self) -> <Self as Norm>::Output {
            self.norm2().sqrt()
        }
    }

    impl<D: Dimension> Add<Vector3<D>> for Vector3<D> {
        type Output = Vector3<D>;
        #[inline]
        fn add(self, rhs: Vector3<D>) -> Self::Output {
            Vector3::new(self.x + rhs.x,
                         self.y + rhs.y,
                         self.z + rhs.z)
        }
    }
    impl<D: Dimension> Sub<Vector3<D>> for Vector3<D> {
        type Output = Vector3<D>;
        #[inline]
        fn sub(self, rhs: Vector3<D>) -> Self::Output {
            Vector3::new(self.x - rhs.x,
                         self.y - rhs.y,
                         self.z - rhs.z)
        }
    }
    impl<D: Dimension> Mul<f64> for Vector3<D> {
        type Output = Vector3<D>;
        #[inline]
        fn mul(self, rhs: f64) -> Self::Output {
            Vector3::new(self.x * rhs,
                         self.y * rhs,
                         self.z * rhs)
        }
    }
    impl<D: Dimension> Mul<Vector3<D>> for f64 {
        type Output = Vector3<D>;
        #[inline]
        fn mul(self, rhs: Vector3<D>) -> Self::Output {
            Vector3::new(self * rhs.x,
                         self * rhs.y,
                         self * rhs.z)
        }
    }
    impl<D: Dimension> Div<f64> for Vector3<D> {
        type Output = Vector3<D>;
        #[inline]
        fn div(self, rhs: f64) -> Self::Output {
            Vector3::new(self.x / rhs,
                         self.y / rhs,
                         self.z / rhs)
        }
    }

    impl<D: Dimension + DimToString> Display for Vector3<D> {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "({}, {}, {}) {}", self.x, self.y, self.z, D::to_string())
        }
    }
}
