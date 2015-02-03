#![allow(non_upper_case_globals)]
// use si::SI;
use peano::*;
use std::ops::{Add, Sub, Mul, Div};

pub trait Dim {}

pub type Unitless = (Zero, Zero, Zero, Zero, Zero, Zero, Zero);
pub type Length = (One, Zero, Zero, Zero, Zero, Zero, Zero);
pub type Mass = (Zero, One, Zero, Zero, Zero, Zero, Zero);
pub type Time = (Zero, Zero, One, Zero, Zero, Zero, Zero);
pub type Current = (Zero, Zero, Zero, One, Zero, Zero, Zero);
pub type Temperature = (Zero, Zero, Zero, Zero, One, Zero, Zero);
pub type Intensity = (Zero, Zero, Zero, Zero, Zero, One, Zero);
pub type Quantity = (Zero, Zero, Zero, Zero, Zero, Zero, One);

impl Dim for Unitless {}
impl Dim for Length {}
impl Dim for Mass {}
impl Dim for Time {}
impl Dim for Current {}
impl Dim for Temperature {}
impl Dim for Quantity {}
impl Dim for Intensity {}

pub trait AddDim<Rhs> {
    type Result;
}

// fixme: figure out how to do this
// impl<T: Dim, Rhs: Dim> AddDim<Rhs> for T {
//     type Result = 
// }

pub struct Dimensioned<T: Dim>(f64);

impl<T: Dim> Copy for Dimensioned<T> {}

impl<T: Dim> Add for Dimensioned<T> {
    type Output = Dimensioned<T>;
    fn add(self, rhs: Dimensioned<T>) -> Dimensioned<T> {
        Dimensioned(self.0 + rhs.0)
    }
}

impl<T: Dim> Sub for Dimensioned<T> {
    type Output = Dimensioned<T>;
    fn sub(self, rhs: Dimensioned<T>) -> Dimensioned<T> {
        Dimensioned(self.0 - rhs.0)
    }
}

// impl<T: Dim, U: Dim> Mul<Dimensioned<U>> for Dimensioned<T> {
//     type Output = Dimensioned<<T as AddDim<U>>::Result>;
//     fn mul(self, rhs: Dimensioned<U>) -> Dimensioned<<T as AddDim<U>>::Result> {
//         Dimensioned(self.0 * rhs.0)
//     }
// }


// fixme: pick more unique names? Preface all constants with a unit system?
pub static m: Dimensioned<Length> = Dimensioned(1.0);
pub static g: Dimensioned<Mass> = Dimensioned(1.0);
pub static s: Dimensioned<Time> = Dimensioned(1.0);
pub static A: Dimensioned<Current> = Dimensioned(1.0);
pub static K: Dimensioned<Temperature> = Dimensioned(1.0);
pub static cd: Dimensioned<Intensity> = Dimensioned(1.0);
pub static mol: Dimensioned<Quantity> = Dimensioned(1.0);

#[test]
fn unit_arithmetic() {
    let b = m + m;
    let c = m + g;
}
