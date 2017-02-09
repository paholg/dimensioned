#![cfg(feature = "qc")]

#![feature(plugin)]
#![plugin(quickcheck_macros)]

extern crate quickcheck;

extern crate dimensioned as dim;

mod quickchecking {
    use dim::si::*;

    use quickcheck::TestResult;

    #[quickcheck]
    fn add(x: f64, y: f64) -> bool {
        x*M + y*M == Meter::new(x + y) &&
        x*S + y*S == Second::new(x + y) &&
        x*J + y*J == Joule::new(x + y)
    }

    #[quickcheck]
    fn sub(x: f64, y: f64) -> bool {
        x*M - y*M == Meter::new(x - y) &&
        x*S - y*S == Second::new(x - y) &&
        x*J - y*J == Joule::new(x - y)
    }

    #[quickcheck]
    fn mul(x: f64, y: f64) -> bool {
        (x*M) * y == Meter::new(x * y) &&
        x * (y*M) == Meter::new(x * y) &&
        (x*M) * (y*M) == Meter2::new(x * y)
    }

    #[quickcheck]
    fn div(x: f64, y: f64) -> bool {
        (x*M) / y == Meter::new(x / y) &&
        x / (y*M) == InverseMeter::new(x / y) &&
        (x*M) * (y*M) == Meter2::new(x * y)
    }

    use dim::Sqrt;
    #[quickcheck]
    fn sqrt(x: f64) -> TestResult {
        if x < 0. {
            return TestResult::discard();
        }

        TestResult::from_bool(
            *(x*ONE).sqrt() == x.sqrt() &&
            (x*M*M).sqrt() == Meter::new(x.sqrt()) &&
            (x*M*M/S/S).sqrt() == MeterPerSecond::new(x.sqrt())
        )
    }

    use dim::Cbrt;
    #[quickcheck]
    fn cbrt(x: f64) -> bool {
        *(x*ONE).cbrt() == x.cbrt() &&
        (x*M*M*M).cbrt() == Meter::new(x.cbrt()) &&
        (x*M*M*M/S/S/S).cbrt() == MeterPerSecond::new(x.cbrt())
    }

}
