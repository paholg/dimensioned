# Changelog

This project follows semantic versioning.

### Unpublished
- `[added]` Unit conversion example.
- `[changed]` Removed `Dimensioned` constraint for `D` in `Dim<D, V>`. This enables derived types to work properly. It is a temporary change only until the bug with Rust is fixed. See [issue #6](https://github.com/paholg/dimensioned/issues/6).
- `[added]` Macro for constructing derived units. The derived block of `make_units!` also now works.

### 0.5.0 (2015-12-02)
- `[added]` This change log!
- `[changed]` **BREAKING** Use typenum instead of peano for faster and more complete type-level numbers. ([PR #3](https://github.com/paholg/dimensioned/pull/3))
- `[added]` Re-export useful things from typenum so that crates downstream don't need to depend on it.
