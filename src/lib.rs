/*!
# dimensioned

**dimensioned** is a library for compile time type checking for arbitrary unit systems.

The best place to read about it is [here](http://paholg.com/dimensioned)
*/
#![warn(missing_docs)]

#![feature(optin_builtin_traits)]

extern crate num;


pub use dimensioned::*;

pub mod peano;
#[macro_use]
pub mod dimensioned;
#[macro_use]
mod make_units;

pub mod si;
pub mod cgs;
