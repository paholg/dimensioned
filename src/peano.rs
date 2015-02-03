#![allow(dead_code)]
use std::intrinsics::get_tydesc;

struct Zero;
struct Succ<T: NonNeg>;
struct Pred<T: NonPos>;

type One = Succ<Zero>;

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

trait Negate {
    type Result;
}
impl Negate for Zero {
    type Result = Zero;
}
impl<T: NonNeg + Negate> Negate for Succ<T> {
    type Result = Pred<<T as Negate>::Result>;
}
impl<T: NonPos + Negate> Negate for Pred<T> {
    type Result = Succ<<T as Negate>::Result>;
}

trait SubPeano<Rhs> {
    type Result;
}

/// Subtracting from zero
impl<Rhs: Peano + Negate> SubPeano<Rhs> for Zero {
    type Result = <Rhs as Negate>::Result;
}

/// Subtracting positive numbers from positive numbers (e.g. 3 - 4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Succ<T> {
    type Result = <T as SubPeano<Rhs>>::Result;
}
/// Subtracting zero from positive numbers (e.g. 3 - 0)
impl<T: NonNeg> SubPeano<Zero> for Succ<T> {
    type Result = Succ<T>;
}
/// Subtracting negative numbers from positive numbers (e.g. 3 - -4)
impl<T: NonNeg + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Succ<T> {
    type Result = Succ<Succ<<T as SubPeano<Rhs>>::Result>>;
}

/// Subtracting positive numbers from negative numbers (e.g. -3 - 4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonNeg> SubPeano<Succ<Rhs>> for Pred<T> {
    type Result = Pred<Pred<<T as SubPeano<Rhs>>::Result>>;
}
/// Subtracting zero from negative numbers (e.g. -3 - 0)
impl<T: NonPos> SubPeano<Zero> for Pred<T> {
    type Result = Pred<T>;
}
/// Subtracting negative numbers from negative numbers (e.g. -3 - -4)
impl<T: NonPos + SubPeano<Rhs>, Rhs: NonPos> SubPeano<Pred<Rhs>> for Pred<T> {
    type Result = <T as SubPeano<Rhs>>::Result;
}

// trait MulPeano<Rhs> {
//     type Result;
// }

// /// Multiplying zero by things (e.g. 0 * 7)
// impl<Rhs: Peano> MulPeano<Rhs> for Zero {
//     type Result = Zero;
// }

// /// Multiplying non-negative integers (e.g. 1 * 8)
// impl<T, Rhs> MulPeano<Rhs> for Succ<T>
//     where T: NonNeg + MulPeano<Rhs>, Rhs: AddPeano<<T as MulPeano<Rhs>>::Result> {
//         type Result = <Rhs as AddPeano<<T as MulPeano<Rhs>>::Result>>::Result;
// }

// /// Multiplying non-positive integers (e.g. -1 * -8)
// impl<T, Rhs> MulPeano<Rhs> for Succ<T>
//     where T: NonPos + MulPeano<Rhs>, Rhs: AddPeano<<T as MulPeano<Rhs>>::Result> {
//         type Result = <Rhs as AddPeano<<T as MulPeano<Rhs>>::Result>>::Result;
// }



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

    println!("\nA couple numbers:");
    print!("      0 = ");
    print_type::<Zero>();
    print!("      1 = ");
    print_type::<One>();
    print!("      2 = ");
    print_type::<Two>();

    print!("     -1 = ");
    print_type::<NegOne>();
    print!("     -2 = ");
    print_type::<NegTwo>();

    println!("\nAddition:");

    print!(" 0 +  0 = ");
    print_type::<<Zero as AddPeano<Zero>>::Result>();
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

    println!("\nNegation:");
    print!("     -0 = ");
    print_type::<<Zero as Negate>::Result>();
    print!("     -2 = ");
    print_type::<<Two as Negate>::Result>();
    print!("    --2 = ");
    print_type::<<NegTwo as Negate>::Result>();

    println!("\nSubtraction:");
    print!(" 0 -  0 = ");
    print_type::<<Zero as SubPeano<Zero>>::Result>();
    print!(" 2 -  0 = ");
    print_type::<<Two as SubPeano<Zero>>::Result>();
    print!(" 0 -  2 = ");
    print_type::<<Zero as SubPeano<Two>>::Result>();
    print!(" 3 -  2 = ");
    print_type::<<Three as SubPeano<Two>>::Result>();
    print!(" 2 -  3 = ");
    print_type::<<Two as SubPeano<Three>>::Result>();
    print!(" 1 - -2 = ");
    print_type::<<One as SubPeano<NegTwo>>::Result>();

    print!("-3 - -2 = ");
    print_type::<<NegThree as SubPeano<NegTwo>>::Result>();
    print!("-2 - -3 = ");
    print_type::<<NegTwo as SubPeano<NegThree>>::Result>();
    print!("-1 -  2 = ");
    print_type::<<NegOne as SubPeano<Two>>::Result>();

    // println!("\nMultiplication:");
    // print!(" 3 *  0 = ");
    // print_type::<<Three as MulPeano<Zero>>::Result>();
    // print!(" 0 *  3 = ");
    // print_type::<<Zero as MulPeano<Three>>::Result>();
    // print!(" 2 *  2 = ");
    // print_type::<<Two as MulPeano<Two>>::Result>();

}
