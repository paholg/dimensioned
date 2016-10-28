//! # dimensioned
//!
//! `Dimensioned` is a library for compile time type checking for arbitrary unit systems.
//!
//! For in depth tutorials, check [here](http://paholg.com/project/dimensioned).
//!
#![doc(html_logo_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_favicon_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]
// #![warn(missing_docs)] fixme

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

#[macro_use]
pub extern crate typenum;

#[macro_use]
extern crate generic_array;

mod traits;

#[macro_use]
mod make_units;

pub mod unit_systems;
pub use unit_systems::*;

pub use traits::*;


pub mod array;
