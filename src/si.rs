// #![allow(non_upper_case_globals)]
// //use std::fmt;
// use dimensioned::Dimensioned;
// use dimensioned::Dim;
// use peano::*;
// // use std::ops::{Add, Sub, Mul, Div};

// /// SI units
// pub struct SI<Length: Peano, Mass: Peano, Time: Peano, Current: Peano, Temp: Peano, Intensity: Peano, Quanity: Peano>;

// impl<Length: Peano, Mass: Peano, Time: Peano, Current: Peano, Temp: Peano, Intensity: Peano, Quantity: Peano> Dim for SI<Length, Mass, Time, Current, Temp, Intensity, Quantity> {}

// // impl<Length: Peano, Mass: Peano, Time: Peano, Current: Peano, Temp: Peano, Intensity: Peano, Quantity: Peano> Add<> for SI<Length, Mass, Time, Current, Temp, Intensity, Quantity> {

// // }

// pub type Length = SI<One, Zero, Zero, Zero, Zero, Zero, Zero>;
// pub type Mass = SI<Zero, One, Zero, Zero, Zero, Zero, Zero>;
// pub type Time = SI<Zero, Zero, One, Zero, Zero, Zero, Zero>;
// pub type Current = SI<Zero, Zero, Zero, One, Zero, Zero, Zero>;
// pub type Temp = SI<Zero, Zero, Zero, Zero, One, Zero, Zero>;
// pub type Intensity = SI<Zero, Zero, Zero, Zero, Zero, One, Zero>;
// pub type Quantity = SI<Zero, Zero, Zero, Zero, Zero, Zero, One>;

// pub static m: Dimensioned<Length, f64> = Dimensioned(1.0);
// pub static g: Dimensioned<Mass, f64> = Dimensioned(1.0e-3);
// pub static kg: Dimensioned<Mass, f64> = Dimensioned(1.0);
// pub static s: Dimensioned<Time, f64> = Dimensioned(1.0);
// pub static A: Dimensioned<Current, f64> = Dimensioned(1.0);
// pub static K: Dimensioned<Temp, f64> = Dimensioned(1.0);
// pub static cd: Dimensioned<Intensity, f64> = Dimensioned(1.0);
// pub static mol: Dimensioned<Quantity, f64> = Dimensioned(1.0);































// impl Dim for SI {}
// impl Mul<SI, SI> for SI {
//     fn mul(&self, other: &SI) -> SI {
//         SI {
//             meters: self.meters + other.meters, kilograms: self.kilograms + other.kilograms,
//             seconds: self.seconds + other.seconds, amperes: self.amperes + other.amperes,
//             kelvin: self.kelvin + other.kelvin, candela: self.candela + other.candela,
//             moles: self.moles + other.moles
//         }
//     }
// }
// impl Div<SI, SI> for SI {
//     fn div(&self, other: &SI) -> SI {
//         SI {
//             meters: self.meters - other.meters, kilograms: self.kilograms - other.kilograms,
//             seconds: self.seconds - other.seconds, amperes: self.amperes - other.amperes,
//             kelvin: self.kelvin - other.kelvin, candela: self.candela - other.candela,
//             moles: self.moles - other.moles
//         }
//     }
// }
// impl PartialEq for SI {
//     fn eq(&self, other: &SI) -> bool {
//         self.meters == other.meters && self.kilograms == other.kilograms &&
//             self.seconds == other.seconds && self.amperes == other.amperes &&
//             self.kelvin == other.kelvin && self.candela == other.candela &&
//             self.moles == other.moles
//     }
//     fn ne(&self, other: &SI) -> bool {
//         !(self == other)
//     }
// }

// impl Clone for SI {
//     fn clone(&self) -> SI {
//         SI {
//             meters: self.meters, kilograms: self.kilograms, seconds: self.seconds,
//             amperes: self.amperes, kelvin: self.kelvin, candela: self.candela, moles: self.moles
//         }
//     }
// }

// impl fmt::Show for SI {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut unit_string = String::new();
//         let mut first = true;
//         for (unit, exp) in ["m", "kg", "s", "A", "K", "cd", "mol"].iter().zip([self.meters, self.kilograms, self.seconds, self.amperes, self.kelvin, self.candela, self.moles].iter()) {
//             if *exp != 0 {
//                 if first == false {
//                     unit_string.push('*');
//                 }
//                 unit_string.push_str(*unit);
//                 if *exp != 1 {
//                     unit_string = format!("{}^{}", unit_string, exp);
//                 }
//                 first = false;
//             }
//         }
//         write!(f, "{}", unit_string)
//     }
// }
