// use std::fmt;
// use core::str::FromStr;
// use std::num::Float;
// // use si::SI;
// use std::clone::Clone;
use std::ops::{Add, Sub, Mul, Div};
// pub trait Dim: PartialEq + fmt::Show + Mul<Self, Self> + Div<Self, Self> {
// }

enum P {
    Zero,
    Succ<Zero>,
}

trait Peano {
    fn add_one(&self) -> Succ<Self>;
}

trait Natural: Peano {
    fn sub_one<T>(N: Succ<T>) -> T where T: Peano {  }
}

struct Zero;
struct Succ<T: Peano>;


impl Peano for Zero {
    fn add_one(&self) -> Succ<Zero> { Succ::<Zero> }
}

impl<T: Peano> Peano for Succ<T> {
    fn add_one(&self) -> Succ<Self> { Succ::<Self> }
}

// impl<T: Peano> Natural for Succ<T> {
//     fn sub_one(&self) ->
// }


// impl<T: Peano> Add for Zero {
//     type Output = T;

//     fn add(self, _rhs: T) -> T {
//         _rhs
//     }
// }


// trait AddAss<Rhs> {
//     type Sum;
//     fn add(&self, rhs: &Rhs) -> <Self as AddAss<Rhs>>::Sum;
// }

// impl<Rhs: Peano> AddAss<Rhs> for Zero {
//     type Sum = Rhs;
//     fn add(&self, rhs: &Rhs) -> Sum { rhs }
// }

// type One = Succ<Zero>;
// type Two = Succ<One>;
// type Three = Succ<Two>;



//struct Two;

// impl<T: Peano> Add<Zero> for T {
//     fn add(&self, rhs: &Zero) -> T { self }
// }

// impl Add<Two, Two> for Zero {
//     fn add(&self, rhs: &Two) -> Two { Two }
// }

// impl Add<One, Two> for One {
//     fn add(&self, rhs: &One) -> Two { Two }
// }

// pub trait Length {
//     type pow;
// }

// pub struct Meters;

// impl Length for Meters {
//     type pow = 1;
// }

// #[test]
// fn add() {
//     Zero + One;
//     Zero + Two;
// }






// /// The dimension trait, shared by all of various unit systems. Each unit system is
// /// stored as a struct with various integer fields, each expressing the power to which
// /// it holds each dimensional quantity.

// /// For example, if an object of type `SIUnits` has the field `meters = 2` and all other
// /// fields equal to `0`, then this represents an area.
// pub trait Dim: fmt::Show + Mul<Self, Self> + Div<Self, Self> + PartialEq + Clone {
//     fn to_si(&self) -> SI;

//     fn unitless() -> Dimensioned<Self>;
//     fn meters() -> Dimensioned<Self>;
//     fn grams() -> Dimensioned<Self>;
//     fn seconds() -> Dimensioned<Self>;
//     fn amperes() -> Dimensioned<Self>;
//     fn kelvin() -> Dimensioned<Self>;
//     fn candela() -> Dimensioned<Self>;
//     fn moles() -> Dimensioned<Self>;
// }

// pub struct Dimensioned<T: Dim> {
//     pub val: f64,
//     pub units: T,
// }

// impl<T: Dim> Add<Dimensioned<T>, Dimensioned<T>> for Dimensioned<T> {
//     fn add(&self, other: &Dimensioned<T>) -> Dimensioned<T> {
//         assert_eq!(self.units, other.units);
//         Dimensioned{ val: self.val + other.val, units: self.units.clone() }
//     }
// }
// impl<T: Dim> Sub<Dimensioned<T>, Dimensioned<T>> for Dimensioned<T> {
//     fn sub(&self, other: &Dimensioned<T>) -> Dimensioned<T> {
//         assert_eq!(self.units, other.units);
//         Dimensioned{ val: self.val - other.val, units: self.units.clone() }
//     }
// }
// impl<T: Dim> Mul<Dimensioned<T>, Dimensioned<T>> for Dimensioned<T> {
//     fn mul(&self, other: &Dimensioned<T>) -> Dimensioned<T> {
//         Dimensioned{ val: self.val * other.val, units: self.units * other.units }
//     }
// }
// impl<T: Dim> Div<Dimensioned<T>, Dimensioned<T>> for Dimensioned<T> {
//     fn div(&self, other: &Dimensioned<T>) -> Dimensioned<T> {
//         Dimensioned{ val: self.val / other.val, units: self.units / other.units }
//     }
// }
// impl<T: Dim> Neg<Dimensioned<T>> for Dimensioned<T> {
//     fn neg(&self) -> Dimensioned<T> {
//         Dimensioned{ val: -self.val, units: self.units.clone() }
//     }
// }
// impl<T: Dim> Mul<f64, Dimensioned<T>> for Dimensioned<T> {
//     fn mul(&self, other: &f64) -> Dimensioned<T> {
//         Dimensioned{ val: self.val * other.clone(), units: self.units.clone() }
//     }
// }
// impl<T: Dim> Mul<Dimensioned<T>, Dimensioned<T>> for f64 {
//     fn mul(&self, other: &Dimensioned<T>) -> Dimensioned<T> {
//         Dimensioned{ val: *self * other.val, units: other.units.clone() }
//     }
// }
// impl<T: Dim> PartialEq for Dimensioned<T> {
//     fn eq(&self, other: &Dimensioned<T>) -> bool {
//         self.val == other.val && self.units == other.units
//     }
//     fn ne(&self, other: &Dimensioned<T>) -> bool {
//         !(self == other)
//     }
// }
// // fixme: make formatters print number of digits correctly
// impl<T: Dim> fmt::Show for Dimensioned<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{} {}", self.val, self.units)
//     }
// }
// impl<T: Dim> fmt::LowerExp for Dimensioned<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:e} {}", self.val, self.units)
//     }
// }
// impl<T: Dim> fmt::UpperExp for Dimensioned<T> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:E} {}", self.val, self.units)
//     }
// }

// // expect s to contain just one unit, with no prefix, such as m, g, or V
// fn one_unit_no_prefix<T: Dim>(s: &str) -> Option<Dimensioned<T>> {
//     use std::option::Option::{Some, None};
//     match s.len() {
//         1 => match s {
//             "A" => Some(Dim::amperes()),
//             "g" => Some(Dim::grams()),
//             "K" => Some(Dim::kelvin()),
//             "m" => Some(Dim::meters()),
//             "N" => Some(Dim::grams()*1000.0*Dim::meters()/Dim::seconds()/Dim::seconds()),
//             "s" => Some(Dim::seconds()),
//             _ => None,
//         },
//         2 => match s {
//             "cd" => Some(Dim::candela()),
//             _ => None,
//         },
//         3 => match s {
//             "mol" => Some(Dim::moles()),
//             _ => None,
//         },
//         _ => None,
//     }
// }

// // expect s to contain just one unit, such as km or mV
// fn one_unit<T: Dim>(s: &str) -> Option<Dimensioned<T>> {
//     // first try: no prefix
//     let first_try = one_unit_no_prefix(s);
//     // second try: one letter prefix
//     let second_try = match first_try {
//         Some(_) => return first_try,
//         None => if s.len() < 2 { return None; } else { one_unit_no_prefix(s.slice_from(1)) },
//     };
//     // third try: two letter prefix
//     let third_try = match second_try {
//         Some(unit) => return match s.chars().nth(0).unwrap() {
//             'Y' => Some(yotta(unit)),
//             'Z' => Some(zetta(unit)),
//             'E' => Some(exa(unit)),
//             'P' => Some(peta(unit)),
//             'T' => Some(tera(unit)),
//             'G' => Some(giga(unit)),
//             'M' => Some(mega(unit)),
//             'k' => Some(kilo(unit)),
//             'h' => Some(hecto(unit)),
//             'd' => Some(deci(unit)),
//             'c' => Some(centi(unit)),
//             'm' => Some(milli(unit)),
//             'n' => Some(nano(unit)),
//             'p' => Some(pico(unit)),
//             'f' => Some(femto(unit)),
//             'a' => Some(atto(unit)),
//             'z' => Some(zepto(unit)),
//             'y' => Some(yocto(unit)),
//             _ => None, // Safe to return here, as all two letter prefixes overlap with one-letter ones
//         },
//         None => one_unit_no_prefix(s.slice_from(2)),
//     };
//     if s.len() < 3 {
//         return None;
//     }
//     match third_try {
//         Some(unit) => match s.slice_to(2) {
//             "da" => Some(deca(unit)),
//             "mu" => Some(micro(unit)),
//             _ => None,
//         },
//         None => None,
//     }
// }

// // fixme: units and prefixes should really be a hashmap, so it's all in one place


// // impl<T: Dim> FromStr for Dimensioned<T> {
// //     fn from_str(s: &str) -> Option<Dimensioned<T>> {
// //         let re1 = regex!(r"(^-?\d+\.?\d*([eE]-?\d+)?)\s+(.*)");
// //         let (val_str, units) = match re1.captures_iter(s).nth(0) {
// //             Some(cap) => (cap.at(1), cap.at(3)),
// //             None => return Option::None,
// //         };
// //         let val: f64 = from_str(val_str).unwrap();
// //         // at(0) is full thing
// //         // at(1) is / sign, if included
// //         // at(2) is unit
// //         // at(3) is full exponent, including ^
// //         // at(4) is power, if included
// //         let re_unit = regex!(r"(/?)\s*([:alpha:]+)(\^(-?\d+))?");
// //         let mut dnum: Dimensioned<T> = Dim::unitless() * val;
// //         for cap in re_unit.captures_iter(units) {
// //             //println!("capped: {}, {}, {}, {}, {}", cap.at(0), cap.at(1), cap.at(2), cap.at(3), cap.at(4));
// //             let power: i8 = match cap.at(4) {
// //                 "" => 1,
// //                 p => from_str(p).unwrap(),
// //             } * match cap.at(1) {
// //                 "" => 1,
// //                 _ => -1,
// //             };
// //             let unit:Dimensioned<T> = match one_unit(cap.at(2)) {
// //                 Some(u) => u,
// //                 //None => panic!("Stuck at: {}", cap.at(2)),
// //                 None => return None,
// //             };
// //             if power > 0 {
// //                 for _ in range(0, power) {
// //                     dnum = dnum * unit;
// //                 }
// //             }
// //             else if power < 0 {
// //                 for _ in range(0, -power) {
// //                     dnum = dnum / unit;
// //                 }
// //             }
// //         }
// //         Option::Some(dnum)
// //     }
// // }

// // #[test]
// // fn str_parse_test() {
// //     let x1: Dimensioned<SI> = match from_str("-4.5e-7 YN") {
// //         Some(dnum) => dnum,
// //         None => panic!("Couldn't convert!"),
// //     };
// //     println!("x1: {:e}", x1);
// //     //3.0f64 * x1;
// //     //let x2: Dimensioned<SI> = nano(Dim::meters())/pico(Dim::seconds())/Dim::seconds() * 4.5e-7;
// //     //assert_eq!(x1, x2);
// // }

// fn prefix<T: Dim>(dnum: Dimensioned<T>, exp: i32) -> Dimensioned<T> {
//     Dimensioned { val: dnum.val*10f64.powi(exp), units: dnum.units }
// }

// pub fn yocto <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -24) }
// pub fn zepto <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -21) }
// pub fn atto  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -18) }
// pub fn femto <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -15) }
// pub fn pico  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -12) }
// pub fn nano  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -9)  }
// pub fn micro <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -6)  }
// pub fn milli <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -3)  }
// pub fn centi <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -2)  }
// pub fn deci  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, -1)  }
// pub fn deca  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 1)   }
// pub fn hecto <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 2)   }
// pub fn kilo  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 3)   }
// pub fn mega  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 6)   }
// pub fn giga  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 9)   }
// pub fn tera  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 12)  }
// pub fn peta  <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 15)  }
// pub fn exa   <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 18)  }
// pub fn zetta <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 21)  }
// pub fn yotta <T: Dim>(dnum: Dimensioned<T>) -> Dimensioned<T> {  prefix(dnum, 24)  }



