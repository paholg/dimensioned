#![allow(non_upper_case_globals)]
use peano::*;
use dimensioned::*;
//use std::num::rational::Ratio;

/// Units with just meters and seconds as a test case
pub struct MS<Meter: PInt, Second: PInt>;
impl<M, S> DimToString for MS<M, S>
    where M: PInt, S: PInt {
        fn to_string() -> String {
            let mstr = match <M as ToInt>::to_int() {
                0 => ("", "".to_string()),
                1 => ("m", "".to_string()),
                n => ("m^", (n/1).to_string())
            };
            let sstr = match <S as ToInt>::to_int() {
                0 => ("", "".to_string()),
                1 => ("s", "".to_string()),
                n => ("s^", (n/1).to_string())
            };
            // let sstr = match <S as ToInt>::to_int() {
            //     0 => "",
            //     1 => "s",
            //     n => concat!("s^", Ratio::new(n,1).reduced().to_str_radix(10).as_slice() )
            // };
            format!("{}{}{}{}", mstr.0, mstr.1, sstr.0, sstr.1)
        }
    }

impl<Meter: PInt, Second: PInt> Dimension for MS<Meter, Second> {}


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


pub static one: Dim<Unitless, f64> = Dim(1.0);
pub static m: Dim<Meter, f64> = Dim(1.0);
pub static s: Dim<Second, f64> = Dim(1.0);

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
