[![crates.io](https://img.shields.io/crates/v/dimensioned.svg)](https://crates.io/crates/dimensioned)
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

[Documentation](http://paholg.com/dimensioned/)

Dimensioned
=====

Dimensioned is a Rust library for compile-time dimensional analysis.

The goal of dimensioned is to give you safety when working any units of measure and easy conversion
between unit systems all while being as invisible and out of the way as possible.

The meat of dimensioned is located in the `make_units!` macro, which is used to create a unit
system. Dimensioned also ships with some unit systems, and it is recommended that you use one of
them if it fits your use case. If you feel that dimensioned should ship with more unit systems,
please submit an issue.

**NOTE:** Dimensioned is undergoing much flux right now, so you should expect breaking changes for
at least the near future.

## Dimensioned Basics

Dimensioned requires Rust version 1.13.0 or higher. That is currently the beta, but should be
stable around the middle of November.

If you use dimensioned with the nightly compiler and the "oibit" feature, you will gain a bit of
flexibility when multiplying or dividing something with dimensions by a non-primitive that does not
have dimensions. // fixme: reword, add example

Dimensioned can be used without the `std` library by using it with default features. Doing so uses
some experimental features of `core`, and so a nightly compiler is required. There is no reason not
to use the "oibit" feature in this case.

## Examples

Here is a short example to showcase some of its features:

```rust
extern crate dimensioned as dim;

use dim::{mks, cgs};
use std::convert::From;

fn speed(dist: mks::Meter<f64>, time: mks::Second<f64>) -> mks::MeterPerSecond<f64> {
    dist / time
}

fn main() {
    let x = 6.0 * mks::m;
    let t = 3.0 * mks::s;
    let v = 2.0 * mks::m/mks::s;
    let v2 = speed(x, t);
    assert_eq!(v, v2);

    let v3 = 2000.0 * cgs::m / cgs::s;
    let v4 = cgs::CGS::From(v);
    assert_eq(v3, v4);
}
```

The way that this works is the `si` module contains an `SI<V, A>` type, where `V` is the type of
whatever value is being wrapped (in this case `f64`) and `A` is a type-level array of
type-numbers. You don't have to worry too much about that, just be aware that `A` is different for
each unique unit.

The `si` module provides aliases for the `SI` type with different values of `A` corresponding to
each base unit, which is what `Meter<V>` and `Second<V>` are. It also provides aliases for many
derived types, such as `MeterPerSecond<V>`. If you find that you need an alias for a derived type
that is not provided, you can create it relatively painlessly with the `unit!()` macro.

One thing to note is that error messages containing these types tend to be very ugly; if you see an
error with very long, nested types, then I recommend jumping straight to that line in the
code. Chances are, it just caught an error with your dimensional analysis! It's also possible that
it means you ran into something where dimensioned is lacking; please file an issue if you feel it's
missing something.

## Creating a custom unit system

## Error Messages
