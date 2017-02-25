#![cfg_attr(feature = "oibit", feature(optin_builtin_traits))]

#[macro_use]
extern crate dimensioned as dim;

mod ms {
    make_units! {
        MS;
        ONE: Unitless;

        base {
            M: Meter, "m";
            S: Second, "s";
        }

        derived {
        }

        constants {
            MIN: Second = 60.0;
            HR: Second = 60.0 * 60.0;
            DAY: Second = 24.0 * 60.0 * 60.0;

            IN: Meter = 0.0254;
            FT: Meter = 0.3048;
            YD: Meter = 0.9144;
        }

        fmt = true;
    }

    pub use self::f64consts::*;
}


mod fm {
    make_units! {
        FM;
        ONE: Unitless;

        base {
            FT: Foot, "ft";
            MIN: Minute, "min";

        }

        derived {
        }

        constants {
            S: Minute = 1.0 / 60.0;
            HR: Minute = 60.0;
            DAY: Minute = 24.0 * 60.0;

            IN: Foot = 1.0 / 12.0;
            YD: Foot = 3.0;
            M: Foot = 1.0 / 0.3048;
        }

        fmt = true;
    }

    pub use self::f64consts::*;
}

use std::ops::Mul;
use dim::typenum::{Integer, Prod};
use std::convert::From;

impl<V, Length, Time> From<ms::MS<V, tarr![Length, Time]>> for fm::FM<Prod<V, f64>, tarr![Length, Time]>
    where V: Mul<f64>, Length: Integer, Time: Integer,
{
    fn from(other: ms::MS<V, tarr![Length, Time]>) -> Self {
        let length_fac = (1.0_f64 / 0.3048).powi(Length::to_i32());
        let time_fac = (1.0_f64 / 60.0).powi(Time::to_i32());
        let fac = length_fac * time_fac;

        fm::FM::new( other.value_unsafe * fac )
    }
}

impl<V, Length, Time> From<fm::FM<V, tarr![Length, Time]>> for ms::MS<Prod<V, f64>, tarr![Length, Time]>
    where V: Mul<f64>, Length: Integer, Time: Integer,
{
    fn from(other: fm::FM<V, tarr![Length, Time]>) -> Self {
        let length_fac = 0.3048_f64.powi(Length::to_i32());
        let time_fac = 60_f64.powi(Time::to_i32());
        let fac = length_fac * time_fac;

        ms::MS::new( other.value_unsafe * fac )
    }
}

// ---------------------------------------------------------------------------------------

fn main() {
    let x1 = 3.0 * ms::M + 2.0 * ms::FT + ms::Meter::new(1.2);
    let x2 = 3.0 * fm::M + 2.0 * fm::FT + 1.2 * fm::M;

    assert_eq!(x1, ms::MS::from(x2));
    assert_eq!(fm::FM::from(x1), x2);

    // prints: x1 = 4.81 m, x2 = 15.78 ft
    println!("x1 = {:.2}, x2 = {:.2}", x1, x2);

    let x = 4.0*ms::M*ms::M;

    use dim::Sqrt;
    println!("{}", x.sqrt());
}
