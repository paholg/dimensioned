
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

//use peano::*;
use dimensioned::*;

pub struct CGS<Centimeter: Peano, Gram: Peano, Second: Peano>;
impl<Centimeter: Peano, Gram: Peano, Second: Peano> Dimension for CGS<Centimeter, Gram, Second> {}


impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> KeepDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: Peano + KeepPeano<Centimeter2>, Gram1: Peano + KeepPeano<Gram2>, Second1: Peano + KeepPeano<Second2>, Centimeter2: Peano, Gram2: Peano, Second2: Peano
{
  type Output = CGS<<Centimeter1 as KeepPeano<Centimeter2>>::Output, <Gram1 as KeepPeano<Gram2>>::Output, <Second1 as KeepPeano<Second2>>::Output>;
}

impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> AddDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: Peano + AddPeano<Centimeter2>, Gram1: Peano + AddPeano<Gram2>, Second1: Peano + AddPeano<Second2>, Centimeter2: Peano, Gram2: Peano, Second2: Peano
{
  type Output = CGS<<Centimeter1 as AddPeano<Centimeter2>>::Output, <Gram1 as AddPeano<Gram2>>::Output, <Second1 as AddPeano<Second2>>::Output>;
}

impl<Centimeter1, Gram1, Second1, Centimeter2, Gram2, Second2> SubDim<CGS<Centimeter2, Gram2, Second2>> for CGS<Centimeter1, Gram1, Second1>
where Centimeter1: Peano + SubPeano<Centimeter2>, Gram1: Peano + SubPeano<Gram2>, Second1: Peano + SubPeano<Second2>, Centimeter2: Peano, Gram2: Peano, Second2: Peano
{
  type Output = CGS<<Centimeter1 as SubPeano<Centimeter2>>::Output, <Gram1 as SubPeano<Gram2>>::Output, <Second1 as SubPeano<Second2>>::Output>;
}

impl<Centimeter, Gram, Second, RHS> MulDim<RHS> for CGS<Centimeter, Gram, Second>
where Centimeter: Peano + MulPeano<RHS>, Gram: Peano + MulPeano<RHS>, Second: Peano + MulPeano<RHS>, RHS: Peano
{
  type Output = CGS<<Centimeter as MulPeano<RHS>>::Output, <Gram as MulPeano<RHS>>::Output, <Second as MulPeano<RHS>>::Output>;
}

impl<Centimeter, Gram, Second, RHS> DivDim<RHS> for CGS<Centimeter, Gram, Second>
where Centimeter: Peano + DivPeano<RHS>, Gram: Peano + DivPeano<RHS>, Second: Peano + DivPeano<RHS>, RHS: Peano
{
  type Output = CGS<<Centimeter as DivPeano<RHS>>::Output, <Gram as DivPeano<RHS>>::Output, <Second as DivPeano<RHS>>::Output>;
}

impl<Centimeter, Gram, Second> DimToString for CGS<Centimeter, Gram, Second>
  where Centimeter: ToInt, Gram: ToInt, Second: ToInt {
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
