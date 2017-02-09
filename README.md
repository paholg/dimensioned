[![crates.io](https://img.shields.io/crates/v/dimensioned.svg)](https://crates.io/crates/dimensioned)
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

[Documentation](http://paholg.com/dimensioned/)

Dimensioned
=====

*Dimensioned* is a Rust library for compile-time dimensional analysis.

Its goal is to provide zero cost unit safety while requiring minimal effort from the programmer.

For a moderately in-depth example of using *dimensioned*,
[please go here](https://github.com/paholg/monte-carlo-test/), where cover using *dimensioned* in a
couple different ways and what trade-offs are involved.

The meat of *dimensioned* is located in the `make_units!` macro, which is used to create a unit
system. *Dimensioned* also ships with some unit systems, and it is recommended that you use one of
them if it fits your use case. If you feel that *dimensioned* should ship with more unit systems,
please submit an issue.


## Use

*Dimensioned* requires at least Rust version 1.15.

If you are using Rust nightly, then you may enable the "oibit" feature of dimensioned. This will
make it work a bit better for wrapping non-primitives in units. The recommended way to use
*dimensioned* is by wrapping only primitives in units, in which case this feature is not helpful.

## Minimal Example

```rust
extern crate dimensioned as dim;

use dim::{mks, cgs};
use std::convert::From;

fn speed(dist: mks::Meter<f64>, time: mks::Second<f64>) -> mks::MeterPerSecond<f64> {
    dist / time
}

fn main() {
    let x = 6.0 * mks::M;
    let t = 3.0 * mks::S;
    let v = 2.0 * mks::M/mks::S;
    let v2 = speed(x, t);
    assert_eq!(v, v2);

    let v3 = 2000.0 * cgs::M / cgs::S;
    let v4 = cgs::CGS::From(v);
    assert_eq(v3, v4);
}
```

For more in-depth examples, please see the documentation or the one linked above.

## Unit Systems

The heart of *dimensioned* is the `make_units!` macro, which creates a unit system. While
*dimensioned* ships with some unit systems, the macro is available for you to make any you desire.

If you think there's a unit system that would be widely used, please submit an issue. I see no
reason why *dimensioned* shouldn't provide all kinds of unit systems.

## Error Messages

Probably the biggest weakness of *dimensioned* are the error messages generated. The type
signatures coming from *dimensioned* tend to just look like a bunch of gobbly-guck. Someday, we may
have a better way to display them.

For now, my advice is that when you get an error message involving *dimensioned*, just go to the
line number and hopefully the issue will be apparant from the code alone.
