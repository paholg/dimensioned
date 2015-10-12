/*!
# dimensioned

**dimensioned** is a library for compile time type checking for arbitrary unit systems.

For in depth tutorials, check [here](http://paholg.com/dimensioned).
 */
#![doc(html_logo_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_favicon_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]
#![warn(missing_docs)]

#![feature(optin_builtin_traits, zero_one, const_fn)]
#![feature(type_macros)]


extern crate peano;
extern crate num;


pub use dimensioned::*;

#[macro_use]
pub mod dimensioned;
#[macro_use]
mod make_units;

pub mod si;
pub mod cgs;
