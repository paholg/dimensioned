//! # dimensioned
//!
//! `Dimensioned` is a library for compile time type checking for arbitrary unit systems.
//!
//! For in depth tutorials, check [here](http://paholg.com/project/dimensioned).
//!
#![doc(html_logo_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_favicon_url = "http://paholg.com/dimensioned/imgs/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]

// fixme #![warn(missing_docs)]

#![cfg_attr(feature = "oibit", feature(optin_builtin_traits))]
#![cfg_attr(not(feature="std"), feature(core_float, core_intrinsics))]

#![cfg_attr(not(feature="std"), no_std)]

// For clippy:
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(unknown_lints)]
#![deny(clippy)]
#![allow(type_complexity, len_without_is_empty)]

// Macro debugging
// #![feature(trace_macros)]
// trace_macros!(true);


/// Re-exports from std or core, which allow the `make_units` macro to work properly with or
/// without the std library.  These are not guaranteed to stay here, and you should import from
/// `core` or `std` directly, not from here.
#[doc(hidden)]
#[cfg(feature = "std")]
pub mod reexported {
    pub use std::*;
}

/// Re-exports from std or core, which allow the `make_units` macro to work properly with or
/// without the std library.  These are not guaranteed to stay here, and you should import from
/// `core` or `std` directly, not from here.
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub mod reexported {
    pub use core::*;
}

#[macro_use]
pub extern crate typenum;

// Copied from typenum so that users don't have to import typenum.
// Only change is the paths.
/// Construct a type-level array of type-level integers.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
/// #[macro_use]
/// extern crate generic_array;
///
/// use dim::typenum::consts::*;
/// type Arr = tarr![P3, P2, N5, N8, P2];
///
/// fn main() {
///     use dim::array::ToGA;
///     let x = Arr::to_ga();
///     let y = arr![isize; 3, 2, -5, -8, 2];
///
///     assert_eq!(x, y);
/// }
/// ```
#[macro_export]
macro_rules! tarr {
    () => ( $crate::typenum::ATerm );
    ($n:ty) => ( $crate::typenum::TArr<$n, $crate::typenum::ATerm> );
    ($n:ty,) => ( $crate::typenum::TArr<$n, $crate::typenum::ATerm> );
    ($n:ty, $($tail:ty),+) => ( $crate::typenum::TArr<$n, tarr![$($tail),+]> );
    ($n:ty, $($tail:ty),+,) => ( $crate::typenum::TArr<$n, tarr![$($tail),+]> );
}


#[macro_use]
pub extern crate generic_array;

pub mod traits;

#[macro_use]
mod make_units;

pub mod unit_systems;
pub use unit_systems::{cgs, si, mks};

pub use traits::*;


pub mod array;
