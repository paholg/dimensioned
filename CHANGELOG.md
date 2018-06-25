# Changelog

This project follows semantic versioning.

### Unpublished
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
