/*!
Compile-time dimensional analysis for various unit systems using Rust's type system.

Its goal is to provide zero cost unit safety while requiring minimal effort from the programmer.

For a short introduction and some examples, check out the [readme on
GitHub](https://github.com/paholg/dimensioned).
*/

#![doc(html_logo_url = "https://raw.githubusercontent.com/paholg/dimensioned/master/favicon.png",
       html_favicon_url = "https://raw.githubusercontent.com/paholg/dimensioned/master/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]

#![no_std]

#![warn(missing_docs)]

#![cfg_attr(feature = "oibit", feature(optin_builtin_traits))]
#![cfg_attr(feature = "spec", feature(specialization))]

#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]

#![allow(unknown_lints)]
#![deny(clippy)]
#![allow(type_complexity, float_cmp, useless_attribute, doc_markdown)]

// Macro debugging
// #![feature(trace_macros)]
// trace_macros!(true);

pub extern crate typenum;

// Copied from typenum so that users don't have to import typenum for the make_units macro to work.
// Only change is the paths.
/// Construct a type-level array of type-level integers
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

#[macro_use] mod make_units;
mod fmt;


include!(concat!(env!("OUT_DIR"), "/unit_systems.rs"));
pub mod traits;
pub mod dimensions;
pub mod conversion;
pub mod array;
pub mod f32prefixes;
pub mod f64prefixes;


pub use traits::*;
pub use unit_systems::{si, ucum, cgs, mks, fps};


// Used for the make_units macro
#[doc(hidden)]
pub mod dimcore {
    pub use core::{marker, fmt, ops, mem, f32, f64};
}
