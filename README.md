[![crates.io](https://img.shields.io/crates/v/dimensioned.svg)](https://crates.io/crates/dimensioned)
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

[Documentation](http://paholg.com/dimensioned/)

Dimensioned
=====

A Rust library for compile-time dimensional analysis.

Its goal is to provide zero cost unit safety while requiring minimal effort from the programmer.

# Use

Dimensioned requires at least Rust version 1.15. It does not depend on `std`.

If you are using Rust nightly, then you may enable the "oibit" feature of dimensioned. This will
make it work a bit better for wrapping non-primitives in units. The recommended way to use
dimensioned is by wrapping only primitives in units, in which case this feature is not helpful.

# Examples

[The Simulation Example](https://github.com/paholg/dimensioned-examples/blob/master/hard-spheres.md)
provides a simple physics simulation and covers how one can use dimensioned with it in a couple
different ways, and what the trade-offs are. If you're curious about what might be involved in
adding dimensioned to one of your projects, or what it might look like in semi-real code, then that
is the place for you.

[The Conversion Example](https://github.com/paholg/dimensioned-examples/blob/master/src/conversion.md)
covers how one might implement conversions between unit systems.

Finally, just to get the juices flowing, here's a simple example illustrating some of what
dimensioned can do:

```rust
extern crate dimensioned as dim;

use dim::{si, cgs};

// Calculates speed given a distance and time. Only works for SI units.
fn speed(dist: si::Meter<f64>, time: si::Second<f64>) -> si::MeterPerSecond<f64> {
    dist / time
}

use std::ops::Div;
use dim::dimensions::{Length, Time};
use dim::typenum::Quot;

// Calculates speed as before, but now we can use *any* unit system.
fn generic_speed<L, T>(dist: L, time: T) -> Quot<L, T>
    where L: Length + Div<T>, T: Time,
{
    dist / time
}

fn main() {
    let x = 6.0 * si::M;
    let t = 3.0 * si::S;
    let v = 2.0 * si::M/si::S;
    let v2 = speed(x, t);
    assert_eq!(v, v2);

    let v3 = generic_speed(6.0 * cgs::M, 3.0 * cgs::S);
    let v4 = v.into();
    assert_eq!(v3, v4);
}
```

This example is also included as `examples/readme-example.rs`.

# Unit Systems

Dimensioned aims to include unit systems for a large variety of uses. It also includes a
`make_units!` macro to allow you to create any unit system you desire.

# Error Messages

Probably the biggest weakness of dimensioned are the error messages generated. The type
signatures coming from dimensioned tend to just look like a bunch of gobbly-guck. Someday, we may
have a better way to display them.

For now, my advice is that when you get an error message involving *dimensioned*, just go to the
line number and hopefully the issue will be apparant from the code alone.

# Friends of dimensioned

If there are any libraries that work particularly well with dimensioned, such as the vector3d
library used in part 3 of the simulation example, please let me know and they will be listed here.
