
#[macro_export]
macro_rules! make_units { ($System:ident, $Unitless:ident, $one:ident; base { $($Type:ident, $constant:ident, $print_as:ident;)+ } derived {$($derived_constant:ident: $Derived:ident = $e:expr;)*} ) => (
    make_units_adv!{
        $System, $Unitless, $one, f64;
        base {
            $(One, $Type, $constant, $print_as;)*
        }
        derived {
            $($derived_constant: $Derived = $e;)*
        }
    }

    );
}

#[macro_export]
macro_rules! make_units_adv { ($System:ident, $Unitless:ident, $one:ident, $OneType:ident; base { $($Root:ident, $Type:ident, $constant:ident, $print_as:ident;)+ } derived {$($derived_constant:ident: $Derived:ident = $e: expr;    )*} ) => (
    #[allow(unused_imports)]
    use $crate::peano::{Zero, One, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten};
    use $crate::peano::{Peano, KeepPeano, AddPeano, SubPeano, MulPeano, DivPeano, Negate, ToInt};
    use $crate::dimensioned::{Dimension, Dimensionless, Dim, KeepDim, MulDim, DivDim, PowerDim, RootDim, InvertDim, DimToString};
    use std::marker::PhantomData;

    #[derive(Copy, Clone)]
    pub struct $System<$($Type: Peano = Zero),*> {
        $($constant: PhantomData<$Type>),*
    }
    impl<$($Type: Peano),*> Dimension for $System<$($Type),*> {}

    // using $Type and $constant for these traits is confusing. It should really be $Type_Left and
    // $Type_Right or something, but as far as I can tell, that is not supported by Rust
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> KeepDim<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + KeepPeano<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as KeepPeano<$constant>>::Output),*>;
        }
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> MulDim<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + AddPeano<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as AddPeano<$constant>>::Output),*>;
        }
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> DivDim<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + SubPeano<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as SubPeano<$constant>>::Output),*>;
        }
    impl<$($Type),*, RHS> PowerDim<RHS> for $System<$($Type),*> where
        $($Type: Peano + MulPeano<RHS>),*, RHS: Peano {
            type Output = $System<$(<$Type as MulPeano<RHS>>::Output),*>;
        }
    impl<$($Type),*, RHS> RootDim<RHS> for $System<$($Type),*> where
        $($Type: Peano + DivPeano<RHS>),*, RHS: Peano {
            type Output = $System<$(<$Type as DivPeano<RHS>>::Output),*>;
        }
    impl<$($Type),*> InvertDim for $System<$($Type),*> where
        $($Type: Peano + Negate),* {
            type Output = $System<$(<$Type as Negate>::Output),*>;
        }

    impl<$($Type),*> DimToString for $System<$($Type),*>
        where $($Type: ToInt),* {
            fn to_string() -> String {
                // fimxe: make this betterer
                let mut _string = String::new();
                $(
                    let temp = match <$Type as ToInt>::to_int() {
                        0 => ("", "".to_string()),
                        1 => (stringify!($print_as), "*".to_string()),
                        _power => (stringify!($print_as), format!("^{}*", _power/<$Root as ToInt>::to_int()))
                    };
                    _string = format!("{}{}{}", _string, temp.0, temp.1)
                );*;
                _string.pop(); // get rid of the last '*'
                _string
            }
        }

    pub type $Unitless = $System;
    impl Dimensionless for $Unitless {}
    #[allow(non_upper_case_globals)]
    pub const $one: Dim<$Unitless, $OneType> = Dim(1.0, PhantomData);

    __make_types!($System, $($Type, $Root),+ |);

    $(#[allow(non_upper_case_globals)] pub const $constant: Dim<$Type, $OneType> = Dim(1.0, PhantomData));*;

    // $(#[allow(non_upper_case_globals)] pub const $derived_constant: Dim<TYPEPEPEPE, f64> = $e;)*


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
//     ($a:ident * $b: expr) => ($a as MulDim<__convert_expression!($b)>>::Output);
//     ($a:ident / $b: expr) => ($a as DivDim<__convert_expression!($b)>>::Output);
//     ($a:ident) => ($a);
// }

