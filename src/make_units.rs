
#[macro_export]
macro_rules! make_units { ($System:ident, $allowed_root:ident; base { $($Type:ident, $constant:ident, $print_as:ident;)* } derived {$($Derived:ident($derived_constant: ident) = $e: expr;    )*} ) => (
    #[derive(Copy, Clone)]
    pub struct $System<$($Type: Peano),*> {
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
    impl<$($Type),*, $($constant),*> AddDim<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + AddPeano<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as AddPeano<$constant>>::Output),*>;
        }
    #[allow(non_camel_case_types)]
    impl<$($Type),*, $($constant),*> SubDim<$System<$($constant),*>> for $System<$($Type),*> where
        $($Type: Peano + SubPeano<$constant>),*, $($constant: Peano),* {
            type Output = $System<$(<$Type as SubPeano<$constant>>::Output),*>;
        }
    impl<$($Type),*, RHS> MulDim<RHS> for $System<$($Type),*> where
        $($Type: Peano + MulPeano<RHS>),*, RHS: Peano {
            type Output = $System<$(<$Type as MulPeano<RHS>>::Output),*>;
        }
    impl<$($Type),*, RHS> DivDim<RHS> for $System<$($Type),*> where
        $($Type: Peano + DivPeano<RHS>),*, RHS: Peano {
            type Output = $System<$(<$Type as DivPeano<RHS>>::Output),*>;
        }
    impl<$($Type),*> NegDim for $System<$($Type),*> where
        $($Type: Peano + Negate),* {
            type Output = $System<$(<$Type as Negate>::Output),*>;
        }

    impl<$($Type),*> DimToString for $System<$($Type),*>
        where $($Type: ToInt),* {
            fn to_string() -> String {
                let mut _string = String::new();
                $(
                    let temp = match <$Type as ToInt>::to_int() {
                        0 => ("", "".to_string()),
                        1 => (stringify!($print_as), "*".to_string()),
                        _power => (stringify!($print_as), format!("^{}*", _power/<$allowed_root as ToInt>::to_int()))
                    };
                    _string = format!("{}{}{}", _string, temp.0, temp.1);
                )*
                _string.pop();
                _string
            }
        }

    __make_types!($System, $allowed_root | $($Type),* | $($Type),* |);
    // fixme: when __make_types is fixed, use following line to call:
    // __make_types!($System, $allowed_root | $($Type),* | $($Type),*);

    #[allow(non_upper_case_globals)]
    pub const one: Dim<Unitless, f64> = Dim(1.0, PhantomData);
    $(#[allow(non_upper_case_globals)] pub const $constant: Dim<$Type, f64> = Dim(1.0, PhantomData);)*

        // $(
        //   __make_derived_type!($Derived, $e);
        //   pub const $derived_constant: Dim<$Derived, f64> = Dim(1.0, PhantomData);
        //   )*
        );
}

// fixme: This is a bunch of malarky. Can't cycle the direction I want, so we reverse
// $Typse to compensate. The "correct" way is commented below, but won't work until RFC:
// https://github.com/rust-lang/rust/issues/24827 is fixed
//#[doc_hidden] fixme: uncomment when more mature
#[macro_export]
macro_rules! __make_types {
    // this first arm filters out the end Types into Zeros with Num, so we go from say
    // (One | Meters, Seconds | Meters, Seconds) to (One, Zero | Meters, Seconds |
    // Meters) and then we move to a different branch I would like a cleaner way to
    // create the list $Num, Zero, Zero, ... but I have yet to find one.
    ($System: ident, $($Nums: ident),+ | $($Types: ident),+ | $DeadType: ident, $($Others: ident),* | $($New: ident),*) => (
        __make_types!($System, $($Nums),+, Zero | $($Types),+ | $($Others),* | $DeadType $(, $New)*);
        );
    // This branch creates our Unitless type and then trims off the last $Type that we no longer need:
    ($System: ident, $First: ident, $($Nums: ident),* | $($Types: ident),+ | $Type: ident | $($New: ident),+) => (
        pub type Unitless = $System<Zero, $($Nums),*>;
        impl Dimensionless for Unitless {}
        __make_types!($System, $($Nums),*, $First | $Type, $($New),* );
        );
    // Create the type, cycle the numbers, then call again:
    ($System: ident, $First: ident, $($Nums: ident),* | $Type: ident, $($Types: ident),+) => (
        pub type $Type = $System<$First, $($Nums),*>;
        __make_types!($System, $($Nums),*, $First | $($Types),*);
        );
    ($System: ident, $($Nums: ident),* | $Type: ident) => (
        pub type $Type = $System<$($Nums),*>;
        );
}
// end of malarky

// // we call this with __make_types!(Num | Types | Types)
// #[doc_hidden]
// #[macro_export]
// macro_rules! __make_types {
//     // this first arm filters out the end Types into Zeros with Num, so we go from say
//     // (One | Meters, Seconds | Meters, Seconds) to (One, Zero | Meters, Seconds |
//     // Meters) and then we move to a different branch I would like a cleaner way to
//     // create the list $Num, Zero, Zero, ... but I have yet to find one.
//     ($System: ident, $($Nums: ident),+ | $($Types: ident),+ | $DeadType: ident, $($Others: ident),+) => (
//         __make_types!($System, $($Nums),*, Zero | $($Types),* | $($Others),*);
//         );
//     // This branch creates our Unitless type and then trims off the last $Type that we no longer need:
//     ($System: ident, $First: ident, $($Nums: ident),* | $($Types: ident),+ | $Type: ident) => (
//         pub type Unitless = $System<Zero, $($Nums),*>;
//         impl Dimensionless for Unitless {}
//         __make_types!($System, $First, $($Nums),* | $($Types),* );
//         );
//     // Create the type, cycle the numbers, then call again:
//     ($System: ident, $($Nums: ident),*, $Last: ident | $Type: ident, $($Types: ident),+) => (
//         pub type $Type = $System<$($Nums),*, $Last>;
//         __make_types!($System, $Last, $($Nums),* | $($Types),*);
//         );
//     ($System: ident, $($Nums: ident),* | $Type: ident) => (
//         pub type $Type = $System<$($Nums),*>;
//         );
// }


// #[macro_export]
// macro_rules! __make_derived_type {
//     ($D: ident, $e: expr) => (
//         pub type $D = __convert_expression!($e);
//         );
// }

// #[macro_export]
// macro_rules! __convert_expression {
//     ($a: ident * $b: expr) => ($a as AddDim<__convert_expression!($b)>>::Output);
//     ($a: ident / $b: expr) => ($a as SubDim<__convert_expression!($b)>>::Output);
//     ($a: ident) => ($a);
// }
