#![allow(unstable)]
#![allow(dead_code)]
use std::intrinsics::get_tydesc;

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

// trait AddPeano<Rhs> {
//     type Result;
// }

// impl<Rhs: Peano> AddPeano<Rhs> for Zero {
//     type Result = Rhs;
// }

// /// Adding two non-negative integers (e.g. 1 + 1)
// impl<Rhs: NonNeg, T: AddPeano<Rhs> + NonNeg> AddPeano<Rhs> for Succ<T> {
//     type Result = Succ<<T as AddPeano<Rhs>>::Result>;
// }

// /// Adding two non-positive integers (e.g. -1 + -1)
// impl<Rhs: NonPos, T: AddPeano<Rhs> + NonPos> AddPeano<Rhs> for Pred<T> {
//     type Result = Pred<<T as AddPeano<Rhs>>::Result>;
// }

trait AddOne {
    type Result;
}
impl AddOne for Zero {
    type Result = Succ<Zero>;
}
impl<T: NonNeg> AddOne for Succ<T> {
    type Result = Succ<Succ<T>>;
}
impl<T: NonPos> AddOne for Pred<T> {
    type Result = T;
}

trait SubOne {
    type Result;
}
impl SubOne for Zero {
    type Result = Pred<Zero>;
}
impl<T: NonNeg> SubOne for Succ<T> {
    type Result = T;
}
impl<T: NonPos> SubOne for Pred<T> {
    type Result = Pred<Pred<T>>;
}

trait AddPeano<Rhs> {
    type Result;
}

impl<Rhs: Peano> AddPeano<Rhs> for Zero {
    type Result = Rhs;
}

impl<Rhs, T> AddPeano<Rhs> for Succ<T>
    where T: AddPeano<<Rhs as AddOne>::Result> + NonNeg, Rhs: AddOne {
        // Result: T + Rhs as AddOne
        type Result = <T as AddPeano<<Rhs as AddOne>::Result>>::Result;
}

fn print_type<T>() {
    let type_name = unsafe { (*get_tydesc::<T>()).name };
    println!("{}", type_name);
}

#[test]
fn count() {
    type One = Succ<Zero>;
    print!("      1 = ");
    print_type::<One>();
    type NegOne = Pred<Zero>;
    print!("     -1 = ");
    print_type::<NegOne>();

    print!("  0 + 1 = ");
    print_type::<<Zero as AddOne>::Result>();
    print!("  1 + 1 = ");
    type Two = <One as AddOne>::Result;
    print_type::<Two>();
    print!(" -1 + 1 = ");
    print_type::<<NegOne as AddOne>::Result>();

    print!("  0 - 1 = ");
    print_type::<<Zero as SubOne>::Result>();
    print!("  1 - 1 = ");
    print_type::<<One as SubOne>::Result>();
    print!(" -1 - 1 = ");
    type NegTwo = <NegOne as SubOne>::Result;
    print_type::<NegTwo>();

    print!("1 + 2 = ");
    print_type::<<One as AddPeano<Two>>::Sum>();
}
