#![allow(non_upper_case_globals)]
use peano::*;
use dimensioned::*;

/// Units with just meters and seconds as a test case
pub struct MS<Meter: PInt, Second: PInt>;

impl<Meter: PInt, Second: PInt> Dim for MS<Meter, Second> {}


impl<L1, L2, R1, R2>
    AddDim<MS<R1, R2>> for MS<L1, L2>
    where L1: PInt + AddPeano<R1>, L2: PInt + AddPeano<R2>, R1: PInt, R2: PInt {
              type Output = MS<<L1 as AddPeano<R1>>::Output, <L2 as AddPeano<R2>>::Output>;
}

impl<L1, L2, R1, R2>
    SubDim<MS<R1, R2>> for MS<L1, L2>
    where L1: PInt + SubPeano<R1>, L2: PInt + SubPeano<R2>, R1: PInt, R2: PInt {
              type Output = MS<<L1 as SubPeano<R1>>::Output, <L2 as SubPeano<R2>>::Output>;
    }



pub type Unitless = MS<Zero, Zero>;
impl Dimensionless for Unitless {}
pub type Meter = MS<One, Zero>;
pub type Second = MS<Zero, One>;


pub static one: Dimensioned<Unitless, f64> = Dimensioned(1.0);
pub static m: Dimensioned<Meter, f64> = Dimensioned(1.0);
pub static s: Dimensioned<Second, f64> = Dimensioned(1.0);

#[test]
fn test_ms() {
    let x = m;
    let y = m*7.3;
    let t = s;
    let n = one*3.5;
    let v1 = x/t;
    let v2 = y/t;
    // fixme: can't do this yet!
    //3.4 * x;
    n + x/y;
    x+x;
    n + one*3.2;
    x*one;
    one*x;
    v1 + v2;
    -x;
}




















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
