//! # dimensioned
//!
//! dimensioned** is a library for compile time type checking for arbitrary unit systems.
//!
//! For in depth tutorials, check [here](http://paholg.com/project/dimensioned).
//!
#![doc(html_logo_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_favicon_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]
#![warn(missing_docs)]

#![cfg_attr(feature = "nightly", feature(optin_builtin_traits))]
#![cfg_attr(not(feature="std"), feature(core_float, core_intrinsics))]

#![cfg_attr(not(feature="std"), no_std)]

/// Re-exports from std or core, which allow the `make_units` macro to work properly with or
/// without the std library.  These are not guaranteed to stay here, and you should import from
/// `core` or `std` directly, not from here.
#[cfg(feature = "std")]
pub mod reexported {
    pub use std::*;
}

/// Re-exports from std or core, which allow the `make_units` macro to work properly with or
/// without the std library.  These are not guaranteed to stay here, and you should import from
/// `core` or `std` directly, not from here.
#[cfg(not(feature="std"))]
pub mod reexported {
    pub use core::*;
}

pub extern crate typenum;

#[macro_use]
pub mod dim;
#[macro_use]
mod make_units;
pub mod unit_systems;

pub use typenum::Same;
pub use typenum::int::Integer;
pub use typenum::consts::{N9, N8, N7, N6, N5, N4, N3, N2, N1, Z0, P1, P2, P3, P4, P5, P6, P7, P8,
                          P9};

pub use dim::*;
pub use unit_systems::{si, cgs, mks};
