extern crate dimensioned as dim;

use dim::si::dimensions::{Length, Time, Velocity};
use dim::si::{Meter, MeterPerSecond, Second};

// A simple test case to exercise the `dimensions` module type aliases.

#[test]
fn units() {
    fn calc_velocity(distance: Length<i64>, time: Time<i64>) -> Velocity<i64> {
        distance / time
    }

    assert_eq!(
        calc_velocity(Meter::new(10), Second::new(2)),
        MeterPerSecond::new(5)
    );
}
