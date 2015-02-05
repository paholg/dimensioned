
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_upper_case_globals)]
use peano::*;
use dimensioned::*;

pub struct SI<Meter: PInt, Kilogram: PInt, Second: PInt, Amp: PInt, Kelvin: PInt, Candela: PInt, Mole: PInt>;
impl<Meter: PInt, Kilogram: PInt, Second: PInt, Amp: PInt, Kelvin: PInt, Candela: PInt, Mole: PInt> Dim for SI<Meter, Kilogram, Second, Amp, Kelvin, Candela, Mole> {}


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
pub type Unitless = SI<Zero, Zero, Zero, Zero, Zero, Zero, Zero>;
impl Dimensionless for Unitless {}
pub type Meter = SI<Succ<Zero>, Zero, Zero, Zero, Zero, Zero, Zero>;
pub type Kilogram = SI<Zero, Succ<Zero>, Zero, Zero, Zero, Zero, Zero>;
pub type Second = SI<Zero, Zero, Succ<Zero>, Zero, Zero, Zero, Zero>;
pub type Amp = SI<Zero, Zero, Zero, Succ<Zero>, Zero, Zero, Zero>;
pub type Kelvin = SI<Zero, Zero, Zero, Zero, Succ<Zero>, Zero, Zero>;
pub type Candela = SI<Zero, Zero, Zero, Zero, Zero, Succ<Zero>, Zero>;
pub type Mole = SI<Zero, Zero, Zero, Zero, Zero, Zero, Succ<Zero>>;

pub static one: Dimensioned<Unitless, f64> = Dimensioned(1.0);
pub static m: Dimensioned<Meter, f64> = Dimensioned(1.0);
pub static kg: Dimensioned<Kilogram, f64> = Dimensioned(1.0);
pub static s: Dimensioned<Second, f64> = Dimensioned(1.0);
pub static A: Dimensioned<Amp, f64> = Dimensioned(1.0);
pub static K: Dimensioned<Kelvin, f64> = Dimensioned(1.0);
pub static cd: Dimensioned<Candela, f64> = Dimensioned(1.0);
pub static mol: Dimensioned<Mole, f64> = Dimensioned(1.0);
