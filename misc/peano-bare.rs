use std::ops::Mul;
use std::fmt;

// Peano -----------------------------------------------------------------------
struct Zero;
struct Succ<N>;

trait AddPeano<N> {
    type Output;
}

/// Adding things to zero (e.g. 0 + 3)
impl<N> AddPeano<N> for Zero {
    type Output = N;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<M, N> AddPeano<N> for Succ<M> where N: AddPeano<M> {
        type Output = Succ<<N as AddPeano<M>>::Output>;
}

trait ToInt {
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<N> ToInt for Succ<N> where N: ToInt {
    fn to_int() -> i32 { 1 + <N as ToInt>::to_int() }
}

// Unit system -----------------------------------------------------------------------
struct Units<Meter, Second>;

trait AddDim<R> {
    type Output;
}

impl<Ml, Sl, Mr, Sr> AddDim<Units<Mr, Sr>> for Units<Ml, Sl>
    where Ml: AddPeano<Mr>, Sl: AddPeano<Sr> {
        type Output = Units<<Ml as AddPeano<Mr>>::Output, <Sl as AddPeano<Sr>>::Output>;
}

trait DimToString {
    fn to_string() -> String;
}

impl<Meter, Second> DimToString for Units<Meter, Second> where Meter: ToInt, Second: ToInt {
    fn to_string() -> String {
      let m_str = match <Meter as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("m", "".to_string()),
            n => ("m^", (n/1).to_string())
          };
      let s_str = match <Second as ToInt>::to_int() {
            0 => ("", "".to_string()),
            1 => ("s", "".to_string()),
            n => ("s^", (n/1).to_string())
          };
      format!("{}{}{}{}", m_str.0, m_str.1, s_str.0, s_str.1)
    }
}

// Dim -------------------------------------------------------------------------
struct Dim<U, V>(pub V);
impl<U, V> Copy for Dim<U, V> where V: Copy {}

impl<Ul, Vl, Ur, Vr> Mul<Dim<Ur,Vr>> for Dim<Ul, Vl>
    where Ul: AddDim<Ur>, Vl: Mul<Vr> {
        type Output = Dim<<Ul as AddDim<Ur>>::Output, <Vl as Mul<Vr>>::Output>;

        fn mul(self, rhs: Dim<Ur, Vr>) -> <Self as Mul<Dim<Ur, Vr>>>::Output {
            Dim( (self.0)*(rhs.0) )
        }
}

impl<U, V> fmt::Display for Dim<U, V> where U: DimToString, V: fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{} {}", self.0, <U as DimToString>::to_string())
    }
}

type One = Succ<Zero>;
type Two = Succ<One>;
type Three = Succ<Two>;
type Four = Succ<Three>;
type Five = Succ<Four>;



static m: Dim<Units<One, Zero>, f64> = Dim(1.0);
static s: Dim<Units<Zero, One>, f64> = Dim(1.0);

fn main() {
    let x = m;
    let t = s;
    let a = x*x;
    println!("y: {}, t: {}", a, t);
}
