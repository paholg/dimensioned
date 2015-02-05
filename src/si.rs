
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_upper_case_globals)]
use peano::*;
use dimensioned::*;

pub struct SI<Length: PInt, Mass: PInt, Time: PInt, Current: PInt, Temp: PInt, Intensity: PInt, Quantity: PInt>;
impl<Length: PInt, Mass: PInt, Time: PInt, Current: PInt, Temp: PInt, Intensity: PInt, Quantity: PInt> Dim for SI<Length, Mass, Time, Current, Temp, Intensity, Quantity> {}


impl<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1, Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2> AddDim<SI<Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2>> for SI<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1>
where Length1: PInt + AddPeano<Length2>, Mass1: PInt + AddPeano<Mass2>, Time1: PInt + AddPeano<Time2>, Current1: PInt + AddPeano<Current2>, Temp1: PInt + AddPeano<Temp2>, Intensity1: PInt + AddPeano<Intensity2>, Quantity1: PInt + AddPeano<Quantity2>, Length2: PInt, Mass2: PInt, Time2: PInt, Current2: PInt, Temp2: PInt, Intensity2: PInt, Quantity2: PInt
{
  type Output = SI<<Length1 as AddPeano<Length2>>::Output, <Mass1 as AddPeano<Mass2>>::Output, <Time1 as AddPeano<Time2>>::Output, <Current1 as AddPeano<Current2>>::Output, <Temp1 as AddPeano<Temp2>>::Output, <Intensity1 as AddPeano<Intensity2>>::Output, <Quantity1 as AddPeano<Quantity2>>::Output>;
}

impl<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1, Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2> SubDim<SI<Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2>> for SI<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1>
where Length1: PInt + SubPeano<Length2>, Mass1: PInt + SubPeano<Mass2>, Time1: PInt + SubPeano<Time2>, Current1: PInt + SubPeano<Current2>, Temp1: PInt + SubPeano<Temp2>, Intensity1: PInt + SubPeano<Intensity2>, Quantity1: PInt + SubPeano<Quantity2>, Length2: PInt, Mass2: PInt, Time2: PInt, Current2: PInt, Temp2: PInt, Intensity2: PInt, Quantity2: PInt
{
  type Output = SI<<Length1 as SubPeano<Length2>>::Output, <Mass1 as SubPeano<Mass2>>::Output, <Time1 as SubPeano<Time2>>::Output, <Current1 as SubPeano<Current2>>::Output, <Temp1 as SubPeano<Temp2>>::Output, <Intensity1 as SubPeano<Intensity2>>::Output, <Quantity1 as SubPeano<Quantity2>>::Output>;
}

impl<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1, Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2> MulDim<SI<Length2, Mass2, Time2, Current2, Temp2, Intensity2, Quantity2>> for SI<Length1, Mass1, Time1, Current1, Temp1, Intensity1, Quantity1>
where Length1: PInt + MulPeano<Length2>, Mass1: PInt + MulPeano<Mass2>, Time1: PInt + MulPeano<Time2>, Current1: PInt + MulPeano<Current2>, Temp1: PInt + MulPeano<Temp2>, Intensity1: PInt + MulPeano<Intensity2>, Quantity1: PInt + MulPeano<Quantity2>, Length2: PInt, Mass2: PInt, Time2: PInt, Current2: PInt, Temp2: PInt, Intensity2: PInt, Quantity2: PInt
{
  type Output = SI<<Length1 as MulPeano<Length2>>::Output, <Mass1 as MulPeano<Mass2>>::Output, <Time1 as MulPeano<Time2>>::Output, <Current1 as MulPeano<Current2>>::Output, <Temp1 as MulPeano<Temp2>>::Output, <Intensity1 as MulPeano<Intensity2>>::Output, <Quantity1 as MulPeano<Quantity2>>::Output>;
}
pub type Unitless = SI<Zero, Zero, Zero, Zero, Zero, Zero, Zero>;
impl Dimensionless for Unitless {}
pub type Length = SI<Succ<Zero>, Zero, Zero, Zero, Zero, Zero, Zero>;
pub type Mass = SI<Zero, Succ<Zero>, Zero, Zero, Zero, Zero, Zero>;
pub type Time = SI<Zero, Zero, Succ<Zero>, Zero, Zero, Zero, Zero>;
pub type Current = SI<Zero, Zero, Zero, Succ<Zero>, Zero, Zero, Zero>;
pub type Temp = SI<Zero, Zero, Zero, Zero, Succ<Zero>, Zero, Zero>;
pub type Intensity = SI<Zero, Zero, Zero, Zero, Zero, Succ<Zero>, Zero>;
pub type Quantity = SI<Zero, Zero, Zero, Zero, Zero, Zero, Succ<Zero>>;

pub static one: Dimensioned<Unitless, f64> = Dimensioned(1.0);
pub static m: Dimensioned<Length, f64> = Dimensioned(1.0);
pub static kg: Dimensioned<Mass, f64> = Dimensioned(1.0);
pub static s: Dimensioned<Time, f64> = Dimensioned(1.0);
pub static A: Dimensioned<Current, f64> = Dimensioned(1.0);
pub static K: Dimensioned<Temp, f64> = Dimensioned(1.0);
pub static cd: Dimensioned<Intensity, f64> = Dimensioned(1.0);
pub static mol: Dimensioned<Quantity, f64> = Dimensioned(1.0);
