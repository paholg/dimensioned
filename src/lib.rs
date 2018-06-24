/*!
Compile-time dimensional analysis for various unit systems using Rust's type system.

Its goal is to provide zero cost unit safety while requiring minimal effort from the programmer.

For a short introduction and some examples, check out the [readme on
GitHub](https://github.com/paholg/dimensioned).

# Use

The recommended way to import this crate is with the rename

```rust
extern crate dimensioned as dim;
```

and then either import the module for the unit system you will be using,

```rust
# extern crate dimensioned as dim;
# fn main() {
use dim::si;
let x = 3.0 * si::M;
# }
```

or import the constants and/or types that you care about,

```rust
# extern crate dimensioned as dim;
# fn main() {
use dim::si::{Meter, M, S};
let x: Meter<f64> = 3.0 * M;
let t = 2.0 * S;
# }
```

For clarity, all the examples contained herein will import unit system modules.


# How it works

When a unit system is created, say the SI system, a struct with two parameters is made; in this
case `SI<V, U>`. The first parameter, `V`, is for the value type -- it can be anything, and is the
value to which we are giving units. The second parameter, `U`, is where the magic happens. It
represents the units, and is a type-level array of type-level numbers. Now, these guys have some
pretty ugly type signatures, so I will express the array as the macro which maps to it, `tarr![]`,
and the type numbers as their aliases; `N1` for -1, `Z0` for 0, `P1` for 1, etc.

The SI system has seven base units, so that's how many elements the type array needs to have. The
order is the order in which the base units were defined, and the values of the type numbers are the
power to which each unit is raised. For example, the first three SI units, in order, are `Meter`,
`Kilogram`, `Second`, so the following aliases exist:

```ignore
type Meter<V>    = SI<V, tarr![P1, Z0, Z0, Z0, Z0, Z0, Z0]>;
type Kilogram<V> = SI<V, tarr![Z0, P1, Z0, Z0, Z0, Z0, Z0]>;
type Second<V>   = SI<V, tarr![Z0, Z0, P1, Z0, Z0, Z0, Z0]>;
type Newton<V>   = SI<V, tarr![P1, P1, N2, Z0, Z0, Z0, Z0]>;
...
```

In addition to creating the unit system struct, `SI`, the type aliases, and constants for each,
various traits are implemented for the unit system. These include arithmetic operations, which work
as follows.

If you try to add a `Meter<f64>` to a `Second<f64>`, then you will get a compiler error because
they have different types and so addition is not defined. Multiplication, on the other hand, is
defined, and results in a normal multiplication of the value types, and the unit powers added.

So, multiplying `Meter<f64>` by `Second<f64>` gives `SI<f64, tarr![P1, Z0, P1, Z0, Z0, Z0,
Z0]>`.

## Example

```rust
extern crate dimensioned as dim;

use dim::si;

fn main() {
    let x = 3.0 * si::M;
    let t = 2.0 * si::S;
    assert_eq!("6 m*s", &format!("{}", x*t));
}
```

That's basically it. All of the dimensional safety comes from whether things typecheck, and from
performing type-level arithmetic, thanks to the [typenum](http://paholg.com/typenum/)
crate. Pretty much everything else is for ergonomics.

*/

#![doc(html_logo_url = "https://raw.githubusercontent.com/paholg/dimensioned/master/favicon.png",
       html_favicon_url = "https://raw.githubusercontent.com/paholg/dimensioned/master/favicon.png",
       html_root_url = "http://paholg.com/dimensioned")]

#![feature(core_intrinsics)]
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

#[cfg(feature = "approx")]
pub extern crate approx;

#[cfg(feature = "serde")]
pub extern crate serde;
#[cfg(feature = "serde_test")]
extern crate serde_test;

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
