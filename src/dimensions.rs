//! Marker traits for various dimensions.
//!
//! The traits in this module can be useful for writing code that is generic with regards to unit
//! system, but involves a specific dimension.
//!
//! Where appropriate, these traits are implemented for the unit systems in this library, but they
//! are not automatically implemented by the `make_units` macro.
//!
//! # Example
//! ```rust
//! extern crate dimensioned as dim;
//!
//! use dim::dimensions::Length;
//! use std::ops::Add;
//!
//! fn add_lengths<T: Length + Add<T, Output=T>>(a: T, b: T) -> T {
//!     a + b
//! }
//!
//! fn main() {
//!     use dim::si::{M, S};
//!
//!     let x = 3.0*M;
//!     let y = 1.0*M;
//!
//!     assert_eq!(x+y, add_lengths(x, y));
//!
//!     // Compiler error:
//!     // add_lengths(1.0*S, 2.0*S);
//! }
//! ```

use Dimensioned;

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
