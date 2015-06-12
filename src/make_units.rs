
#[macro_export]
macro_rules! make_units { ($System:ident, $Unitless:ident, $one:ident; base { $($Type:ident, $constant:ident, $print_as:ident;)+ } derived {$($derived_constant:ident: $Derived:ident = $e:expr;)*} ) => (
    make_units_adv!{
        $System, $Unitless, $one, f64, 1.0;
        base {
            $(P1, $Type, $constant, $print_as;)*
        }
        derived {
            $($derived_constant: $Derived = $e;)*
        }
    }

    );
}

#[macro_export]
macro_rules! make_units_adv { ($System:ident, $Unitless:ident, $one:ident, $OneType:ident, $val:expr; base { $($Root:ident, $Type:ident, $constant:ident, $print_as:ident;)+ } derived {$($derived_constant:ident: $Derived:ident = $e: expr;    )*} ) => (
    #[allow(unused_imports)]
    use $crate::dimensioned::{Zero, P1, P2, P3, P4, P5, P6, P7, P8, P9, N1, N2, N3, N4, N5, N6, N7, N8, N9};
    use $crate::peano::{Peano, Same, ToInt};
    use $crate::dimensioned::{Dimension, Dimensionless, Dim, Pow, Root, Recip, DimToString};
    use ::std::ops::{Add, Neg, Sub, Mul, Div};
    use std::marker::PhantomData;

    #[derive(Copy, Clone)]
    pub struct $System<$($Type: Peano = Zero),*> {
        $($constant: PhantomData<$Type>),*
    }
    impl<$($Type: Peano),*> Dimension for $System<$($Type),*> {}

    // using $Type and $constant for these traits is confusing. It should really be $Type_Left and
    // $Type_Right or something, but as far as I can tell, that is not supported by Rust
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> Same<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + Same<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as Same<$constant>>::Output),*>;
        }
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> Mul<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + Add<$constant>),*, $($constant: Peano),*, $(<$Type as Add<$constant>>::Output: Peano),* {
            type Output = $System<$(<$Type as Add<$constant>>::Output),*>;
            #[allow(unused_variables)]
            fn mul(self, rhs: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> Div<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + Sub<$constant>),*, $($constant: Peano),*, $(<$Type as Sub<$constant>>::Output: Peano),* {
            type Output = $System<$(<$Type as Sub<$constant>>::Output),*>;
            #[allow(unused_variables)]
            fn div(self, rhs: $System<$($constant),*>) -> Self::Output { unreachable!()  }
        }
    impl<$($Type),*, RHS> Pow<RHS> for $System<$($Type),*> where
        $($Type: Peano + Mul<RHS>),*, RHS: Peano, $(<$Type as Mul<RHS>>::Output: Peano),* {
            type Output = $System<$(<$Type as Mul<RHS>>::Output),*>;
            #[allow(unused_variables)]
            fn pow(rhs: RHS) -> Self::Output { unreachable!() }
        }
    impl<$($Type),*, RHS> Root<RHS> for $System<$($Type),*> where
        $($Type: Peano + Div<RHS>),*, RHS: Peano, $(<$Type as Div<RHS>>::Output: Peano),* {
            type Output = $System<$(<$Type as Div<RHS>>::Output),*>;
            #[allow(unused_variables)]
            fn root(radicand: RHS) -> Self::Output { unreachable!() }
        }
    impl<$($Type),*> Recip for $System<$($Type),*> where
        $($Type: Peano + Neg),*, $(<$Type as Neg>::Output: Peano),* {
            type Output = $System<$(<$Type as Neg>::Output),*>;
            fn recip(self) -> Self::Output { unreachable!() }
        }


    fn pretty_dim(roots: [i32; count_args!($($Type),*)], exps: [i32; count_args!($($Type),*)], tokens: [&'static str; count_args!($($Type),*)]) -> String {
        let mut __string = String::new();
        for ((&root, &exp), &token) in roots.iter().zip(exps.iter()).zip(tokens.iter()) {
            let __temp: (&'static str, String) = match exp {
                0 => ("", "".to_string()),
                1 => (token, "*".to_string()),
                _ => (token, format!("^{}*", exp/root)),
            };
            __string = format!("{}{}{}", __string, __temp.0, __temp.1);
        }
        __string.pop(); // remove last "*"
        __string
    }


    impl<$($Type),*> DimToString for $System<$($Type),*>
        where $($Type: ToInt),* {
            fn to_string() -> String {
                // fixme: add #[allow(unused_variables)] lints for these. Not working
                // for me for some reason.
                let allowed_roots = [$($Root::to_int()),*];
                let exponents = [$($Type::to_int()),*];
                let print_tokens = [$(stringify!($print_as)),*];

                pretty_dim(allowed_roots, exponents, print_tokens)
            }
        }

    pub type $Unitless = $System;
    impl Dimensionless for $Unitless {}
    #[allow(non_upper_case_globals)]
    pub const $one: Dim<$Unitless, $OneType> = Dim($val, PhantomData);

    __make_types!($System, $($Type, $Root),+ |);

    $(#[allow(non_upper_case_globals)] pub const $constant: Dim<$Type, $OneType> = Dim($val, PhantomData));*;

    // $(#[allow(non_upper_case_globals)] pub const $derived_constant: Dim<TYPEPEPEPE, f64> = $e;)*


    );
}

/// Counts the number of arguments its called with and gives you the total
#[macro_export]
macro_rules! count_args {
    ($arg:ident, $($args:ident),+) => (
        1 + count_args!($($args),+);
    );
    ($arg:ident) => (
        1
    );
}

#[doc_hidden]
#[macro_export]
macro_rules! __make_types {
    ($System:ident, $Type:ident, $Root:ident, $($Types:ident, $Roots:ident),+ | $($Zeros:ident),*) => (
        pub type $Type = $System< $($Zeros,)* $Root>;
        __make_types!($System, $($Types, $Roots),+ | Zero $(, $Zeros)*);
        );
    ($System:ident, $Type:ident, $Root:ident | $($Zeros:ident),*) => (
        pub type $Type = $System<$($Zeros,)* $Root>;
        );
}

// #[macro_export]
// macro_rules! __make_derived_type {
//     ($D:ident, $e: expr) => (
//         pub type $D = __convert_expression!($e);
//         );
// }

// #[macro_export]
// macro_rules! __convert_expression {
//     ($a:ident * $b: expr) => ($a as Mul<__convert_expression!($b)>>::Output);
//     ($a:ident / $b: expr) => ($a as Div<__convert_expression!($b)>>::Output);
//     ($a:ident) => ($a);
// }

