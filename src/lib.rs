/*!
# dimensioned

**dimensioned** is a library for compile time type checking for arbitrary unit systems.

For in depth tutorials, check [here](http://paholg.com/project/dimensioned).
 */
#![doc(html_logo_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_favicon_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]
#![warn(missing_docs)]

#![feature(optin_builtin_traits, const_fn)]
#![feature(type_macros)]


pub extern crate typenum;

#[macro_use]
pub mod dim;
#[macro_use]
mod make_units;
pub mod unit_systems;

pub use typenum::Same;
pub use typenum::int::Integer;
pub use typenum::consts::{N9, N8, N7, N6, N5, N4, N3, N2, N1, Z0, P1, P2, P3, P4, P5, P6, P7, P8, P9};

pub use dim::*;
pub use unit_systems::{si, cgs, mks};

