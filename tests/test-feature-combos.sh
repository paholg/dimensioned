#!/bin/sh

# This is a simple script to make it easy to test combinations of
# features.  Currently it only tests with no features and with
# features taken individually, along with "test" which lumps together
# all the features that don't require nightly.

set -ev

cargo test

for f in test approx clapme quickcheck serde_test rand; do
    echo cargo test --features $f
    cargo test --features $f
done
