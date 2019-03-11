#![cfg(test)]
#![cfg(feature = "quickcheck")]

#[macro_use]
extern crate quickcheck;

extern crate dimensioned as dim;

mod quickchecking {
    use dim::si::*;

    use quickcheck::TestResult;

    quickcheck! {
        fn add(x: f64, y: f64) -> bool {
            let m_res = x * M + y * M == Meter::new(x + y);
            let s_res = x * S + y * S == Second::new(x + y);
            let j_res = x * J + y * J == Joule::new(x + y);

            m_res && s_res && j_res
        }
    }

    quickcheck! {
        fn sub(x: f64, y: f64) -> bool {
            let m_res = x * M - y * M == Meter::new(x - y);
            let s_res = x * S - y * S == Second::new(x - y);
            let j_res = x * J - y * J == Joule::new(x - y);

            m_res && s_res && j_res
        }
    }

    quickcheck! {
        fn mul(x: f64, y: f64) -> bool {
            let res1 = (x * M) * y == Meter::new(x * y);
            let res2 = x * (y * M) == Meter::new(x * y);
            let res3 = (x * M) * (y * M) == Meter2::new(x * y);

            res1 && res2 && res3
        }
    }

    quickcheck! {
        fn div(x: f64, y: f64) -> bool {
            let res1 = (x * M) / y == Meter::new(x / y);
            let res2 = x / (y * M) == PerMeter::new(x / y);
            let res3 = (x * M) * (y * M) == Meter2::new(x * y);

            res1 && res2 && res3
        }
    }

    use dim::Sqrt;
    quickcheck! {
        fn sqrt(x: f64) -> TestResult {
            if x < 0. {
                return TestResult::discard();
            }

            let res1 = *(x * ONE).sqrt() == x.sqrt();
            let res2 = (x * M * M).sqrt() == Meter::new(x.sqrt());
            let res3 = (x * M * M / S / S).sqrt() == MeterPerSecond::new(x.sqrt());

            TestResult::from_bool(res1 && res2 && res3)
        }
    }

    use dim::Cbrt;
    #[cfg(feature = "std")]
    quickcheck! {
        fn cbrt(x: f64) -> bool {
            let res1 = *(x * ONE).cbrt() == x.cbrt();
            let res2 = (x * M * M * M).cbrt() == Meter::new(x.cbrt());
            let res3 = (x * M * M * M / S / S / S).cbrt() == MeterPerSecond::new(x.cbrt());

            res1 && res2 && res3
        }
    }
}
