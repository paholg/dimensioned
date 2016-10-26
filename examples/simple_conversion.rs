#[macro_use]
extern crate dimensioned as dim;

use dim::Dim;
use dim::typenum::Integer;
use std::ops::Mul;

mod ms {
    make_units! {
        MS, Unitless;
        base {
            Meter, m;
            Second, s;
        }
        derived {
        }
    }

    pub trait FromMS<Meter: Integer, Second: Integer, V> where Self: Sized {
        fn from_ms(from: Dim<MS<Meter, Second>, V>) -> Dim<Self, V>;
    }
}


mod fs {
    make_units! {
        FS, Unitless;
        base {
            Foot, ft;
            Second, s;
        }
        derived {
        }
    }

    pub trait FromFS<Foot: Integer, Second: Integer, V> where Self: Sized {
        fn from_fs(from: Dim<FS<Foot, Second>, V>) -> Dim<Self, V>;
    }
}

impl<Meter: Integer, Second: Integer, V: Mul<f64, Output = V>> ms::FromMS<Meter, Second, V> for fs::FS<Meter, Second> {
    fn from_ms(from: Dim<ms::MS<Meter, Second>, V>) -> Dim<Self, V> {
        Dim::new(from.0 * 3.281)
    }
}

impl<Foot: Integer, Second: Integer, V: Mul<f64, Output = V>> fs::FromFS<Foot, Second, V> for ms::MS<Foot, Second> {
    fn from_fs(from: Dim<fs::FS<Foot, Second>, V>) -> Dim<Self, V> {
        Dim::new(from.0 * 0.305)
    }
}

fn main() {
    let x_m = ms::Meter::new(5.0);
    use ms::FromMS;
    let x_ft = fs::FS::from_ms(x_m);

    use fs::FromFS;
    println!("x in meters: {}, x in feet: {}, x in meters again: {}", x_m, x_ft, ms::MS::from_fs(x_ft));
}
