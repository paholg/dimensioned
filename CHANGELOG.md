# Changelog

This project follows semantic versioning.

### Unpublished
- `[added]` Usable from rust version 1.13.0. There is now a default feature `nightly`, without which
  dimensioned does not need a nightly version of the compiler. Note that this makes multiplication
  and division not work as well, to the point that you can pretty much only use `Dim` with
  primitives.
- `[added]` Usable without libstd. There is now a default feature `std`, without which `libcore`
  will be used. This uses some unstable features.
- `[added]` Unit conversion example.
- `[changed]` Removed `Dimensioned` constraint for `D` in `Dim<D, V>`. This enables derived types
  to work properly. It is a temporary change only until the bug with Rust is fixed. See
  [issue #6](https://github.com/paholg/dimensioned/issues/6).
- `[added]` Macro for constructing derived units. The derived block of `make_units!` also now
  works.

### 0.5.0 (2015-12-02)
- `[added]` This change log!
- `[changed]` **BREAKING** Use typenum instead of peano for faster and more complete type-level numbers. ([PR #3](https://github.com/paholg/dimensioned/pull/3))
- `[added]` Re-export useful things from typenum so that crates downstream don't need to depend on it.
