Dimensioned =====

[![crates.io](https://img.shields.io/crates/v/units.svg)](https://crates.io/crates/units)
[![Build Status](https://travis-ci.org/paholg/dimensioned.svg?branch=master)](https://travis-ci.org/paholg/dimensioned)


Dimensioned is a library for compile-time type checking of arbitrary unit systems.

To use, place in Cargo.toml:

```
[dependencies]
  dimensioned = "*"
```

Note, though, that it is currently going through volatile changes and will stabilize
sometime in the future.

Dimensioned now has a convenient macro for creating a unit system. Here is an example of
its use (from
[examples/vec.rs](https://github.com/paholg/dimensioned/blob/master/examples/vec.rs)):

```
make_units! {
    SI, One;
    base {
        Meter, meter, m;
        Kilogram, kilogram, kg;
        Second, second, s;
        Ampere, ampere, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
    }
}
```

The first line inside the macro, `SI, One;` is the name of your unit system followed by
the highest allowed root that you want in Peano number form. Generally, you want this to
be `One`, but for CGS units, for example, the square root of base units is allowed, so
it should be `Two`. The numbers `One` through `Ten` are defined in the library; if for
some reason you need something higher, you can use the syntax

```type Eleven = Succ<Ten>;```

Then, the block `base {}` designates the base units of the system. They are given in the
form `Type, constant, print_form;` so that the first line, `Meter, meter, m;` will
define the type `Meter`, the constant `meter` which is equal to 1f64 m, and when printed
will have the `m` token.

The `derived {}` block is for derived units. It is not yet implemented, and is only
there to ensure future compatibility.

Dimensioned has its own (under construction) website, available
[here](http://paholg.com/dimensioned/).  It is also on
[Crates.io](https://crates.io/crates/dimensioned/).
