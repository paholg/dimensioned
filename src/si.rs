
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//use peano::*;
use dimensioned::*;

pub struct SI<Meter: Peano, Kilogram: Peano, Second: Peano, Amp: Peano, Kelvin: Peano, Candela: Peano, Mole: Peano>;
impl<Meter: Peano, Kilogram: Peano, Second: Peano, Amp: Peano, Kelvin: Peano, Candela: Peano, Mole: Peano> Dimension for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole> {}


impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> KeepDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: Peano + KeepPeano<Meter2>, Kilogram1: Peano + KeepPeano<Kilogram2>, Second1: Peano + KeepPeano<Second2>, Amp1: Peano + KeepPeano<Amp2>, Kelvin1: Peano + KeepPeano<Kelvin2>, Candela1: Peano + KeepPeano<Candela2>, Mole1: Peano + KeepPeano<Mole2>, Meter2: Peano, Kilogram2: Peano, Second2: Peano, Amp2: Peano, Kelvin2: Peano, Candela2: Peano, Mole2: Peano
{
  type Output = SI<<Meter1 as KeepPeano<Meter2>>::Output, <Kilogram1 as KeepPeano<Kilogram2>>::Output, <Second1 as KeepPeano<Second2>>::Output, <Amp1 as KeepPeano<Amp2>>::Output, <Kelvin1 as KeepPeano<Kelvin2>>::Output, <Candela1 as KeepPeano<Candela2>>::Output, <Mole1 as KeepPeano<Mole2>>::Output>;
}

impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> AddDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: Peano + AddPeano<Meter2>, Kilogram1: Peano + AddPeano<Kilogram2>, Second1: Peano + AddPeano<Second2>, Amp1: Peano + AddPeano<Amp2>, Kelvin1: Peano + AddPeano<Kelvin2>, Candela1: Peano + AddPeano<Candela2>, Mole1: Peano + AddPeano<Mole2>, Meter2: Peano, Kilogram2: Peano, Second2: Peano, Amp2: Peano, Kelvin2: Peano, Candela2: Peano, Mole2: Peano
{
  type Output = SI<<Meter1 as AddPeano<Meter2>>::Output, <Kilogram1 as AddPeano<Kilogram2>>::Output, <Second1 as AddPeano<Second2>>::Output, <Amp1 as AddPeano<Amp2>>::Output, <Kelvin1 as AddPeano<Kelvin2>>::Output, <Candela1 as AddPeano<Candela2>>::Output, <Mole1 as AddPeano<Mole2>>::Output>;
}

impl<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1, Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2> SubDim<SI<Meter2, Kilogram2, Second2, Amp2, Kelvin2, Candela2, Mole2>> for SI<Meter1, Kilogram1, Second1, Amp1, Kelvin1, Candela1, Mole1>
where Meter1: Peano + SubPeano<Meter2>, Kilogram1: Peano + SubPeano<Kilogram2>, Second1: Peano + SubPeano<Second2>, Amp1: Peano + SubPeano<Amp2>, Kelvin1: Peano + SubPeano<Kelvin2>, Candela1: Peano + SubPeano<Candela2>, Mole1: Peano + SubPeano<Mole2>, Meter2: Peano, Kilogram2: Peano, Second2: Peano, Amp2: Peano, Kelvin2: Peano, Candela2: Peano, Mole2: Peano
{
  type Output = SI<<Meter1 as SubPeano<Meter2>>::Output, <Kilogram1 as SubPeano<Kilogram2>>::Output, <Second1 as SubPeano<Second2>>::Output, <Amp1 as SubPeano<Amp2>>::Output, <Kelvin1 as SubPeano<Kelvin2>>::Output, <Candela1 as SubPeano<Candela2>>::Output, <Mole1 as SubPeano<Mole2>>::Output>;
}

impl<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole, RHS> MulDim<RHS> for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole>
where Meter: Peano + MulPeano<RHS>, Kilogram: Peano + MulPeano<RHS>, Second: Peano + MulPeano<RHS>, Amp: Peano + MulPeano<RHS>, Kelvin: Peano + MulPeano<RHS>, Candela: Peano + MulPeano<RHS>, Mole: Peano + MulPeano<RHS>, RHS: Peano
{
  type Output = SI<<Meter as MulPeano<RHS>>::Output, <Kilogram as MulPeano<RHS>>::Output, <Second as MulPeano<RHS>>::Output, <Amp as MulPeano<RHS>>::Output, <Kelvin as MulPeano<RHS>>::Output, <Candela as MulPeano<RHS>>::Output, <Mole as MulPeano<RHS>>::Output>;
}

impl<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole, RHS> DivDim<RHS> for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole>
where Meter: Peano + DivPeano<RHS>, Kilogram: Peano + DivPeano<RHS>, Second: Peano + DivPeano<RHS>, Amp: Peano + DivPeano<RHS>, Kelvin: Peano + DivPeano<RHS>, Candela: Peano + DivPeano<RHS>, Mole: Peano + DivPeano<RHS>, RHS: Peano
{
  type Output = SI<<Meter as DivPeano<RHS>>::Output, <Kilogram as DivPeano<RHS>>::Output, <Second as DivPeano<RHS>>::Output, <Amp as DivPeano<RHS>>::Output, <Kelvin as DivPeano<RHS>>::Output, <Candela as DivPeano<RHS>>::Output, <Mole as DivPeano<RHS>>::Output>;
}

impl<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole> DimToString for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole>
  where Meter: ToInt, Kilogram: ToInt, Second: ToInt, Amp: ToInt, Kelvin: ToInt, Candela: ToInt, Mole: ToInt {
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
pub static meter: Dim<Meter, f64> = Dim(1.0);
pub static kilogram: Dim<Kilogram, f64> = Dim(1.0);
pub static second: Dim<Second, f64> = Dim(1.0);
pub static amp: Dim<Amp, f64> = Dim(1.0);
pub static kelvin: Dim<Kelvin, f64> = Dim(1.0);
pub static candela: Dim<Candela, f64> = Dim(1.0);
pub static mole: Dim<Mole, f64> = Dim(1.0);
