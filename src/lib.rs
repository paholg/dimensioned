#![feature(core)]
#![feature(unboxed_closures)]

extern crate num;

pub use dimensioned::*;

pub mod peano;
#[macro_use]
pub mod dimensioned;
pub mod si;
pub mod u;
