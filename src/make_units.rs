/// Create a unit system!
///
/// This macro is the meat of the library.
///
/// fixme: Outline of normal vs. advanced use.
/// fixme: Note about formatting -- can disable it, but debug is always on
/// fixme: Discussion of operations
/// fixme: Example
///
/// As this macro makes some imports and creates a module named `inner`, it is highly recommended
/// that you place any call to it in its own module.
///
#[macro_export]
macro_rules! make_units {

    (@ops $System:ident, $Unitless:ident) => (
        // -------------------------------------------------------------------------------
        // Unary: Neg, Not

        macro_rules! unary_op {
            ($Trait:ident, $fun:ident) => (
                impl<V, A> $Trait for $System<V, A> where
                    V: $Trait,
                {
                    type Output = $System<<V as $Trait>::Output, A>;
                    fn $fun(self) -> Self::Output {
                        $System::new( $Trait::$fun(self.value) )
                    }
                }
            );
        }

        unary_op!(Not, not);
        unary_op!(Neg, neg);

        // -------------------------------------------------------------------------------
        // Unit-preserving: Add, Sub, BitAnd, BitOr, BitXor

        macro_rules! binary_unit_preserve {
            ($Trait:ident, $fun:ident, $TraitAssign:ident, $fun_assign:ident) => (
                // Both have units
                impl<Vl, A, Vr> $Trait<$System<Vr, A>> for $System<Vl, A> where
                    Vl: $Trait<Vr>,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
                    fn $fun(self, rhs: $System<Vr, A>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs.value) )
                    }
                }

                // Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
                    Vl: $Trait<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs) )
                    }
                }

                // Assign: Both have units
                impl<Vl, A, Vr> $TraitAssign<$System<Vr, A>> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>,
                {
                    fn $fun_assign(&mut self, rhs: $System<Vr, A>) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs.value)
                    }
                }

                // Assign: Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
                {
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs)
                    }
                }
            );
        }

        binary_unit_preserve!(Add, add, AddAssign, add_assign);
        binary_unit_preserve!(Sub, sub, SubAssign, sub_assign);
        binary_unit_preserve!(BitAnd, bitand, BitAndAssign, bitand_assign);
        binary_unit_preserve!(BitOr, bitor, BitOrAssign, bitor_assign);
        binary_unit_preserve!(BitXor, bitxor, BitXorAssign, bitxor_assign);

        // -------------------------------------------------------------------------------
        // Unit-changing: Mul, Div

        macro_rules! binary_unit_change {
            ($Trait:ident, $fun:ident, $op:ident, $TraitAssign:ident, $fun_assign:ident) => (
                // Both have units
                impl<Vl, Al, Vr, Ar> $Trait<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $Trait<Vr>, Al: $op<Ar>,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, <Al as $op<Ar>>::Output>;
                    fn $fun(self, rhs: $System<Vr, Ar>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs.value) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Al, Vr, Ar> $TraitAssign<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $TraitAssign<Vr>,
                    $System<Vr, Ar>: Dimensionless,
                {
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ar>) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs.value)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim,
                {
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs)
                    }
                }
            );
        }

        binary_unit_change!(Mul, mul, Add, MulAssign, mul_assign);
        binary_unit_change!(Div, div, Sub, DivAssign, div_assign);

        // -------------------------------------------------------------------------------
        // Rem (it's kinda its own thing)

        // Both have units
        impl<Vl, Al, Vr, Ar> Rem<$System<Vr, Ar>> for $System<Vl, Al> where
            Vl: Rem<Vr>
        {
            type Output = $System<<Vl as Rem<Vr>>::Output, Al>;
            fn rem(self, rhs: $System<Vr, Ar>) -> Self::Output {
                $System::new( self.value % rhs.value )
            }
        }

        // Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> Rem<Vr> for $System<Vl, A> where
            Vl: Rem<Vr>, Vr: NotDim,
        {
            type Output = $System<<Vl as Rem<Vr>>::Output, A>;
            fn rem(self, rhs: Vr) -> Self::Output {
                $System::new( self.value % rhs )
            }
        }

        // Assign, both have units
        impl<Vl, Al, Vr, Ar> RemAssign<$System<Vr, Ar>> for $System<Vl, Al> where
            Vl: RemAssign<Vr>,
        {
            fn rem_assign(&mut self, rhs: $System<Vr, Ar>) {
                self.value %= rhs.value
            }
        }

        // Assign: Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> RemAssign<Vr> for $System<Vl, A> where
            Vl: RemAssign<Vr>, Vr: NotDim,
        {
            fn rem_assign(&mut self, rhs: Vr) {
                self.value %= rhs
            }
        }

        // -------------------------------------------------------------------------------
        // Shl, Shr

        macro_rules! binary_shift {
            ($Trait:ident, $fun:ident, $TraitAssign:ident, $fun_assign:ident) => (
                // Lhs has units, rhs unitless
                impl<Vl, Al, Vr, Ar> $Trait<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $Trait<Vr>, $System<Vr, Ar>: Dimensionless
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, Al>;
                    fn $fun(self, rhs: $System<Vr, Ar>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs.value) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, Al, Vr> $Trait<Vr> for $System<Vl, Al> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, Al>;
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Al, Vr, Ar> $TraitAssign<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $TraitAssign<Vr>,
                    $System<Vr, Ar>: Dimensionless,
                {
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ar>) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs.value)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim,
                {
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value, rhs)
                    }
                }
            );
        }

        binary_shift!(Shl, shl, ShlAssign, shl_assign);
        binary_shift!(Shr, shr, ShrAssign, shr_assign);

        // -------------------------------------------------------------------------------
        // All things with primitives
        macro_rules! prim {
            ($t: ty) => (

                // Operations that require lhs and rhs to have the same units
                // Add, Sub, BitAnd, BitOr, BitXor
                macro_rules! same_units {
                    ($Trait:ident, $fun:ident, $TraitAssign:ident, $fun_assign:ident) => (

                        // Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $Trait<$t> for $System<V, A> where
                            V: $Trait<$t>, $System<V, A>: Dimensionless
                        {
                            type Output = $System<<V as $Trait<$t>>::Output, A>;
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value, rhs) )
                            }
                        }

                        // Primitive on lhs, unitless on rhs
                        impl<V, A> $Trait<$System<V, A>> for $t where
                            $t: $Trait<V>, $System<V, A>: Dimensionless
                        {
                            type Output = $System<<$t as $Trait<V>>::Output, A>;
                            fn $fun(self, rhs: $System<V, A>) -> Self::Output {
                                $System::new( $Trait::$fun(self, rhs.value) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $TraitAssign<$t> for $System<V, A> where
                            V: $TraitAssign<$t>, $System<V, A>: Dimensionless
                        {
                            fn $fun_assign(&mut self, rhs: $t) {
                                $TraitAssign::$fun_assign(&mut self.value, rhs)
                            }
                        }

                    );
                }

                same_units!(Add, add, AddAssign, add_assign);
                same_units!(Sub, sub, SubAssign, sub_assign);

                same_units!(BitAnd, bitand, BitAndAssign, bitand_assign);
                same_units!(BitOr, bitor, BitOrAssign, bitor_assign);
                same_units!(BitXor, bitxor, BitXorAssign, bitxor_assign);

                // Operations that don't care what units lhs has when rhs is a scalar
                // Mul, Div, Rem, Shl, Shr
                macro_rules! rhs_units_differ {
                    ($Trait:ident, $fun:ident, $TraitAssign:ident, $fun_assign:ident) => (
                        // Units on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $Trait<$t> for $System<V, A> where
                            V: $Trait<$t>
                        {
                            type Output = $System<<V as $Trait<$t>>::Output, A>;
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value, rhs) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $TraitAssign<$t> for $System<V, A> where
                            V: $TraitAssign<$t>
                        {
                            fn $fun_assign(&mut self, rhs: $t) {
                                $TraitAssign::$fun_assign(&mut self.value, rhs)
                            }
                        }

                    );
                }

                rhs_units_differ!(Mul, mul, MulAssign, mul_assign);
                rhs_units_differ!(Div, div, DivAssign, div_assign);
                rhs_units_differ!(Rem, rem, RemAssign, rem_assign);
                rhs_units_differ!(Shl, shl, ShlAssign, shl_assign);
                rhs_units_differ!(Shr, shr, ShrAssign, shr_assign);

                // Mul: Primitive on lhs, units on rhs
                impl<V, A> Mul<$System<V, A>> for $t where $t: Mul<V> {
                    type Output = $System<Prod<$t, V>, A>;
                    #[inline]
                    fn mul(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self*rhs.value)
                    }
                }

                // Div: Primitive on lhs, units on rhs
                impl<V, A> Div<$System<V, A>> for $t where $t: Div<V>, A: Neg {
                    type Output = $System<Quot<$t, V>, <A as Neg>::Output>;
                    #[inline]
                    fn div(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self / rhs.value)
                    }
                }

                // Rem: Primitive on lhs, units on rhs
                impl<V, A> Rem<$System<V, A>> for $t where $t: Rem<V> {
                    type Output = $Unitless<<$t as Rem<V>>::Output>;
                    #[inline]
                    fn rem(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self % rhs.value)
                    }
                }

            );
        }

        prim!(f32);
        prim!(f64);

        prim!(i8);
        prim!(i16);
        prim!(i32);
        prim!(i64);
        prim!(isize);

        prim!(u8);
        prim!(u16);
        prim!(u32);
        prim!(u64);
        prim!(usize);

        prim!(bool);
        prim!(char);
    );

    (@fmt true S $System:ident $(R $Root:ident P $print_as:expr;)* T $Trait:ident E $token:expr) => (
        impl<V, A> fmt::$Trait for $System<V, A> where
            V: fmt::$Trait,
        Length<A>: ArrayLength<isize>,
            A: TypeArray + Len + ToGA<Output = GenericArray<isize, Length<A>>>,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
            {
                use $crate::typenum::consts::*;
                use $crate::typenum::Integer;
                let allowed_roots = [$($Root::to_isize()),*];
                let exponents = A::to_ga();
                let print_tokens = [$($print_as),*];

                let mut first = true;

                try!(write!(f, $token, self.value));

                for ((&root, exp), token) in
                    allowed_roots.into_iter()
                    .zip(exponents.into_iter())
                    .zip(print_tokens.iter())
                {
                    if first {
                        first = false;
                        if exp != 0 {
                            try!(write!(f, " "));
                        }
                    } else if exp != 0 {
                        try!(write!(f, "*"));
                    }

                    match exp {
                        0 => (),
                        _ if exp == root => try!(write!(f, "{}", token)),
                        _ => {
                            if exp % root == 0 {
                                try!(write!(f, "{}^{}", token, exp/root))
                            } else {
                                try!(write!(f, "{}^{:.2}", token, exp as f32/root as f32))
                            }
                        },
                    }
                }
                Ok(())
            }
        }
    );

    (@fmt false S $System:ident $(R $Root:ident P $print_as:expr;)* T $Trait:ident E $token:expr) => ();

    // define arrays for all the base units
    (@base_arrays $Unitless:ident, $Unit:ident, $Root:ident, $($Units:ident, $Roots:ident,)*) => (
        pub type $Unitless = tarr![Z0, $(make_units!(@convert_to_zero $Units)),*];
        make_units!(@next_array U $Unit R $Root $(U $Units R $Roots)* $(E $Units)*);
    );

    (@next_array U $Unit:ident R $Root:ident $(U $Units:ident R $Roots:ident)*
     $(F $FrontZeros:ident)* E $Zero:ident $(E $EndZeros:ident)*) => (
        pub type $Unit = tarr![
            $(make_units!(@convert_to_zero $FrontZeros),)*
                $Root,
            Z0 $(, make_units!(@convert_to_zero $EndZeros))*
        ];
        make_units!(@next_array $(U $Units R $Roots)* $(F $FrontZeros)* F $Zero $(E $EndZeros)*);
    );

    (@next_array U $Unit:ident R $Root:ident $(F $FrontZeros:ident)*) => (
        pub type $Unit = tarr![
            $(make_units!(@convert_to_zero $FrontZeros),)*
                $Root
        ];
    );

    (@convert_to_zero $Unit:ident) => ( Z0 );
    (@convert_to_zero) => ();

    ($System:ident;
     $one:ident: $Unitless:ident;
     base {
         $($constant:ident: $Type:ident, $print_as:expr;)+
     }
     derived {
         $($derived_const:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        make_units!{
            $System;
            $one: $Unitless;
            base {
                $(P1, $constant: $Type, $print_as;)*
            }
            derived {
                $($derived_const: $Derived = ($($derived_rhs)+);)*
            }
            fmt = true;
        }

    );

    ($System:ident;
     $one:ident: $Unitless:ident;
     base {
         $($constant:ident: $Type:ident, $print_as:expr;)+
     }
     derived {
         $($derived_const:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     }
     fmt = $to_fmt:ident;
    ) => (
        make_units!{
            $System;
            $one: $Unitless;
            base {
                $(P1, $constant: $Type, $print_as;)*
            }
            derived {
                $($derived_const: $Derived = ($($derived_rhs)+);)*
            }
            fmt = $to_fmt;
        }

    );

    ($System:ident;
     $one:ident: $Unitless:ident;
     base {
         $($Root:ident, $constant:ident: $Unit:ident, $print_as:expr;)+
     }
     derived {
         $($derived_const:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     }
     fmt = $to_fmt:ident;
    ) => (
        use $crate::reexported::marker;

        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $System<V, A> {
            pub value: V,
            // fixme: this shouldn't be public once `new` can be const
            pub _marker: marker::PhantomData<A>,
        }

        impl<V, A> $System<V, A> {
            #[inline]
            #[allow(dead_code)]
            pub fn new(v: V) -> Self {
                $System { value: v, _marker: marker::PhantomData }
            }

            /// Perform an operation on the contained value.
            ///
            /// Use of this function is discouraged, as the operation may be one that does not
            /// perserve units, and this function has no way to protect against that.
            #[inline]
            #[allow(dead_code)]
            pub fn map_unsafe<O, F: FnOnce(V) -> O>(self, f: F) -> $System<O, A> {
                $System::new(f(self.value))
            }
        }

        impl<V, A> $System<V, A> where $System<V, A>: $crate::Dimensionless {
            /// Perform an operation on the contained value.
            ///
            /// This function is only defined for unitless types, so it is perfectly safe to use
            /// with operations that may not perserve units.
            pub fn map<O, F: FnOnce(V) -> O>(self, f: F) -> $System<O, A> {
                $System::new(f(self.value))
            }
        }

        use $crate::Dimensioned;
        impl<V, A> Dimensioned for $System<V, A> {}

        #[cfg(feature = "oibit")]
        use $crate::NotDim;
        #[cfg(feature = "oibit")]
        impl<V, A> !NotDim for $System<V, A> {}

        #[doc(hidden)]
        pub mod inner {
            #[allow(unused_imports)]
            use $crate::traits::*;
            #[allow(unused_imports)]
            use $crate::typenum::consts::*;
            make_units!(@base_arrays $Unitless, $($Unit, $Root,)*);
            $(pub type $Derived = unit!(@commas $($derived_rhs)+);)*
        }

        pub type $Unitless<__TypeParameter> = $System<__TypeParameter, inner::$Unitless>;
        $(pub type $Unit<__TypeParameter> = $System<__TypeParameter, inner::$Unit>;)*

            use $crate::Dimensionless;
        impl<__TypeParameter> $crate::Dimensionless for $Unitless<__TypeParameter> {}

        $(pub type $Derived<__TypeParameter> = $System<__TypeParameter, inner::$Derived>;)*

        // --------------------------------------------------------------------------------
        // Define consts
        pub mod f32consts {
            use super::*;
            use $crate::reexported::marker::PhantomData;
            pub const $one: $Unitless<f32> = $System { value: 1.0, _marker: PhantomData };
            $(pub const $constant: $Unit<f32> = $System { value: 1.0, _marker: PhantomData };)*
            $(pub const $derived_const: $Derived<f32> = $System { value: 1.0, _marker: PhantomData };)*
        }

        pub mod f64consts {
            use super::*;
            use $crate::reexported::marker::PhantomData;
            pub const $one: $Unitless<f64> = $System { value: 1.0, _marker: PhantomData };
            $(pub const $constant: $Unit<f64> = $System { value: 1.0, _marker: PhantomData };)*
            $(pub const $derived_const: $Derived<f64> = $System { value: 1.0, _marker: PhantomData };)*
        }

        // --------------------------------------------------------------------------------
        // Formatting
        use $crate::reexported::fmt;
        use $crate::typenum::{Len, Length, TypeArray};
        use $crate::generic_array::{GenericArray, ArrayLength};
        use $crate::array::ToGA;

        make_units!(@fmt true S $System $(R $Root P $print_as;)* T Debug E "{:?}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Display E "{}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Octal E "{:o}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T LowerHex E "{:x}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T UpperHex E "{:X}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Pointer E "{:p}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Binary E "{:b}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T LowerExp E "{:e}");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T UpperExp E "{:E}");

        // --------------------------------------------------------------------------------
        // Operator traits from this crate

        impl<V, A> $crate::Recip for $System<V, A> where V: $crate::Recip, A: $crate::reexported::ops::Neg, {
            type Output = $System<<V as $crate::Recip>::Output, $crate::typenum::Negate<A>>;
            fn recip(self) -> Self::Output { $System::new(self.value.recip()) }
        }

        use $crate::typenum::Pow;
        impl<Exp, V, A> Pow<Exp> for $System<V, A>
            where V: Pow<Exp>,
                  A: $crate::reexported::ops::Mul<Exp>,
        {
            type Output = $System< <V as Pow<Exp>>::Output, $crate::typenum::Prod<A, Exp>>;
            fn powi(self, exp: Exp) -> Self::Output {
                $System::new( self.value.powi(exp) )
            }
        }

        impl<Index, V, A> $crate::Root<Index> for $System<V, A>
            where V: $crate::Root<Index>,
                  A: $crate::reexported::ops::Div<Index>,
        {
            type Output = $System< <V as $crate::Root<Index>>::Output, $crate::typenum::Quot<A, Index>>;
            fn root(self, idx: Index) -> Self::Output {
                $System::new( self.value.root(idx) )
            }
        }

        // --------------------------------------------------------------------------------
        // Operators

        use $crate::reexported::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign,
                                      BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub,
                                      SubAssign, Rem, RemAssign, Neg, Not, Shl, ShlAssign, Shr,
                                      ShrAssign};
        use $crate::typenum::{Prod, Quot};

        make_units!(@ops $System, $Unitless);

        // --------------------------------------------------------------------------------
        // Deref only for dimensionless things

        use $crate::reexported::ops::Deref;
        impl<V, A> Deref for $System<V, A> where $System<V, A>: Dimensionless {
            type Target = V;
            fn deref(&self) -> &Self::Target {
                &self.value
            }
        }

        // --------------------------------------------------------------------------------
        // Index

        use $crate::reexported::ops::Index;
        impl<V, A, Idx> Index<Idx> for $System<V, A>
            where V: Index<Idx>,
                  <V as Index<Idx>>::Output: Sized,
                  $System<<V as Index<Idx>>::Output, A>: Sized
        {
            type Output = $System<<V as Index<Idx>>::Output, A>;
            fn index(&self, index: Idx) -> &Self::Output {
                // fixme: ensure this is safe
                unsafe {
                    $crate::reexported::mem::transmute(&self.value[index])
                }
            }
        }

        use $crate::reexported::ops::IndexMut;
        impl<V, A, Idx> IndexMut<Idx> for $System<V, A>
            where $System<V, A>: Index<Idx>,
                  V: Index<Idx> + IndexMut<Idx>,
        <V as Index<Idx>>::Output: Sized,
        <$System<V, A> as Index<Idx>>::Output: Sized
        {
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output{
                // fixme: ensure this is safe
                unsafe {
                    $crate::reexported::mem::transmute(&mut self.value[index])
                }
            }
        }

        // --------------------------------------------------------------------------------
    );
}

/// Creates a derived unit based on existing ones.
/// Currently only supports the operations * and /.
///
/// Note that in order to use this macro, you must be sure that the name of the unit system and all
/// items from its `inner` module are in scope. As a result of this, it should probably be called
/// in its own module.
///
/// The ideal way to create derived units is inside the `make_units!` macro, but this lets you
/// create derived units for systems that are defined elsewhere.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
/// use dim::si::{Meter, Second};
///
/// mod derived {
///     use dim::si::inner::*;
///     use dim::si::SI;
///     unit!(SI: MPS = Meter / Second);
/// }
/// use derived::MPS;
///
///
/// fn main() {
///    let x = Meter::new(5.0);
///    let t = Second::new(2.0);
///    let v = x / t;
///    let v2 = MPS::new(2.5);
///    assert_eq!(v, v2);
/// }
/// ```
#[macro_export]
macro_rules! unit {
    (@eval $a:ty,) => ($a);
    (@eval $a:ty, *, $b:ty, $($tail:tt)*) =>
        (unit!(@eval $crate::typenum::Sum<$a, $b>, $($tail)* ));
    (@eval $a:ty, /, $b:ty, $($tail:tt)*) =>
        (unit!(@eval $crate::typenum::Diff<$a, $b>, $($tail)* ));
    (@commas $t:ty) => ($t);
    (@commas $($tail:tt)*) => (unit!(@eval $($tail,)*));
    ($System:ident: $name:ident = $($tail:tt)*) => ( pub type $name<__TypeParameter> = $System<__TypeParameter, unit!(@commas $($tail)*)>;);
}
