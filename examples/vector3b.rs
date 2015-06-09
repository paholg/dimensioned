extern crate dimensioned as dim;
extern crate num;

use vector3b::{Vector3, Norm2, Cross};

use num::Float;

use dim::Sqrt;
use dim::si::{one, m, kg, s};

pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

impl<N> Norm for Vector3<N> where Vector3<N>: Norm2, <Vector3<N> as Norm2>::Output: Sqrt {
    type Output = <<Vector3<N> as Norm2>::Output as Sqrt>::Output;
    #[inline]
    fn norm(self) -> Self::Output { self.norm2().sqrt() }
}


fn main() {
    let xhat = Vector3::new(one, 0.0*one, 0.0*one);
    let yhat = Vector3::new(0.0*one, one, 0.0*one);
    let zhat = Vector3::new(0.0*one, 0.0*one, one);


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

mod vector3b {
    use num::traits::Float;
    use std::ops::{Add, Sub, Mul, Div};
    use std::fmt::{self, Display};


    #[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord)]
    pub struct Vector3<N> {
        x: N,
        y: N,
        z: N,
    }

    impl<N> Vector3<N> {
        #[inline]
        pub fn new(x: N, y: N, z: N) -> Vector3<N> { Vector3{ x: x, y: y, z: z} }
    }

    pub trait Dot<N = Self> {
        type Output;
        fn dot(self, rhs: N) -> Self::Output;
    }

    impl<M, N> Dot<Vector3<N>> for Vector3<M> where M: Mul<N>, <M as Mul<N>>::Output: Add, <<M as Mul<N>>::Output as Add>::Output: Add<<M as Mul<N>>::Output> {
        type Output = <<<M as Mul<N>>::Output as Add>::Output as Add<<M as Mul<N>>::Output>>::Output;
        #[inline]
        fn dot(self, rhs: Vector3<N>) -> Self::Output { self.x*rhs.x + self.y*rhs.y + self.z*rhs.z }
    }

    pub trait Norm2 {
        type Output;
        fn norm2(self) -> Self::Output;
    }

    impl<N> Norm2 for Vector3<N> where Vector3<N>: Dot + Copy {
        type Output = <Vector3<N> as Dot>::Output;
        #[inline]
        fn norm2(self) -> Self::Output { self.dot(self) }
    }

    pub trait Norm {
        type Output;
        fn norm(self) -> Self::Output;
    }

    impl<N> Norm for Vector3<N> where Vector3<N>: Norm2, <Vector3<N> as Norm2>::Output: Float {
        type Output = <Vector3<N> as Norm2>::Output;
        #[inline]
        fn norm(self) -> Self::Output { self.norm2().sqrt() }
    }

    pub trait Cross<N> {
        type Output;
        fn cross(self, rhs: N) -> Self::Output;
    }

    impl<M, N> Cross<Vector3<N>> for Vector3<M> where M: Mul<N> + Copy, N: Copy, <M as Mul<N>>::Output: Sub<<M as Mul<N>>::Output> {
        type Output = Vector3<<<M as Mul<N>>::Output as Sub<<M as Mul<N>>::Output>>::Output>;
        #[inline]
        fn cross(self, rhs: Vector3<N>) -> Self::Output { Vector3{ x: self.y*rhs.z - self.z*rhs.y, y: self.z*rhs.x - self.x*rhs.z, z: self.x*rhs.y - self.y*rhs.x } }
    }

    impl<M, N> Add<Vector3<N>> for Vector3<M> where M: Add<N> {
        type Output = Vector3<<M as Add<N>>::Output>;
        #[inline]
        fn add(self, rhs: Vector3<N>) -> Self::Output { Vector3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z } }
    }

    impl<M, N> Sub<Vector3<N>> for Vector3<M> where M: Sub<N> {
        type Output = Vector3<<M as Sub<N>>::Output>;
        #[inline]
        fn sub(self, rhs: Vector3<N>) -> Self::Output { Vector3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } }
    }

    /// Scalar multiplication
    impl<N, T> Mul<T> for Vector3<N> where N: Mul<T>, T: Copy {
        type Output = Vector3<<N as Mul<T>>::Output>;
        #[inline]
        fn mul(self, rhs: T) -> Self::Output { Vector3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs } }
    }

    /// Scalar multiplication with the scalar on the left. There is not a generic way to do
    /// this. Note that this requires multiplication by a scalar to be commutative!!!!!
    impl<N: Mul<f64>> Mul<Vector3<N>> for f64 {
        type Output = Vector3<<N as Mul<f64>>::Output>;
        #[inline]
        fn mul(self, rhs: Vector3<N>) -> Self::Output { rhs * self }
    }

    /// Scalar division
    impl<N, T> Div<T> for Vector3<N> where N: Div<T>, T: Copy {
        type Output = Vector3<<N as Div<T>>::Output>;
        #[inline]
        fn div(self, rhs: T) -> Self::Output { Vector3{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs } }
    }

    impl<N: Display> Display for Vector3<N> {
        fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
            write!(f, "({}, {}, {})", self.x, self.y, self.z)
        }
    }
}
