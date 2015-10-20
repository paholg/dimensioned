[![crates.io](https://img.shields.io/crates/v/dimensioned.svg)](https://crates.io/crates/dimensioned)
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

Dimensioned
=====

Dimensioned is a Rust library for compile-time type checking of arbitrary unit systems. It
has its own [website](http://paholg.com/projects/dimensioned/).

Here is a short example of its use:

```rust
extern crate dimensioned;
use dimensioned::si::{one, m, s};

fn main() {
    let x = 6.0 * m;
    let t = 3.0 * s;
    let v = 2.0 * m/s;
    assert_eq!(v, x/t);
}
```

For more in-depth examples and documentation, see the website.

Note: This library is unstable, and you should expect breaking changes for now. It also
depends on unstable Rust features.
