/**
Create a unit system.

As this macro performs various imports, it is strongly recommended that you call it
inside of its own module.

Note that it has some imports from the peano crate, so it must be included.

# Example
```rust
#[macro_use]
extern crate dimensioned;

mod fruit {
    make_units! {
        Fruit, Unitless, one;
        base {
            Apple, apple, a;
            Banana, banana, b;
            Cucumber, cuke, c;
            Mango, mango, m;
            Watermelon, watermelon, w;
        }
        derived {
        }
    }
}
use fruit::{apple, banana, cuke, mango, watermelon};

fn main() {
    let fruit_salad = apple * banana * mango * mango * watermelon;
    println!("Mmmm, delicious: {}", fruit_salad);
    assert_eq!(format!("{}", fruit_salad), "1 a*b*m^2*w");
}
```

The line `Fruit, Unitless, one;` names the unit system `Fruit`, names its type for
unitless data `Unitless` and creates the corresponding constant `one`.

The `base` block is used to define the base units of this system. The line `Apple,
apple, a;` creates the unit `Apple`, the corresponding constant `apple`, and will use
the token "a" to print `Apple`s.

The `derived` block is not yet implemented, but will be used to define derived units and
constants.
*/
#[macro_export]
macro_rules! make_units {
    ($System:ident, $Unitless:ident, $one:ident;
     base {
         $($Type:ident, $constant:ident, $print_as:ident;)+
     }
     derived {
         $($derived_constant:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        make_units_adv!{
            $System, $Unitless, $one, f64, 1.0;
            base {
                $(P1, $Type, $constant, $print_as;)*
            }
            derived {
                $($derived_constant: $Derived = ($($derived_rhs)+);)*
            }
        }

        );
}
/**
Create a unit system with more flexibility than `make_units!()`.

As this macro performs various imports, it is strongly recommended that you call it
inside of its own module.

# Example

Here we define the **CGS** unit system.

```rust
#[macro_use]
extern crate dimensioned;

mod cgs {
    make_units_adv! {
        CGS, Unitless, one, f64, 1.0;
        base {
            P2, Centimeter, cm, cm;
            P2, Gram, g, g;
            P1, Second, s, s;
        }
        derived {
        }
    }
}

# fn main() {
# }
```

The line `CGS, Unitless, one, f64, 1.0;` names the unit system `CGS`, names its type for
unitless data `Unitless` and creates the corresponding constant `one`. It also states
that all constants will be of type `Quantity<D, f64>` and will be initialized to a value of
`1.0`.

Once associated constants hit, `std::num::One` will be used to determine the initalize value.

The `base` block is used to define the base units of this system. The line `P2,
Centimeter, cm, cm;` creates the unit `Centimeter`, the corresponding constant `cm`, and
will use the token "cm" to print `Centimeter`s. It also states that square roots will be
allowed for `Centimeter`s; the `P2` is the type number for 2 and dictates the highest
root allowed. You will almost always want this to be `P1`. For `CGS`, though, some
derived units are defined in terms of square roots of base units, so they are necessary
to allow.

The `derived` block is not yet implemented, but will be used to define derived units and
constants.

*/
#[macro_export]
macro_rules! make_units_adv {
    ($System:ident, $Unitless:ident, $one:ident, $OneType:ident, $val:expr;
     base {
         $($Root:ident, $Type:ident, $constant:ident, $print_as:ident;)+
     }
     derived {
         $($derived_constant:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        #[allow(unused_imports)]
        use $crate::{Z0, P1, P2, P3, P4, P5, P6, P7, P8, P9, N1, N2, N3, N4, N5, N6, N7, N8, N9};
        use $crate::Integer;
        use $crate::{Dimension, Dimensionless, Quantity, Pow, Root, Recip, QuantityToString};
        use ::std::ops::{Add, Neg, Sub, Mul, Div};
        use ::std::marker::PhantomData;

        #[derive(Copy, Clone)]
        pub struct $System<$($Type: Integer = Z0),*> {
            $($constant: PhantomData<$Type>),*
        }
        impl<$($Type: Integer),*> Dimension for $System<$($Type),*> {}

        // using $Type and $constant for these traits is confusing. It should really be $Type_Left and
        // $Type_Right or something, but that is not yet supported by Rust
        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($constant),*> Mul<$System<$($constant),*>> for $System<$($Type),*>
            where $($Type: Integer + Add<$constant>),*,
        $($constant: Integer),*,
        $(<$Type as Add<$constant>>::Output: Integer),*,
        $System<$(<$Type as Add<$constant>>::Output),*>: Dimension,
        {
            type Output = $System<$(<$Type as Add<$constant>>::Output),*>;
            #[allow(unused_variables)]
            fn mul(self, rhs: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }
        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($constant),*> Div<$System<$($constant),*>> for $System<$($Type),*> where
            $($Type: Integer + Sub<$constant>),*, $($constant: Integer),*, $(<$Type as Sub<$constant>>::Output: Integer),* {
            type Output = $System<$(<$Type as Sub<$constant>>::Output),*>;
            #[allow(unused_variables)]
            fn div(self, rhs: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }
        impl<$($Type),*, RHS> Pow<RHS> for $System<$($Type),*> where
            $($Type: Integer + Mul<RHS>),*, RHS: Integer, $(<$Type as Mul<RHS>>::Output: Integer),* {
            type Output = $System<$(<$Type as Mul<RHS>>::Output),*>;
            #[allow(unused_variables)]
            fn pow(rhs: RHS) -> Self::Output { unreachable!() }
        }
        impl<$($Type),*, RHS> Root<RHS> for $System<$($Type),*> where
            $($Type: Integer + Div<RHS>),*, RHS: Integer, $(<$Type as Div<RHS>>::Output: Integer),* {
            type Output = $System<$(<$Type as Div<RHS>>::Output),*>;
            #[allow(unused_variables)]
            fn root(radicand: RHS) -> Self::Output { unreachable!() }
        }
        impl<$($Type),*> Recip for $System<$($Type),*> where
            $($Type: Integer + Neg),*, $(<$Type as Neg>::Output: Integer),* {
            type Output = $System<$(<$Type as Neg>::Output),*>;
            fn recip(self) -> Self::Output { unreachable!() }
        }


        fn pretty_dim(roots: [i32; count_args!($($Type),*)], exps: [i32; count_args!($($Type),*)], tokens: [&'static str; count_args!($($Type),*)]) -> String {
            let mut __string = String::new();
            for ((&root, &exp), &token) in roots.iter().zip(exps.iter()).zip(tokens.iter()) {
                let __temp: (&'static str, String) = match exp {
                    0 => ("", "".to_owned()),
                    _ if exp == root => (token, "*".to_owned()),
                    _ => {
                        if exp % root == 0 {
                            (token, format!("^{}*", exp/root))
                        } else {
                            (token, format!("^{:.2}*", exp as f32/root as f32))
                        }
                    },
                };
                __string = format!("{}{}{}", __string, __temp.0, __temp.1);
            }
            __string.pop(); // remove the last "*"
            __string
        }


        impl<$($Type),*> QuantityToString for $System<$($Type),*>
            where $($Type: Integer),* {
            fn to_string() -> String {
                // fixme: add #[allow(unused_variables)] lints for these. Not working
                // for me for some reason.
                let allowed_roots = [$($Root::to_i32()),*];
                let exponents = [$($Type::to_i32()),*];
                let print_tokens = [$(stringify!($print_as)),*];

                pretty_dim(allowed_roots, exponents, print_tokens)
            }
        }

        pub type $Unitless = $System;
        impl Dimensionless for $Unitless {}
        #[allow(non_upper_case_globals, dead_code)]
        pub const $one: Quantity<$Unitless, $OneType> = Quantity($val, PhantomData);

        __make_base_types!($System, $($Type, $Root),+ |);

        $(#[allow(non_upper_case_globals)] pub const $constant: Quantity<$Type, $OneType> = Quantity($val, PhantomData));*;

        $(pub type $Derived = unit!($($derived_rhs)+);
          #[allow(non_upper_case_globals)]
          pub const $derived_constant: Quantity<$Derived, $OneType> = Quantity($val, PhantomData);
        )*
        );
}

/** Counts the number of arguments its called with and gives you the total.

#Example

```rust
#[macro_use]
extern crate dimensioned as dim;

fn main() {
    let x = count_args!(a, b, cat, banana);
    assert_eq!(4, x);
}
```
*/
#[macro_export]
macro_rules! count_args {
    ($arg:ident, $($args:ident),+) => (
        1 + count_args!($($args),+);
    );
    ($arg:ident) => (
        1
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_base_types {
    ($System:ident, $Type:ident, $Root:ident, $($Types:ident, $Roots:ident),+ | $($Zeros:ident),*) => (
        pub type $Type = $System< $($Zeros,)* $Root>;
        __make_base_types!($System, $($Types, $Roots),+ | Z0 $(, $Zeros)*);
        );
    ($System:ident, $Type:ident, $Root:ident | $($Zeros:ident),*) => (
        pub type $Type = $System<$($Zeros,)* $Root>;
        );
}

/// Creates a derived unit based on existing ones.
/// Currently only supports the operations * and /.
///
/// The ideal way to create derived units is inside the make_units! macro (which calls this one),
/// but this also lets you create derived units for systems that are already defined.
///
/// # Example
/// ```rust
/// #![feature(type_macros)]
/// #[macro_use]
/// extern crate dimensioned as dim;
/// use std::ops::Div;
/// use dim::{Quantity};
/// use dim::si::*;
/// type MPS = unit!(Meter / Second);
///
/// fn speed(dist: Quantity<Meter, f64>, time: Quantity<Second, f64>) -> Quantity<MPS, f64> {
///     dist / time
/// }
///
/// fn main() {
///     let x = 5.0 * m;
///     let t = 1.0 * s;
///     let v = speed(x, t);
///     assert_eq!(v, x/t);
/// }
/// ```
#[macro_export]
macro_rules! unit {
    // { ( $($LHS:tt)+ ) } => { unit!($($LHS)+) };
    { $LHS:tt * $($RHS:tt)+ } => { <unit!($LHS) as Mul<unit!($($RHS)+)>>::Output };
    { $LHS:tt / $($RHS:tt)+ } => { <unit!($LHS) as Div<unit!($($RHS)+)>>::Output };
    { $LHS:ty } => { $LHS };
}
