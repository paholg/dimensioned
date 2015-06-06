// use std::ops::{Add, Sub, Mul, Div};
// use std::fmt::{self, Display};
// use std::marker::PhantomData;

// use dimensioned::{Dimension};


// #[derive(Copy, Clone, PartialEq, PartialOrd)]
// pub struct Vector3<D: Dimension> {
//     x: f64,
//     y: f64,
//     z: f64,
//     p: PhantomData<D>
// }

// impl<D: Dimension> Vector3 {
//     #[inline]
//     pub fn new(x: f64, y: f64, z: f64) -> Vector3<D> { Vector3{ x: x, y: y, z: z, p: PhantomData} }

//     #[inline]
//     pub fn cross(self, rhs: Vector3) -> Vector3 { Vector3::new(self.y*rhs.z - self.z*rhs.y, self.z*rhs.x - self.x*rhs.z, self.x*rhs.y - self.y*rhs.x ) }
//     #[inline]
//     pub fn dot(self, rhs: Vector3) -> f64 { self.x*rhs.x + self.y*rhs.y + self.z*rhs.z }
//     #[inline]
//     pub fn norm2(self) -> f64 { self.dot(self) }
//     #[inline]
//     pub fn norm(self) -> f64 { self.norm2().sqrt() }
// }

// impl Add<Vector3> for Vector3 {
//     type Output = Vector3;
//     #[inline]
//     fn add(self, rhs: Vector3) -> Self::Output { Vector3{ x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z } }
// }
// impl Sub<Vector3> for Vector3 {
//     type Output = Vector3;
//     #[inline]
//     fn sub(self, rhs: Vector3) -> Self::Output { Vector3{ x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z } }
// }
// impl Mul<f64> for Vector3 {
//     type Output = Vector3;
//     #[inline]
//     fn mul(self, rhs: f64) -> Self::Output { Vector3{ x: self.x * rhs, y: self.y * rhs, z: self.z * rhs } }
// }
// impl Mul<Vector3> for f64 {
//     type Output = Vector3;
//     #[inline]
//     fn mul(self, rhs: Vector3) -> Self::Output { Vector3{ x: self * rhs.x, y: self * rhs.y, z: self * rhs.z } }
// }
// impl Div<f64> for Vector3 {
//     type Output = Vector3;
//     #[inline]
//     fn div(self, rhs: f64) -> Self::Output { Vector3{ x: self.x / rhs, y: self.y / rhs, z: self.z / rhs } }
// }

// impl Display for Vector3 {
//     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
//         write!(f, "({}, {}, {})", self.x, self.y, self.z)
//     }
// }
