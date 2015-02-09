
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//use peano::*;
use dimensioned::*;

pub struct CGS<Centimeter: PInt, Gram: PInt, Second: PInt>;
impl<Centimeter: PInt, Gram: PInt, Second: PInt> Dimension for CGS<Centimeter, Gram, Second> {}


impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> KeepDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: PInt + KeepPeano<Centimeter2>, Gram1: PInt + KeepPeano<Gram2>, Second1: PInt + KeepPeano<Second2>, Centimeter2: PInt, Gram2: PInt, Second2: PInt
{
  type Output = CGS<<Centimeter1 as KeepPeano<Centimeter2>>::Output, <Gram1 as KeepPeano<Gram2>>::Output, <Second1 as KeepPeano<Second2>>::Output>;
}

impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> AddDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: PInt + AddPeano<Centimeter2>, Gram1: PInt + AddPeano<Gram2>, Second1: PInt + AddPeano<Second2>, Centimeter2: PInt, Gram2: PInt, Second2: PInt
{
  type Output = CGS<<Centimeter1 as AddPeano<Centimeter2>>::Output, <Gram1 as AddPeano<Gram2>>::Output, <Second1 as AddPeano<Second2>>::Output>;
}

impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> SubDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: PInt + SubPeano<Centimeter2>, Gram1: PInt + SubPeano<Gram2>, Second1: PInt + SubPeano<Second2>, Centimeter2: PInt, Gram2: PInt, Second2: PInt
{
  type Output = CGS<<Centimeter1 as SubPeano<Centimeter2>>::Output, <Gram1 as SubPeano<Gram2>>::Output, <Second1 as SubPeano<Second2>>::Output>;
}

impl<Centimeter, Gram, Second, RHS> MulDim<RHS> for CGS<Centimeter, Gram, Second>
where Centimeter: PInt + MulPeano<RHS>, Gram: PInt + MulPeano<RHS>, Second: PInt + MulPeano<RHS>, RHS: PInt
{
  type Output = CGS<<Centimeter as MulPeano<RHS>>::Output, <Gram as MulPeano<RHS>>::Output, <Second as MulPeano<RHS>>::Output>;
}

impl<Centimeter, Gram, Second, RHS> DivDim<RHS> for CGS<Centimeter, Gram, Second>
where Centimeter: PInt + DivPeano<RHS>, Gram: PInt + DivPeano<RHS>, Second: PInt + DivPeano<RHS>, RHS: PInt
{
  type Output = CGS<<Centimeter as DivPeano<RHS>>::Output, <Gram as DivPeano<RHS>>::Output, <Second as DivPeano<RHS>>::Output>;
}

impl<Centimeter, Gram, Second> DimToString for CGS<Centimeter, Gram, Second>
  where Centimeter: PInt, Gram: PInt, Second: PInt {
    fn to_string() -> String {

      let cm_str = match <Centimeter as ToInt>::to_int() {
            0 => ("", "".to_string()),
            2 => ("cm", "".to_string()),
            n => ("cm^", (n/2).to_string())
          };
      let g_str = match <Gram as ToInt>::to_int() {
            0 => ("", "".to_string()),
            2 => ("g", "".to_string()),
            n => ("g^", (n/2).to_string())
          };
      let s_str = match <Second as ToInt>::to_int() {
            0 => ("", "".to_string()),
            2 => ("s", "".to_string()),
            n => ("s^", (n/2).to_string())
          };
      format!("{}{}{}{}{}{}", cm_str.0, cm_str.1, g_str.0, g_str.1, s_str.0, s_str.1)
  }
}
pub type Unitless = CGS<Zero, Zero, Zero>;
impl Dimensionless for Unitless {}
pub type Centimeter = CGS<Succ<Succ<Zero>>, Zero, Zero>;
pub type Gram = CGS<Zero, Succ<Succ<Zero>>, Zero>;
pub type Second = CGS<Zero, Zero, Succ<Succ<Zero>>>;

pub static one: Dim<Unitless, f64> = Dim(1.0);
pub static centimeter: Dim<Centimeter, f64> = Dim(1.0);
pub static gram: Dim<Gram, f64> = Dim(1.0);
pub static second: Dim<Second, f64> = Dim(1.0);
