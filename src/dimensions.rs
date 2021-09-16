//! Marker traits for various dimensions
//!
//! The traits in this module can be useful for writing code that is generic with regards to unit
//! system, but involves a specific dimension.
//!
//! Where appropriate, these traits are implemented for the unit systems in this library.
//!
//! If you find it lacking, please feel free to submit an issue or pull request to add more.
//!
//! # Example
//! ```rust
//! extern crate dimensioned as dim;
//!
//! use dim::dimensions::{Length, Time};
//! use dim::typenum::Quot;
//! use std::ops::Div;
//!
//! fn speed<L, T>(dist: L, time: T) -> Quot<L, T>
//! where
//!     L: Length + Div<T>,
//!     T: Time,
//! {
//!     dist / time
//! }
//!
//! fn main() {
//!     use dim::si;
//!
//!     let x = 3.0 * si::M;
//!     let t = 2.0 * si::S;
//!     let v = speed(x, t);
//!
//!     assert_eq!(v, x / t);
//!
//!     // Compiler error:
//!     // speed(x, x);
//! }
//! ```

#![allow(missing_docs)]

use crate::Dimensioned;

pub trait Length: Dimensioned {}
pub trait ReciprocalLength: Dimensioned {}
pub trait Area: Dimensioned {}
pub trait Volume: Dimensioned {}

pub trait Mass: Dimensioned {}
pub trait Time: Dimensioned {}
pub trait Temperature: Dimensioned {}

pub trait LuminousIntensity: Dimensioned {}

pub trait Velocity: Dimensioned {}
pub trait Acceleration: Dimensioned {}
pub trait Jerk: Dimensioned {}

pub trait Charge: Dimensioned {}
pub trait Current: Dimensioned {}
pub trait ElectricPotential: Dimensioned {}
pub trait Capacitance: Dimensioned {}
pub trait Resistance: Dimensioned {}
pub trait Conductance: Dimensioned {}
pub trait MagneticFlux: Dimensioned {}
pub trait Inductance: Dimensioned {}

pub trait Frequency: Dimensioned {}

pub trait Force: Dimensioned {}
pub trait Pressure: Dimensioned {}
pub trait Energy: Dimensioned {}
pub trait Power: Dimensioned {}

pub trait AmountOfSubstance: Dimensioned {}
