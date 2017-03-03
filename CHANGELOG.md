# Changelog

This project follows semantic versioning.

### 0.6.0 (Unpublished)
- `[changed]` ***BREAKING*** Dimensioned has been rewritten from the ground up. It still does the
  same thing but the internals are completely different. Stay tuned for a blog post that will cover
  it in more detail.

### 0.5.0 (2015-12-02)
- `[added]` This change log!
- `[changed]` ***BREAKING*** Use typenum instead of peano for faster and more complete type-level
  numbers. ([PR #3](https://github.com/paholg/dimensioned/pull/3))
- `[added]` Re-export useful things from typenum so that crates downstream don't need to depend on
  it.
