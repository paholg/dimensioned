# Changelog

This project follows semantic versioning.

### 0.6.0 (Unpublished)
- `[changed]` ***BREAKING*** Dimensioned has been rewritten from the ground up. It still does the
  same thing but the internals are completely different. The changes are too many to list (although
  I will try), and no compatibility with the previous version should be expected.
- `[changed]` ***BREAKING*** Removal of the `Dim` struct. Now, types will look like `SI<Units, T>`
  instead of `Dim<SI<Units>, T>` and type aliases will look like `Meter<T>` instead of `Dim<Meter,
  T>`. One can still be unit system agnostic by using the `Dimensioned` trait.
- `[added]` Many derived units added to unit systems.
- `[added]` The derived block of `make_units!` now works. Derived units can also be made
  separately. Many derived units have been added to unit systems.
- `[added]` Usable on stable Rust version >= 1.13.0. There is now a feature `oibit` that requires
  nightly Rust and makes multiplication and division more painless with non-primitives.
- `[removed]` Dependency on libstd.
- `[added]` Unit conversion API.

### 0.5.0 (2015-12-02)
- `[added]` This change log!
- `[changed]` ***BREAKING*** Use typenum instead of peano for faster and more complete type-level numbers. ([PR #3](https://github.com/paholg/dimensioned/pull/3))
- `[added]` Re-export useful things from typenum so that crates downstream don't need to depend on it.
