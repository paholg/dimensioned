
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use peano::*;
use dimensioned::*;

pub struct SI<Meter: PInt, Kilogram: PInt, Second: PInt, Amp: PInt, Kelvin: PInt, Candela: PInt, Mole: PInt>;
impl<Meter: PInt, Kilogram: PInt, Second: PInt, Amp: PInt, Kelvin: PInt, Candela: PInt, Mole: PInt> Dimension for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole> {}


impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> KeepDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: PInt + KeepPeano<Meter2>, Kilogram1: PInt + KeepPeano<Kilogram2>, Second1: PInt + KeepPeano<Second2>, Amp1: PInt + KeepPeano<Amp2>, Kelvin1: PInt + KeepPeano<Kelvin2>, Candela1: PInt + KeepPeano<Candela2>, Mole1: PInt + KeepPeano<Mole2>, Meter2: PInt, Kilogram2: PInt, Second2: PInt, Amp2: PInt, Kelvin2: PInt, Candela2: PInt, Mole2: PInt
{
  type Output = SI<<Meter1 as KeepPeano<Meter2>>::Output, <Kilogram1 as KeepPeano<Kilogram2>>::Output, <Second1 as KeepPeano<Second2>>::Output, <Amp1 as KeepPeano<Amp2>>::Output, <Kelvin1 as KeepPeano<Kelvin2>>::Output, <Candela1 as KeepPeano<Candela2>>::Output, <Mole1 as KeepPeano<Mole2>>::Output>;
}

impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> AddDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: PInt + AddPeano<Meter2>, Kilogram1: PInt + AddPeano<Kilogram2>, Second1: PInt + AddPeano<Second2>, Amp1: PInt + AddPeano<Amp2>, Kelvin1: PInt + AddPeano<Kelvin2>, Candela1: PInt + AddPeano<Candela2>, Mole1: PInt + AddPeano<Mole2>, Meter2: PInt, Kilogram2: PInt, Second2: PInt, Amp2: PInt, Kelvin2: PInt, Candela2: PInt, Mole2: PInt
{
  type Output = SI<<Meter1 as AddPeano<Meter2>>::Output, <Kilogram1 as AddPeano<Kilogram2>>::Output, <Second1 as AddPeano<Second2>>::Output, <Amp1 as AddPeano<Amp2>>::Output, <Kelvin1 as AddPeano<Kelvin2>>::Output, <Candela1 as AddPeano<Candela2>>::Output, <Mole1 as AddPeano<Mole2>>::Output>;
}

impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> SubDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: PInt + SubPeano<Meter2>, Kilogram1: PInt + SubPeano<Kilogram2>, Second1: PInt + SubPeano<Second2>, Amp1: PInt + SubPeano<Amp2>, Kelvin1: PInt + SubPeano<Kelvin2>, Candela1: PInt + SubPeano<Candela2>, Mole1: PInt + SubPeano<Mole2>, Meter2: PInt, Kilogram2: PInt, Second2: PInt, Amp2: PInt, Kelvin2: PInt, Candela2: PInt, Mole2: PInt
{
  type Output = SI<<Meter1 as SubPeano<Meter2>>::Output, <Kilogram1 as SubPeano<Kilogram2>>::Output, <Second1 as SubPeano<Second2>>::Output, <Amp1 as SubPeano<Amp2>>::Output, <Kelvin1 as SubPeano<Kelvin2>>::Output, <Candela1 as SubPeano<Candela2>>::Output, <Mole1 as SubPeano<Mole2>>::Output>;
}

impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> MulDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: PInt + MulPeano<Meter2>, Kilogram1: PInt + MulPeano<Kilogram2>, Second1: PInt + MulPeano<Second2>, Amp1: PInt + MulPeano<Amp2>, Kelvin1: PInt + MulPeano<Kelvin2>, Candela1: PInt + MulPeano<Candela2>, Mole1: PInt + MulPeano<Mole2>, Meter2: PInt, Kilogram2: PInt, Second2: PInt, Amp2: PInt, Kelvin2: PInt, Candela2: PInt, Mole2: PInt
{
  type Output = SI<<Meter1 as MulPeano<Meter2>>::Output, <Kilogram1 as MulPeano<Kilogram2>>::Output, <Second1 as MulPeano<Second2>>::Output, <Amp1 as MulPeano<Amp2>>::Output, <Kelvin1 as MulPeano<Kelvin2>>::Output, <Candela1 as MulPeano<Candela2>>::Output, <Mole1 as MulPeano<Mole2>>::Output>;
}

impl<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole> DimToString for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole>
  where Meter: PInt, Kilogram: PInt, Second: PInt, Amp: PInt, Kelvin: PInt, Candela: PInt, Mole: PInt {
    fn to_string() -> String {

      let m_str = match <Meter as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("m", "".to_string()),
            n => ("m^", (n/1).to_string())
          };
      let kg_str = match <Kilogram as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("kg", "".to_string()),
            n => ("kg^", (n/1).to_string())
          };
      let s_str = match <Second as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("s", "".to_string()),
            n => ("s^", (n/1).to_string())
          };
      let A_str = match <Amp as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("A", "".to_string()),
            n => ("A^", (n/1).to_string())
          };
      let K_str = match <Kelvin as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("K", "".to_string()),
            n => ("K^", (n/1).to_string())
          };
      let cd_str = match <Candela as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("cd", "".to_string()),
            n => ("cd^", (n/1).to_string())
          };
      let mol_str = match <Mole as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("mol", "".to_string()),
            n => ("mol^", (n/1).to_string())
          };
      format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}", m_str.0, m_str.1, kg_str.0, kg_str.1, s_str.0, s_str.1, A_str.0, A_str.1, K_str.0, K_str.1, cd_str.0, cd_str.1, mol_str.0, mol_str.1)
  }
}
pub type Unitless = SI<Zero, Zero, Zero, Zero, Zero, Zero, Zero>;
impl Dimensionless for Unitless {}
pub type Meter = SI<Succ<Zero>, Zero, Zero, Zero, Zero, Zero, Zero>;
pub type Kilogram = SI<Zero, Succ<Zero>, Zero, Zero, Zero, Zero, Zero>;
pub type Second = SI<Zero, Zero, Succ<Zero>, Zero, Zero, Zero, Zero>;
pub type Amp = SI<Zero, Zero, Zero, Succ<Zero>, Zero, Zero, Zero>;
pub type Kelvin = SI<Zero, Zero, Zero, Zero, Succ<Zero>, Zero, Zero>;
pub type Candela = SI<Zero, Zero, Zero, Zero, Zero, Succ<Zero>, Zero>;
pub type Mole = SI<Zero, Zero, Zero, Zero, Zero, Zero, Succ<Zero>>;

pub static one: Dim<Unitless, f64> = Dim(1.0);
pub static m: Dim<Meter, f64> = Dim(1.0);
pub static kg: Dim<Kilogram, f64> = Dim(1.0);
pub static s: Dim<Second, f64> = Dim(1.0);
pub static A: Dim<Amp, f64> = Dim(1.0);
pub static K: Dim<Kelvin, f64> = Dim(1.0);
pub static cd: Dim<Candela, f64> = Dim(1.0);
pub static mol: Dim<Mole, f64> = Dim(1.0);
