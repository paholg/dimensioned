//! This module contains the unit systems that dimensioned ships with

mod conversion;

include!(concat!(env!("OUT_DIR"), "/si.rs"));
include!(concat!(env!("OUT_DIR"), "/ucum.rs"));
pub mod cgs;
pub mod mks;
