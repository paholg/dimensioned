use std::ops::{Add, Sub, Mul, Div};

struct Zero;
struct Succ<T: NonNeg>;
struct Pred<T: NonPos>;

trait Peano {}
trait NonZero: Peano {}
trait NonNeg: Peano {}
trait NonPos: Peano {}
trait Pos: Peano + NonZero + NonNeg {}
trait Neg: Peano + NonZero + NonPos {}

impl Peano for Zero {}
impl NonNeg for Zero {}
impl NonPos for Zero {}

impl<T: NonNeg> Peano for Succ<T> {}
impl<T: NonNeg> NonNeg for Succ<T> {}
impl<T: NonNeg> NonZero for Succ<T> {}
impl<T: NonNeg> Pos for Succ<T> {}

impl<T: NonPos> Peano for Pred<T> {}
impl<T: NonPos> NonPos for Pred<T> {}
impl<T: NonPos> NonZero for Pred<T> {}
impl<T: NonPos> Neg for Pred<T> {}

trait AddPeano<Rhs> {
    type Sum;
}

impl<Rhs: Peano> AddPeano<Rhs> for Zero {
    type Sum = Rhs;
}

// impl<Rhs, T> AddPeano<Rhs> for Succ<T>
//     where T: AddPeano<Rhs>, Rhs: NonNeg {
//         type Sum = Succ<<T as AddPeano<Rhs>>::Sum>;
//     }

// trait Natural: Peano {
//     fn sub_one<T>(N: Succ<T>) -> T where T: Peano {  }
// }



// impl Peano for Zero {
//     fn add_one(&self) -> Succ<Zero> { Succ::<Zero> }
// }

// impl<T: Peano> Peano for Succ<T> {
//     fn add_one(&self) -> Succ<Self> { Succ::<Self> }
// }

// impl<T: Peano> Natural for Succ<T> {
//     fn sub_one(&self) ->
// }


// impl<T: Peano> Add for Zero {
//     type Output = T;

//     fn add(self, _rhs: T) -> T {
//         _rhs
//     }
// }



// impl<Rhs: Peano> AddAss<Rhs> for Zero {
//     type Sum = Rhs;
//     fn add(&self, rhs: &Rhs) -> Sum { rhs }
// }

// type One = Succ<Zero>;
// type Two = Succ<One>;
// type Three = Succ<Two>;



//struct Two;

// impl<T: Peano> Add<Zero> for T {
//     fn add(&self, rhs: &Zero) -> T { self }
// }

// impl Add<Two, Two> for Zero {
//     fn add(&self, rhs: &Two) -> Two { Two }
// }

// impl Add<One, Two> for One {
//     fn add(&self, rhs: &One) -> Two { Two }
// }

// pub trait Length {
//     type pow;
// }

// pub struct Meters;

// impl Length for Meters {
//     type pow = 1;
// }

// #[test]
// fn add() {
//     Zero + One;
//     Zero + Two;
// }
