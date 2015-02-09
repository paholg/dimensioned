pub struct Zero;
pub struct Succ<T: NonNeg>;
pub struct Pred<T: NonPos>;

pub type One    = Succ<Zero>;
pub type Two    = Succ<One>;
pub type Three  = Succ<Two>;
pub type Four   = Succ<Three>;
pub type Five   = Succ<Four>;
pub type Six    = Succ<Five>;
pub type Seven  = Succ<Six>;
pub type Eight  = Succ<Seven>;
pub type Nine   = Succ<Eight>;
pub type Ten    = Succ<Nine>;

pub type NegOne    = Pred<Zero>;
pub type NegTwo    = Pred<NegOne>;
pub type NegThree  = Pred<NegTwo>;
pub type NegFour   = Pred<NegThree>;
pub type NegFive   = Pred<NegFour>;
pub type NegSix    = Pred<NegFive>;
pub type NegSeven  = Pred<NegSix>;
pub type NegEight  = Pred<NegSeven>;
pub type NegNine   = Pred<NegEight>;
pub type NegTen    = Pred<NegNine>;

pub trait Peano {}
pub trait NonZero: Peano {}
pub trait NonNeg: Peano {}
pub trait NonPos: Peano {}

impl Peano for Zero {}
impl NonNeg for Zero {}
impl NonPos for Zero {}

impl<T: NonNeg> Peano for Succ<T> {}
impl<T: NonNeg> NonNeg for Succ<T> {}
impl<T: NonNeg> NonZero for Succ<T> {}

impl<T: NonPos> Peano for Pred<T> {}
impl<T: NonPos> NonPos for Pred<T> {}
impl<T: NonPos> NonZero for Pred<T> {}

pub trait PInt: Peano + AddPeano + SubPeano + MulPeano + Negate + ToInt {}
impl PInt for Zero {}
impl<T: NonNeg + PInt> PInt for Succ<T>
    where Succ<T>: AddPeano + SubPeano + MulPeano + Negate + ToInt {}
impl<T: NonPos + PInt> PInt for Pred<T>
    where Pred<T>: AddPeano + SubPeano + MulPeano + Negate + ToInt {}

impl Copy for Zero {}

pub trait AddPeano<RHS = Self> {
    type Output;
}

/// Adding things to zero (e.g. 0 + 3)
impl<RHS: Peano> AddPeano<RHS> for Zero {
    type Output = RHS;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<T: NonNeg + AddPeano<Succ<RHS>>, RHS: NonNeg> AddPeano<Succ<RHS>> for Succ<T> {
    type Output = Succ<<T as AddPeano<Succ<RHS>>>::Output>;
}
/// Adding zero to positive numbers (e.g. 3 + 0)
impl<T: NonNeg> AddPeano<Zero> for Succ<T> {
    type Output = Succ<T>;
}
/// Adding negative numbers to positive numbers (e.g. 2 + -3)
impl<T: NonNeg + AddPeano<RHS>, RHS: NonPos> AddPeano<Pred<RHS>> for Succ<T> {
    type Output = <T as AddPeano<RHS>>::Output;
}

/// Adding negative numbers (e.g. -1 + -2)
impl<T: NonPos + AddPeano<Pred<RHS>>, RHS: NonPos> AddPeano<Pred<RHS>> for Pred<T> {
    type Output = Pred<<T as AddPeano<Pred<RHS>>>::Output>;
}
/// Adding zero to negative numbers (e.g. -3 + 0)
impl<T: NonPos> AddPeano<Zero> for Pred<T> {
    type Output = Pred<T>;
}
/// Adding positive numbers to negative numbers (e.g. -2 + 3)
impl<T: NonPos + AddPeano<RHS>, RHS: NonNeg> AddPeano<Succ<RHS>> for Pred<T> {
    type Output = <T as AddPeano<RHS>>::Output;
}

pub trait Negate {
    type Output;
}
impl Negate for Zero {
    type Output = Zero;
}
impl<T: NonNeg + Negate> Negate for Succ<T> {
    type Output = Pred<<T as Negate>::Output>;
}
impl<T: NonPos + Negate> Negate for Pred<T> {
    type Output = Succ<<T as Negate>::Output>;
}

pub trait SubPeano<RHS = Self> {
    type Output;
}

/// Subtracting from zero
impl<RHS: Peano + Negate> SubPeano<RHS> for Zero {
    type Output = <RHS as Negate>::Output;
}

/// Subtracting positive numbers from positive numbers (e.g. 3 - 4)
impl<T: NonNeg + SubPeano<RHS>, RHS: NonNeg> SubPeano<Succ<RHS>> for Succ<T> {
    type Output = <T as SubPeano<RHS>>::Output;
}
/// Subtracting zero from positive numbers (e.g. 3 - 0)
impl<T: NonNeg> SubPeano<Zero> for Succ<T> {
    type Output = Succ<T>;
}
/// Subtracting negative numbers from positive numbers (e.g. 3 - -4)
impl<T: NonNeg + SubPeano<RHS>, RHS: NonPos> SubPeano<Pred<RHS>> for Succ<T> {
    type Output = Succ<Succ<<T as SubPeano<RHS>>::Output>>;
}

/// Subtracting positive numbers from negative numbers (e.g. -3 - 4)
impl<T: NonPos + SubPeano<RHS>, RHS: NonNeg> SubPeano<Succ<RHS>> for Pred<T> {
    type Output = Pred<Pred<<T as SubPeano<RHS>>::Output>>;
}
/// Subtracting zero from negative numbers (e.g. -3 - 0)
impl<T: NonPos> SubPeano<Zero> for Pred<T> {
    type Output = Pred<T>;
}
/// Subtracting negative numbers from negative numbers (e.g. -3 - -4)
impl<T: NonPos + SubPeano<RHS>, RHS: NonPos> SubPeano<Pred<RHS>> for Pred<T> {
    type Output = <T as SubPeano<RHS>>::Output;
}

pub trait MulPeano<RHS = Self> {
    type Output;
}

/// Multiplying zero by things (e.g. 0 * 7)
impl<RHS: Peano> MulPeano<RHS> for Zero {
    type Output = Zero;
}

/// Multiplying a positive integer by an arbitrary integer (e.g. 2 * N)
impl<T, RHS> MulPeano<RHS> for Succ<T>
    where T: NonNeg + MulPeano<RHS>, RHS: AddPeano<<T as MulPeano<RHS>>::Output> {
        type Output = <RHS as AddPeano<<T as MulPeano<RHS>>::Output>>::Output;
}

/// Multiplying a negative integer by an arbitrary integer (e.g. -2 * N)
impl<T, RHS> MulPeano<RHS> for Pred<T>
    where T: NonPos + MulPeano<RHS>, RHS: Peano, <T as MulPeano<RHS>>::Output: SubPeano<RHS> {
        type Output = <<T as MulPeano<RHS>>::Output as SubPeano<RHS>>::Output;
}


/// Note that, while we define division, we are operating in a ring, so an error
/// will be thrown unless the numerator is divisible by the denominator
pub trait DivPeano<RHS = Self> {
    type Output;
}
impl<RHS> DivPeano<RHS> for Zero
    where RHS: NonZero {
    type Output = Zero;
}
impl<T, RHS> DivPeano<RHS> for Succ<T>
    where Succ<T>: DivPeanoPriv<RHS> {
    type Output = <Succ<T> as DivPeanoPriv<RHS>>::Output;
}
impl<T, RHS> DivPeano<RHS> for Pred<T>
    where T: NonPos + Negate, RHS: Negate, Succ<<T as Negate>::Output>: DivPeanoPriv<<RHS as Negate>::Output>    {
    type Output = <<Pred<T> as Negate>::Output as DivPeanoPriv<<RHS as Negate>::Output>>::Output;
}

// DivPeanoPriv only supports positive numerators. That way, it will terminate with an
// error if you ever try to divide non-divisible things. We can divide things like -4 /
// 2 by first negating both numerator and denominator, which is what DivPeano does.
trait DivPeanoPriv<RHS = Self> {
    type Output;
}

impl<RHS: NonZero> DivPeanoPriv<RHS> for Zero {
    type Output = Zero;
}

//Dividing a positive integer by a positive integer (e.g. 4 / 2)
impl<T, D> DivPeanoPriv<Succ<D>> for Succ<T>
    where T: NonNeg, Succ<T>: DivPeanoPriv<Succ<D>> + SubPeano<Succ<D>>, D: NonNeg {
        type Output = <One as AddPeano<<<Succ<T> as SubPeano<Succ<D>>>::Output as DivPeanoPriv<Succ<D>>>::Output>>::Output;
    }

// Dividing a positive integer by a negative integer (e.g. 4 / -2)
impl<T, D> DivPeanoPriv<Pred<D>> for Succ<T>
    where T: NonNeg, Succ<T>: DivPeanoPriv<Pred<D>> + AddPeano<Pred<D>>, D: NonPos {
        type Output = <NegOne as AddPeano<<<Succ<T> as AddPeano<Pred<D>>>::Output as DivPeanoPriv<Pred<D>>>::Output>>::Output;
    }




pub trait KeepPeano<RHS = Self> {
    type Output;
}
impl<T> KeepPeano<T> for T where T: Peano {
    type Output = T;
}


pub trait ToInt {
    fn to_int() -> i32;
}
impl ToInt for Zero {
    fn to_int() -> i32 { 0 }
}
impl<T:NonNeg + ToInt> ToInt for Succ<T> {
    fn to_int() -> i32 { 1 + <T as ToInt>::to_int() }
}
impl<T:NonPos + ToInt> ToInt for Pred<T> {
    fn to_int() -> i32 { -1 + <T as ToInt>::to_int() }
}
