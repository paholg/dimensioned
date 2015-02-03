#![allow(unstable)]
// #![feature(associated_types)]
// #![feature(globs)]
// #![feature(phase)]
// #![feature(slicing_syntax)]
// #![feature(macro_rules)]
// #[phase(plugin)] extern crate regex_macros;
// extern crate regex;
// extern crate core;

// pub use dimensioned::*;
// pub use si::*;
// pub mod si;
//pub mod dimensioned;

pub use peano::*;
pub mod peano;



// #[test]
// fn construction() {
//     let x = Dim::meters() * 5500.0 ;
//     let y = kilo(Dim::meters()) * 5.5;
//     let t = Dim::seconds() * 1.1;
//     println!("x: {}, y: {}, x+y: {}, a: {}, v: {}", x, y, x+y, x*y, x/t)
// }
// #[test]
// fn prefixes() {
//     //assert_eq!(Dim::meters(1e-24), yocto(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(), zepto(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-18), atto(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-15), femto(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-12), pico(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-9), nano(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-6), micro(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-3), milli(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-2), centi(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e-1), deci(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e1), deca(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e2), hecto(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e3), kilo(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e6), mega(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e9), giga(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e12), tera(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e15), peta(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e18), exa(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e21), zetta(Dim::meters(1.0)));
//     // assert_eq!(Dim::meters(1e24), yotta(Dim::meters(1.0)));
// }
