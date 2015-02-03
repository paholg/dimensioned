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

trait AddPeano<Rhs> {
    type Result;
}

/// Adding things to zero (e.g. 0 + 3)
impl<Rhs: Peano> AddPeano<Rhs> for Zero {
    type Result = Rhs;
}

/// Adding positive numbers (e.g. 1 + 2)
impl<T: NonNeg + AddPeano<Succ<Rhs>>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Succ<T> {
    type Result = Succ<<T as AddPeano<Succ<Rhs>>>::Result>;
}
/// Adding zero to positive numbers (e.g. 3 + 0)
impl<T: NonNeg> AddPeano<Zero> for Succ<T> {
    type Result = Succ<T>;
}
/// Adding negative numbers to positive numbers (e.g. 2 + -3)
impl<T: NonNeg + AddPeano<Rhs>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Succ<T> {
    type Result = <T as AddPeano<Rhs>>::Result;
}

/// Adding negative numbers (e.g. -1 + -2)
impl<T: NonPos + AddPeano<Pred<Rhs>>, Rhs: NonPos> AddPeano<Pred<Rhs>> for Pred<T> {
    type Result = Pred<<T as AddPeano<Pred<Rhs>>>::Result>;
}
/// Adding zero to negative numbers (e.g. -3 + 0)
impl<T: NonPos> AddPeano<Zero> for Pred<T> {
    type Result = Pred<T>;
}
/// Adding positive numbers to negative numbers (e.g. -2 + 3)
impl<T: NonPos + AddPeano<Rhs>, Rhs: NonNeg> AddPeano<Succ<Rhs>> for Pred<T> {
    type Result = <T as AddPeano<Rhs>>::Result;
}


fn print_type<T>() {
    let type_name = unsafe { (*get_tydesc::<T>()).name };
    println!("{}", type_name);
}

#[test]
fn count() {
    type One = Succ<Zero>;
    type Two = Succ<One>;
    type Three = Succ<Two>;

    type NegOne = Pred<Zero>;
    type NegTwo = Pred<NegOne>;
    type NegThree = Pred<NegTwo>;

    print!("      1 = ");
    print_type::<One>();

    print!("     -1 = ");
    print_type::<NegOne>();

    print!(" 0 +  2 = ");
    print_type::<<Zero as AddPeano<Two>>::Result>();
    print!(" 2 +  0 = ");
    print_type::<<Two as AddPeano<Zero>>::Result>();

    print!(" 1 +  2 = ");
    print_type::<<One as AddPeano<Two>>::Result>();
    print!(" 2 +  1 = ");
    print_type::<<Two as AddPeano<One>>::Result>();

    print!(" 1 + -1 = ");
    print_type::<<One as AddPeano<NegOne>>::Result>();
    print!(" 1 + -2 = ");
    print_type::<<One as AddPeano<NegTwo>>::Result>();
    print!(" 2 + -1 = ");
    print_type::<<Two as AddPeano<NegOne>>::Result>();

    print!("-2 +  0 = ");
    print_type::<<NegTwo as AddPeano<Zero>>::Result>();

    print!("-1 + -2 = ");
    print_type::<<NegOne as AddPeano<NegTwo>>::Result>();
    print!("-2 + -1 = ");
    print_type::<<NegTwo as AddPeano<NegOne>>::Result>();

    print!("-1 +  1 = ");
    print_type::<<NegOne as AddPeano<One>>::Result>();
    print!("-1 +  2 = ");
    print_type::<<NegOne as AddPeano<Two>>::Result>();
    print!("-2 +  1 = ");
    print_type::<<NegTwo as AddPeano<One>>::Result>();
}
