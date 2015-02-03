use std::fmt;
// use dimensioned::*;
//use dimensioned::{nano, pico};
use dimensioned::Dimensioned;
use dimensioned::Dim;

/// SI units
pub struct SI {
    pub meters: i8,
    pub kilograms: i8,
    pub seconds: i8,
    pub amperes: i8,
    pub kelvin: i8,
    pub candela: i8,
    pub moles: i8
}

impl Dim for SI {
    fn to_si(&self) -> SI { *self }

    fn unitless() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 0, amperes: 0,
                                         kelvin: 0, candela: 0, moles: 0 } }
    }
    fn meters() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 1, kilograms: 0, seconds: 0, amperes: 0,
                                         kelvin: 0, candela: 0, moles: 0 } }
    }
    fn grams() -> Dimensioned<SI> {
        Dimensioned{val: 0.001, units: SI{ meters: 0, kilograms: 1, seconds: 0, amperes: 0,
                                               kelvin: 0, candela: 0, moles: 0 } }
    }
    fn seconds() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 1, amperes: 0,
                                         kelvin: 0, candela: 0, moles: 0 } }
    }
    fn amperes() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 0, amperes: 1,
                                         kelvin: 0, candela: 0, moles: 0 } }
    }
    fn kelvin() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 0, amperes: 0,
                                         kelvin: 1, candela: 0, moles: 0 } }
    }
    fn candela() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 0, amperes: 0,
                                         kelvin: 0, candela: 1, moles: 0 } }
    }
    fn moles() -> Dimensioned<SI> {
        Dimensioned{val: 1.0, units: SI{ meters: 0, kilograms: 0, seconds: 0, amperes: 0,
                                         kelvin: 0, candela: 0, moles: 1 } }
    }
}
impl Mul<SI, SI> for SI {
    fn mul(&self, other: &SI) -> SI {
        SI {
            meters: self.meters + other.meters, kilograms: self.kilograms + other.kilograms,
            seconds: self.seconds + other.seconds, amperes: self.amperes + other.amperes,
            kelvin: self.kelvin + other.kelvin, candela: self.candela + other.candela,
            moles: self.moles + other.moles
        }
    }
}
impl Div<SI, SI> for SI {
    fn div(&self, other: &SI) -> SI {
        SI {
            meters: self.meters - other.meters, kilograms: self.kilograms - other.kilograms,
            seconds: self.seconds - other.seconds, amperes: self.amperes - other.amperes,
            kelvin: self.kelvin - other.kelvin, candela: self.candela - other.candela,
            moles: self.moles - other.moles
        }
    }
}
impl PartialEq for SI {
    fn eq(&self, other: &SI) -> bool {
        self.meters == other.meters && self.kilograms == other.kilograms &&
            self.seconds == other.seconds && self.amperes == other.amperes &&
            self.kelvin == other.kelvin && self.candela == other.candela &&
            self.moles == other.moles
    }
    fn ne(&self, other: &SI) -> bool {
        !(self == other)
    }
}

impl Clone for SI {
    fn clone(&self) -> SI {
        SI {
            meters: self.meters, kilograms: self.kilograms, seconds: self.seconds,
            amperes: self.amperes, kelvin: self.kelvin, candela: self.candela, moles: self.moles
        }
    }
}

impl fmt::Show for SI {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut unit_string = String::new();
        let mut first = true;
        for (unit, exp) in ["m", "kg", "s", "A", "K", "cd", "mol"].iter().zip([self.meters, self.kilograms, self.seconds, self.amperes, self.kelvin, self.candela, self.moles].iter()) {
            if *exp != 0 {
                if first == false {
                    unit_string.push('*');
                }
                unit_string.push_str(*unit);
                if *exp != 1 {
                    unit_string = format!("{}^{}", unit_string, exp);
                }
                first = false;
            }
        }
        write!(f, "{}", unit_string)
    }
}


// #[macro_export]
// // fixme: find a better way to do this
// macro_rules! si(
//     ($val:expr $dim:expr) => (
//         let s = stringify!($dim);
//         let re = regex!("");
//         );
//     )

// #[test]
// fn str_parse_test() {
//     // let s = si!(3 m/s^2);
//     // let s = "4.5e-7 nm/ps^2";
//     // let x1: Dimensioned<SI> = nano(Dim::meters(4.5e-7))/pico(Dim::seconds(1.0))/Dim::seconds(1.0);
//     // let x2: Dimensioned<SI> = from_str("4.5e-7 nm/ps^2").unwrap();
//     // assert_eq!(x1, x2);

//     // println!("\n\nmacro-test: {}\n\n", s);
// }

