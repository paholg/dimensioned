Dimensioned
=====
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

A library for compile-time type checking of arbitrary unit systems. This library
currently comes with just SI units, but you can easily make your own by editing or
importing unitsmaker.py. No tutorial yet, but it should be easy to emulate.

To build,
```
cargo build
```


For an example of its use, see examples/vec.rs. Note that Vector2d doesn't know anything
about dimensions!

Run the example with

```
cargo run --example vec
```
