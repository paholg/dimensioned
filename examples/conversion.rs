#[macro_use]
extern crate dimensioned as dim;
extern crate typenum;

use dim::{Dim};
use dim::cgs::{self, CGS, FromCGS};
use typenum::int::Integer;

use std::ops::{Mul, Div};

type Quot<A, B> = <A as Div<B>>::Output;


mod fps {
    make_units_adv! {
        FPS, Unitless, one, f64, 1.0;
        base {
            P2, Foot, ft, ft;
            P2, Pound, lb, lb;
            P1, Second, s, s;
        }
        derived {
        }
    }
    pub trait FromFPS<Foot: Integer, Pound: Integer, Second: Integer, V> where Self: Sized {
        fn from_fps(from: Dim<FPS<Foot, Pound, Second>, V>) -> Dim<Self, V>;
    }
}

use fps::{FPS, FromFPS};

impl<Centimeter, Gram, Second, V> FromCGS<Centimeter, Gram, Second, V> for FPS<Centimeter, Gram, Second>
    where V: Mul<f64, Output=V>, Centimeter: Integer, Gram: Integer, Second: Integer, {
    fn from_cgs(from: Dim<CGS<Centimeter, Gram, Second>, V>) -> Dim<Self, V> {
        Dim::new( from.0 * 0.0328084f64.sqrt().powi(Centimeter::to_i32()) * 0.00220462f64.sqrt().powi(Gram::to_i32()) )
    }
}


fn main() {
    let speed = 35.0 * cgs::cm / cgs::s;

    let t2 = 10.0 * cgs::cm;
    let t2f = FPS::from_cgs(t2);

    println!("t2: {}, t2f: {}", t2, t2f);

    // let speed2 = speed.convert_to();

    // let speed2 = MKS::from_cgs(speedc);

    // println!("I was going {}, which is {}. Boo: {}", speedc, speed2, speed2);
}
