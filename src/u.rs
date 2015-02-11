
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//use peano::*;
use dimensioned::*;

pub struct U<Unit: Peano>;
impl<Unit: Peano> Dimension for U<Unit> {}


impl<Unit1, Unit2> KeepDim<U<Unit2>> for U<Unit1>
where Unit1: Peano + KeepPeano<Unit2>, Unit2: Peano
{
  type Output = U<<Unit1 as KeepPeano<Unit2>>::Output>;
}

impl<Unit1, Unit2> AddDim<U<Unit2>> for U<Unit1>
where Unit1: Peano + AddPeano<Unit2>, Unit2: Peano
{
  type Output = U<<Unit1 as AddPeano<Unit2>>::Output>;
}

impl<Unit1, Unit2> SubDim<U<Unit2>> for U<Unit1>
where Unit1: Peano + SubPeano<Unit2>, Unit2: Peano
{
  type Output = U<<Unit1 as SubPeano<Unit2>>::Output>;
}

impl<Unit, RHS> MulDim<RHS> for U<Unit>
where Unit: Peano + MulPeano<RHS>, RHS: Peano
{
  type Output = U<<Unit as MulPeano<RHS>>::Output>;
}

impl<Unit, RHS> DivDim<RHS> for U<Unit>
where Unit: Peano + DivPeano<RHS>, RHS: Peano
{
  type Output = U<<Unit as DivPeano<RHS>>::Output>;
}

impl<Unit> DimToString for U<Unit>
  where Unit: ToInt {
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
