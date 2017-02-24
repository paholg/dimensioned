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
#![no_std]

#![cfg_attr(feature = "oibit", feature(optin_builtin_traits))]

// For clippy:
#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(unknown_lints)]
#![deny(clippy)]
#![allow(type_complexity, float_cmp, useless_attribute)]

// Macro debugging
// #![feature(trace_macros)]
// trace_macros!(true);


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
/// type TArr = tarr![P3, P2, N5, N8, P2];
///
/// fn main() {
///     use dim::array::ToGA;
///     let x = TArr::to_ga();
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

// Get a warning without this. If it's fixed, remove `useless_attribute` from clippy allow list
#[allow(unused_imports)]
#[macro_use]
pub extern crate generic_array;

pub mod traits;

#[macro_use]
mod make_units;

pub mod f32prefixes;
pub mod f64prefixes;

pub mod unit_systems;
pub use unit_systems::{cgs, si, mks, ucum};

pub use traits::*;


pub mod array;

pub mod dimcore {
    pub use core::{marker, fmt, ops, mem, f32, f64};
}
