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
    let xhat: Dim<Unitless, Vector2d> = Dim(Vector2d{x: 1.0, y: 0.0});
    let yhat: Dim<Unitless, Vector2d> = Dim(Vector2d{x: 0.0, y: 1.0});

    let start = -xhat*m*13.0 + yhat*m*33.0;
    let end = xhat*m*26.0 - yhat*m*19.0;

    let displace = end - start;
    let time = s*26.0;
    let vel = displace/time;
    // Because we put norm() in a trait and implemented it for both Vector2d and Dim,
    // calling vel.norm() works as we want it to (returning Dim<Meter, ff64>). This is
    // the recommended way of accessing values inside a Dim.
    let speed = vel.norm();
    // Had we been unable or unwilling to implement norm() inside a trait, we could have
    // achieved the same behavior using the wrap() function, as follows:
    let speed2 = vel.wrap((vel.0).norm());
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}! Again, that's {}!", start, end, displace, time, vel, speed, speed2);

    let center = xhat*m*24.0 - yhat*m*17.0;
    let force = xhat*500.0*kg*m/s/s;
    let r = end-center;
    println!("
Now, she's standing next to a merry-go-round, centered at {}.
That is {} away from her. She decides to spin it, pushing with a force of {}.
That's a torque of {}!", center, r.norm(), force, r.cross(force));
}
```
with output:

```
A physicist was standing at (-13, 33) m.
Then she walked to (26, -19) m, for a displacement of (39, -52) m.
The walk took her 26 s, so she must have had a velocity of (1.5, -2) ms^-1.
That's a speed of 2.5 ms^-1! Again, that's 2.5 ms^-1!

Now, she's standing next to a merry-go-round, centered at (24, -17) m.
That is 2.828427 m away from her. She decides to spin it, pushing with a force of (500, 0) mkgs^-2.
That's a torque of 1000 m^2kgs^-2.
```

Run the example with

```
cargo run --example vec
```
