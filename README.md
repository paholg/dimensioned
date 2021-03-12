[![crates.io](https://img.shields.io/crates/v/dimensioned.svg)](https://crates.io/crates/dimensioned)
[![Build Status](https://github.com/paholg/dimensioned/actions/workflows/check.yml/badge.svg)](https://github.com/paholg/dimensioned/actions/workflows/check.yml)

[Documentation](https://docs.rs/dimensioned/)

Dimensioned
=====

A Rust library for compile-time dimensional analysis.

Its goal is to provide zero cost unit safety while requiring minimal effort from the programmer.

# Use

Dimensioned requires at least Rust version 1.23.0 (and is tested on this version), although some
features may require a newer version.

It does not depend on `std`; simple include without the default feature `std`. Doing so requires a
nightly version of rustc.

If you are using Rust nightly, then you may enable the "oibit" feature of dimensioned. This will
make it work a bit better for wrapping non-primitives in units. The recommended way to use
dimensioned is by wrapping only primitives in units, in which case this feature is not helpful.

# Contributing

Contributions are welcome! There aren't super strict contributing guidelines, but I have a few
requests.

* Note changes that you make in [CHANGELOG.md](CHANGELOG.md).
* Add documentation and tests for anything that you add.
* Try to keep lists alphabetized in files not addressed by rustfmt.
* Don't hesitate to prod me if I haven't responded to you in a timely manner. I get
  busy and forgetful, but I would like to repond to issues and PRs promptly.
* Feel free to ask questions!

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
    let si_x = 6.0 * si::M;
    let si_t = 3.0 * si::S;
    let si_v = 2.0 * si::M / si::S;

    let si_v2 = speed(si_x, si_t);
    assert_eq!(si_v, si_v2);

    let cgs_x = 6.0 * cgs::M;
    let cgs_t = 3.0 * cgs::S;
    let cgs_v = 2.0 * cgs::M / cgs::S;

    let cgs_v2 = generic_speed(cgs_x, cgs_t);
    assert_eq!(cgs_v, cgs_v2);

    let si_v3 = cgs_v2.into();
    assert_eq!(si_v2, si_v3);
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
