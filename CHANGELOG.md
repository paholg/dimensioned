# Changelog

This project follows semantic versioning.

### Unpublished

### 0.7.0 (2018-08-12)
- [changed] ***BREAKING*** Made dimensioned work with `no_std` again, and added the default feature
  `std`. Using this without `std` now requires nightly Rust and using `dimensioned` without default
  features. I don't think this will actually break anything, as it was not working before, but is at
  least a break in the intended way to use `dimensioned` without `std`. Once `sqrt`, `powf`, and
  `cbrt` are usable without std on stable, the nightly requirement can be removed.
- [added] Optional dependency on `ClapMe` and implementation for unit systems via `clapme!` macro.
- [added] Trait `Abs` for taking the absolute value of quantities.
- [added] Feature flag `serde` that derives Serialization/Deserialization for unit systems.
- [added] Modules for integer constants to `make_units!` macro.

### 0.6.0 (2017-03-03)
- [changed] ***BREAKING*** Dimensioned has been rewritten from the ground up. It still does the
  same thing but the internals are completely different. Read about it
  [here](http://paholg.com/2017/03/03/dimensioned_0.6/).

### 0.5.0 (2015-12-02)
- [added] This change log!
- [changed] ***BREAKING*** Use typenum instead of peano for faster and more complete type-level
  numbers. ([PR #3](https://github.com/paholg/dimensioned/pull/3))
- [added] Re-export useful things from typenum so that crates downstream don't need to depend on
  it.
