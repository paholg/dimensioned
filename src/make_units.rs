/// Create a new unit system.
///
/// This macro is the heart of this library and is used to create the unit systems with which it
/// ships.
///
///
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
                    #[inline]
                    fn $fun(self) -> Self::Output {
                        $System::new($Trait::$fun(self.value_unsafe))
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
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, A>) -> Self::Output {
                        $System::new($Trait::$fun(self.value_unsafe, rhs.value_unsafe))
                    }
                }

                // Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
                    Vl: $Trait<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new($Trait::$fun(self.value_unsafe, rhs))
                    }
                }

                // Assign: Both have units
                impl<Vl, A, Vr> $TraitAssign<$System<Vr, A>> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, A>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs)
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
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, Ar>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs.value_unsafe) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Al, Vr, Ar> $TraitAssign<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $TraitAssign<Vr>,
                    $System<Vr, Ar>: Dimensionless,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ar>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs)
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
            #[inline]
            fn rem(self, rhs: $System<Vr, Ar>) -> Self::Output {
                $System::new( self.value_unsafe % rhs.value_unsafe )
            }
        }

        // Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> Rem<Vr> for $System<Vl, A> where
            Vl: Rem<Vr>, Vr: NotDim,
        {
            type Output = $System<<Vl as Rem<Vr>>::Output, A>;
            #[inline]
            fn rem(self, rhs: Vr) -> Self::Output {
                $System::new( self.value_unsafe % rhs )
            }
        }

        // Assign, both have units
        impl<Vl, Al, Vr, Ar> RemAssign<$System<Vr, Ar>> for $System<Vl, Al> where
            Vl: RemAssign<Vr>,
        {
            #[inline]
            fn rem_assign(&mut self, rhs: $System<Vr, Ar>) {
                self.value_unsafe %= rhs.value_unsafe
            }
        }

        // Assign: Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> RemAssign<Vr> for $System<Vl, A> where
            Vl: RemAssign<Vr>, Vr: NotDim,
        {
            #[inline]
            fn rem_assign(&mut self, rhs: Vr) {
                self.value_unsafe %= rhs
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
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, Ar>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs.value_unsafe) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, Al, Vr> $Trait<Vr> for $System<Vl, Al> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, Al>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Al, Vr, Ar> $TraitAssign<$System<Vr, Ar>> for $System<Vl, Al> where
                    Vl: $TraitAssign<Vr>,
                    $System<Vr, Ar>: Dimensionless,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ar>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, A, Vr> $TraitAssign<Vr> for $System<Vl, A> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: Vr) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs)
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
                            #[inline]
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                            }
                        }

                        // Primitive on lhs, unitless on rhs
                        impl<V, A> $Trait<$System<V, A>> for $t where
                            $t: $Trait<V>, $System<V, A>: Dimensionless
                        {
                            type Output = $System<<$t as $Trait<V>>::Output, A>;
                            #[inline]
                            fn $fun(self, rhs: $System<V, A>) -> Self::Output {
                                $System::new( $Trait::$fun(self, rhs.value_unsafe) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $TraitAssign<$t> for $System<V, A> where
                            V: $TraitAssign<$t>, $System<V, A>: Dimensionless
                        {
                            #[inline]
                            fn $fun_assign(&mut self, rhs: $t) {
                                $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs)
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
                            #[inline]
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, A> $TraitAssign<$t> for $System<V, A> where
                            V: $TraitAssign<$t>
                        {
                            #[inline]
                            fn $fun_assign(&mut self, rhs: $t) {
                                $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs)
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
                        $System::new(self * rhs.value_unsafe)
                    }
                }

                // Div: Primitive on lhs, units on rhs
                impl<V, A> Div<$System<V, A>> for $t where $t: Div<V>, A: Neg {
                    type Output = $System<Quot<$t, V>, <A as Neg>::Output>;
                    #[inline]
                    fn div(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self / rhs.value_unsafe)
                    }
                }

                // Rem: Primitive on lhs, units on rhs
                impl<V, A> Rem<$System<V, A>> for $t where $t: Rem<V> {
                    type Output = $Unitless<<$t as Rem<V>>::Output>;
                    #[inline]
                    fn rem(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self % rhs.value_unsafe)
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

                try!(self.value_unsafe.fmt(f));

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
         $($Root:ident, $base:ident: $Unit:ident, $print_as:expr;)+
     }
     derived {
         $($derived_const:ident: $Derived:ident = ($($derived_rhs:tt)+);)*
     }
     constants {
         $($constant:ident: $ConstantUnit:ident = $constant_value:expr;)*
     }
     fmt = $to_fmt:ident;
    ) => (
        use $crate::reexported::marker;
        use $crate::{Dimensioned, Dimensionless};

        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $System<V, A> {
            /// This is the value of whatever type we're giving units. Using it directly bypasses
            /// all of the dimensional analysis that having a unit system provides, and should be
            /// avoided whenever possible.
            ///
            /// If using this member is necessary, it is strongly encouraged to wrap the
            /// calculation in a dimensionally-safe interface.
            pub value_unsafe: V,

            /// This member is only temporarily public and so its use is considered unstable.
            /// Right now, the only way to create a `const` with units is with this pattern:
            ///
            /// ```rust
            /// extern crate dimensioned as dim;
            /// use dim::si;
            ///
            /// const x: si::Meter<f64> = si::Meter { value_unsafe: 3.4, _marker: std::marker::PhantomData };
            /// # fn main() {}
            /// ```
            ///
            /// Once `const_fns` is stabilized, that will be able to be replaced with a call to
            /// `Meter::new` and `_marker` will be made private.
            pub _marker: marker::PhantomData<A>,
        }

        impl<V, A> $System<V, A> {
            #[inline]
            pub fn new(v: V) -> Self {
                $System { value_unsafe: v, _marker: marker::PhantomData }
            }
        }

        // --------------------------------------------------------------------------------
        // Implement traits defined in dim::traits

        impl<V, A> Dimensioned for $System<V, A> {
            type Value = V;
            type Units = A;
            #[inline]
            fn new(val: V) -> Self {
                $System::new(val)
            }

            #[inline]
            fn value_unsafe(&self) -> &V {
                &self.value_unsafe
            }
        }

        use $crate::MapUnsafe;
        impl<ValueIn, UnitsIn, ValueOut, UnitsOut> MapUnsafe<ValueOut, UnitsOut> for $System<ValueIn, UnitsIn> {
            type Output = $System<ValueOut, UnitsOut>;
            #[inline]
            fn map_unsafe<F: FnOnce(ValueIn) -> ValueOut>(self, f: F) -> Self::Output {
                $System::new(f(self.value_unsafe))
            }
        }

        use $crate::Map;
        impl<ValueIn, ValueOut> Map<ValueOut> for $Unitless<ValueIn>
        {
            type Output = $Unitless<ValueOut>;
            #[inline]
            fn map<F: FnOnce(ValueIn) -> ValueOut>(self, f: F) -> Self::Output {
                $System::new(f(self.value_unsafe))
            }
        }

        // --------------------------------------------------------------------------------
        // Define type aliases

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
            $(pub type $Derived = derived!(@commas $($derived_rhs)+);)*
        }

        pub type $Unitless<__TypeParameter> = $System<__TypeParameter, inner::$Unitless>;
        $(pub type $Unit<__TypeParameter> = $System<__TypeParameter, inner::$Unit>;)*

        impl<Value> $crate::Dimensionless for $Unitless<Value> {
            #[inline]
            fn value(&self) -> &Value {
                &self.value_unsafe
            }
        }

        $(pub type $Derived<__TypeParameter> = $System<__TypeParameter, inner::$Derived>;)*

        // --------------------------------------------------------------------------------
        // Define consts
        pub mod f32consts {
            use super::*;
            use $crate::reexported::marker::PhantomData;
            pub const $one: $Unitless<f32> = $System { value_unsafe: 1.0, _marker: PhantomData };
            $(pub const $base: $Unit<f32> = $System { value_unsafe: 1.0, _marker: PhantomData };)*
            $(pub const $derived_const: $Derived<f32> = $System { value_unsafe: 1.0, _marker: PhantomData };)*
            $(pub const $constant: $ConstantUnit<f32> = $System { value_unsafe: $constant_value, _marker: PhantomData };)*
        }

        pub mod f64consts {
            use super::*;
            use $crate::reexported::marker::PhantomData;
            pub const $one: $Unitless<f64> = $System { value_unsafe: 1.0, _marker: PhantomData };
            $(pub const $base: $Unit<f64> = $System { value_unsafe: 1.0, _marker: PhantomData };)*
            $(pub const $derived_const: $Derived<f64> = $System { value_unsafe: 1.0, _marker: PhantomData };)*
            $(pub const $constant: $ConstantUnit<f64> = $System { value_unsafe: $constant_value, _marker: PhantomData };)*
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
            #[inline]
            fn recip(self) -> Self::Output { $System::new(self.value_unsafe.recip()) }
        }

        use $crate::typenum::Pow;
        impl<Exp, V, A> Pow<Exp> for $System<V, A>
            where V: Pow<Exp>,
                  A: $crate::reexported::ops::Mul<Exp>,
        {
            type Output = $System< <V as Pow<Exp>>::Output, $crate::typenum::Prod<A, Exp>>;
            #[inline]
            fn powi(self, exp: Exp) -> Self::Output {
                $System::new( self.value_unsafe.powi(exp) )
            }
        }

        impl<Index, V, A> $crate::Root<Index> for $System<V, A>
            where V: $crate::Root<Index>,
                  A: $crate::reexported::ops::Div<Index>,
        {
            type Output = $System< <V as $crate::Root<Index>>::Output, $crate::typenum::Quot<A, Index>>;
            #[inline]
            fn root(self, idx: Index) -> Self::Output {
                $System::new( self.value_unsafe.root(idx) )
                //self.map_unsafe(|v| v.root(idx))
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
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value_unsafe
            }
        }

        // --------------------------------------------------------------------------------
        // Index

        use $crate::reexported::ops::Index;
        impl<V, A, Idx> Index<Idx> for $System<V, A>
            where V: Index<Idx>,
                  <V as Index<Idx>>::Output: Sized,
        {
            type Output = $System<<V as Index<Idx>>::Output, A>;
            #[inline]
            fn index(&self, index: Idx) -> &Self::Output {
                // fixme: ensure this is safe
                unsafe {
                    $crate::reexported::mem::transmute(&self.value_unsafe[index])
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
            #[inline]
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output{
                // fixme: ensure this is safe
                unsafe {
                    $crate::reexported::mem::transmute(self.value_unsafe.index_mut(index))
                }
            }
        }

        // --------------------------------------------------------------------------------
    );
}

/// Create a derived unit based on existing ones.
///
/// If you need a value of some derived unit, then the easiest way is to multiply and divide
/// constants, like so:
///
/// ```rust
/// extern crate dimensioned as dim;
/// use dim::si::M;
///
/// fn main() {
///     let inverse_volume = 3.0 / M/M/M;
/// }
/// ```
///
/// This macro creates types, so, for example, you could use it to make a derived unit that a
/// function can return, as can be seen in the example below.
///
/// This macro is a bit fragile. It requires precise syntax, only supporting the operators `*` and
/// `/` at the moment. It also requires the base type of your unit system and all things from its
/// `inner` module to be in scope.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
///
/// mod derived {
///     use dim::si::inner::*;
///     use dim::si::SI;
///
///     derived!(SI: InverseMeter3 = Unitless / Meter3);
///     derived!(SI: Newton2PerSecond = Newton * Newton / Second);
/// }
/// use derived::InverseMeter3;
///
/// use dim::{Recip, si};
///
/// fn invert_volume(v: si::Meter3<f64>) -> InverseMeter3<f64> {
///     v.recip()
/// }
///
/// fn main() {
///    let x = 3.0 * si::M;
///    let y = 2.0 * si::M;
///    let z = 5.0 * si::M;
///
///    let inverse_volume = invert_volume(x*y*z);
///    assert_eq!(1.0/x/y/z, inverse_volume);
/// }
/// ```
#[macro_export]
macro_rules! derived {
    (@eval $a:ty,) => ($a);
    (@eval $a:ty, *, $b:ty, $($tail:tt)*) =>
        (derived!(@eval $crate::typenum::Sum<$a, $b>, $($tail)* ));
    (@eval $a:ty, /, $b:ty, $($tail:tt)*) =>
        (derived!(@eval $crate::typenum::Diff<$a, $b>, $($tail)* ));
    (@commas $t:ty) => ($t);
    (@commas $($tail:tt)*) => (derived!(@eval $($tail,)*));
    ($System:ident: $name:ident = $($tail:tt)*) => ( pub type $name<__TypeParameter> = $System<__TypeParameter, derived!(@commas $($tail)*)>;);
}
