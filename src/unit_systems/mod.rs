//! This module contains the unit systems that dimensioned ships with

mod conversion;

include!(concat!(env!("OUT_DIR"), "/si.rs"));
pub mod cgs;
pub mod mks;
pub mod ucum;
