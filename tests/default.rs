extern crate dimensioned as dim;

use crate::dim::si::*;

#[test]
fn default() {
    // SI base units
    assert_eq!(Meter::new(0), Meter::default());
    assert_eq!(Second::new(0.0), Second::default());
    // SI derived units
    assert_eq!(MeterPerSecond::new(0.0), MeterPerSecond::default());
}
