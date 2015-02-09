
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//use peano::*;
use dimensioned::*;

pub struct U<Unit: PInt>;
impl<Unit: PInt> Dimension for U<Unit> {}


impl<Unit1, Unit2> KeepDim<U<Unit2>> for U<Unit1>
where Unit1: PInt + KeepPeano<Unit2>, Unit2: PInt
{
  type Output = U<<Unit1 as KeepPeano<Unit2>>::Output>;
}

impl<Unit1, Unit2> AddDim<U<Unit2>> for U<Unit1>
where Unit1: PInt + AddPeano<Unit2>, Unit2: PInt
{
  type Output = U<<Unit1 as AddPeano<Unit2>>::Output>;
}

impl<Unit1, Unit2> SubDim<U<Unit2>> for U<Unit1>
where Unit1: PInt + SubPeano<Unit2>, Unit2: PInt
{
  type Output = U<<Unit1 as SubPeano<Unit2>>::Output>;
}

impl<Unit, RHS> MulDim<RHS> for U<Unit>
where Unit: PInt + MulPeano<RHS>, RHS: PInt
{
  type Output = U<<Unit as MulPeano<RHS>>::Output>;
}

impl<Unit, RHS> DivDim<RHS> for U<Unit>
where Unit: PInt + DivPeano<RHS>, RHS: PInt
{
  type Output = U<<Unit as DivPeano<RHS>>::Output>;
}

impl<Unit> DimToString for U<Unit>
  where Unit: PInt {
    fn to_string() -> String {

      let u_str = match <Unit as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("u", "".to_string()),
            n => ("u^", (n/1).to_string())
          };
      format!("{}{}", u_str.0, u_str.1)
  }
}
pub type Unitless = U<Zero>;
impl Dimensionless for Unitless {}
pub type Unit = U<Succ<Zero>>;

pub static one: Dim<Unitless, f64> = Dim(1.0);
pub static unit: Dim<Unit, f64> = Dim(1.0);
