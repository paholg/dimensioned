//! Marker traits for various dimensions.
//!
//! The traits in this module can be useful for writing code that is generic with regards to unit
//! system, but involves specific dimensions.
//!
//! Once specialization is stabilized, many of these should be implemented automatically. For
//! example, implementing `Length` and `Time` for your unit system should cause `Speed` to be
//! implemented automatically.
//!
//! # Example
//! ```rust
//! extern crate dimensioned as dim;
//!
//!
//!
//!
//!
//!
//!
//! ```

use Dimensioned;

pub trait Length: Dimensioned {}
pub trait Area: Dimensioned {}
pub trait Volume: Dimensioned {}

pub trait Mass: Dimensioned {}
pub trait Time: Dimensioned {}
pub trait Temperature: Dimensioned {}

pub trait LuminousIntensity: Dimensioned {}

pub trait Speed: Dimensioned {}
pub trait Acceleration: Dimensioned {}

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
