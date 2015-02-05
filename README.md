Dimensioned
=====
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)

A library for compile-time type checking of arbitrary unit systems.

This library currently comes with just SI units, but you can easily make your own by
editing or importing `unitsmaker.py`. It has no tutorial yet, but it should be easy enough
to emulate (just look at `main()`). Note: it does save a file to your computer and does
zero sanity checking, so run it at your own risk.

To build,
```
cargo build
```


Here is an example of its use:

(excerpt of [examples/vec.rs](https://github.com/paholg/dimensioned/blob/master/examples/vec.rs))
```rust
fn main() {
    let start: Dim<Meter, Vector2d> = Dim(Vector2d{x: -13.0, y: 33.0});
    let end: Dim<Meter, Vector2d> = Dim(Vector2d{x: 26.0, y: -19.0});
    let displace = end - start;
    let time = s*26.0;
    let vel = displace/time;
    // Dim has Deref, but doesn't know about norm(), so vel.norm() returns f64 *not*
    // Dim<Meter, f64>, which is what we desire. This is the use for vel.wrap()---it
    // "wraps" the argument in the same dimensions as vel. We could avoid using wrap()
    // by putting norm() in a trait and implementing it for Dim.
    let speed = vel.wrap(vel.norm());
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}!", start, end, displace, time, vel, speed);
}
```
with output:

```
A physicist was standing at (-13, 33) m.
Then she walked to (26, -19) m, for a displacement of (39, -52) m.
The walk took her 26 s, so she must have had a velocity of (1.5, -2) ms^-1.
That's a speed of 2.5 ms^-1!
```

Run the example with

```
cargo run --example vec
```
