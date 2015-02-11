use std::ops::Mul;
use std::fmt;

// Peano -----------------------------------------------------------------------
trait Peano {}
trait NonZero {}
trait NonNeg {}

struct Zero;
struct Succ<T: NonNeg>;

impl Peano for Zero {}
impl NonNeg for Zero {}

impl<T: NonNeg> Peano for Succ<T> {}
impl<T: NonNeg> NonNeg for Succ<T> {}

trait PInt: Peano + AddPeano + ToInt {}
impl PInt for Zero {}

impl<T: PInt + NonNeg> PInt for Succ<T>
    where Succ<T>: Peano + AddPeano + ToInt {}

trait AddPeano<RHS = Self> {
    type Output;
}

/// Adding things to zero (e.g. 0 + 3)
impl<RHS: Peano> AddPeano<RHS> for Zero {
    type Output = RHS;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<T, RHS> AddPeano<Succ<RHS>> for Succ<T>
    where T: NonNeg + AddPeano<Succ<RHS>>, RHS: NonNeg {
        type Output = Succ<<T as AddPeano<Succ<RHS>>>::Output>;
    }



trait ToInt {
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<T: ToInt + NonNeg> ToInt for Succ<T> {
    fn to_int() -> i32 { 1 + <T as ToInt>::to_int() }
}

// Unit system -----------------------------------------------------------------------
struct Units<Meter: PInt>;

trait Dimension {}
impl<Meter: PInt> Dimension for Units<Meter> {}

trait AddDim<RHS = Self>: Dimension {
    type Output;
}

impl<Meter1, Meter2> AddDim<Units<Meter2>> for Units<Meter1>
    where Meter1: PInt + AddPeano<Meter2>, Meter2: PInt {
        type Output = Units<<Meter1 as AddPeano<Meter2>>::Output>;
    }

trait DimToString {
    fn to_string() -> String;
}

impl<Meter: ToInt> DimToString for Units<Meter> {
    fn to_string() -> String {
      let m_str = match <Meter as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("m", "".to_string()),
            n => ("m^", (n/1).to_string())
          };
      format!("{}{}", m_str.0, m_str.1)
    }
}

// Dim -------------------------------------------------------------------------
struct Dim<T, V>(pub V);
impl<T, V: Copy> Copy for Dim<T, V> {}

impl<Tl, Vl, Tr, Vr> Mul<Dim<Tr,Vr>> for Dim<Tl, Vl>
    where Tl: AddDim<Tr>, Vl: Mul<Vr> {
        type Output = Dim<<Tl as AddDim<Tr>>::Output, <Vl as Mul<Vr>>::Output>;
        fn mul(self, rhs: Dim<Tr, Vr>) -> <Self as Mul<Dim<Tr, Vr>>>::Output {
            Dim( (self.0)*(rhs.0) )
        }
    }

impl<T, V> fmt::Display for Dim<T, V> where T: DimToString, V: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, <T as DimToString>::to_string())
    }
}


type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;
type Four = Succ<Three>;
type Five = Succ<Four>;



static m: Dim<Units<One>, f64> = Dim(1.0);

fn main() {
    let x = m;
    let y = x*x*x*x*x*x*x*x*x*x;
    println!("y: {}", y);
}
