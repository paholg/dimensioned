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

// implement a unary operation
    (@unary $System:ident, $Trait:ident, $op:ident, $fun:ident) => (
        impl<V, A> $Trait for $System<V, A> where
            A: $op,
            V: $Trait,
        {
            type Output = $System<<V as $Trait>::Output, <A as $op>::Output>;
            fn $fun(self) -> Self::Output {
                $System::new( $Trait::$fun(self.value) )
            }
        }
    );

// implement a binary operation between two things with units
    (@binary $System:ident, $Trait:ident, $op:ident, $fun:ident) => (
        impl<Vl, Al, Vr, Ar> $Trait<$System<Vr, Ar>> for $System<Vl, Al> where
            Al: $op<Ar>,
            Vl: $Trait<Vr>,
        {
            type Output = $System<<Vl as $Trait<Vr>>::Output, <Al as $op<Ar>>::Output>;
            fn $fun(self, rhs: $System<Vr, Ar>) -> Self::Output {
                $System::new( $Trait::$fun(self.value, rhs.value) )
            }
        }
    );

// Implement an XAssign trait. ONLY FOR OPERATIONS THAT PRESERVE UNITS (e.g. Add, Sub).
    (@assign $System:ident, $Trait:ident, $fun:ident) => (
        impl<Vl, A, Vr> $Trait<$System<Vr, A>> for $System<Vl, A> where
            Vl: $Trait<Vr>,
        {
            fn $fun(&mut self, rhs: $System<Vr, A>) {
                $Trait::$fun(&mut self.value, rhs.value)
            }
        }

        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
            Vl: $Trait<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
        {
            fn $fun(&mut self, rhs: Vr) {
                $Trait::$fun(&mut self.value, rhs)
            }
        }

        macro_rules! prim {
            ($head:tt) => (
                #[cfg(not(feature = "oibit"))]
                impl<V, A> $Trait<$head> for $System<V, A> where
                    V: $Trait<$head>, $System<V, A>: Dimensionless
                {
                    fn $fun(&mut self, rhs: $head) {
                        $Trait::$fun(&mut self.value, rhs)
                    }
                }
            );
            () => ();
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
    );

// Implement a binary operator between the Unitless type of a unit system and a scalar.
// FOR OPERATIONS THAT PRESERVE TYPE (e.g. Add, Sub)
    (@binary_scalar_dimless $System:ident, $Trait:ident, $fun:ident) => (
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
            Vl: $Trait<Vr>, Vr: NotDim, $System<Vl, A>: Dimensionless
        {
            type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
            fn $fun(self, rhs: Vr) -> Self::Output {
                $System::new( $Trait::$fun(self.value, rhs) )
            }
        }

        macro_rules! prim {
            ($head:tt) => (
                #[cfg(not(feature = "oibit"))]
                impl<V, A> $Trait<$head> for $System<V, A> where
                    V: $Trait<$head>, $System<V, A>: Dimensionless
                {
                    type Output = $System<<V as $Trait<$head>>::Output, A>;
                    fn $fun(self, rhs: $head) -> Self::Output {
                        $System::new($Trait::$fun(self.value, rhs))
                    }
                }

                impl<V, A> $Trait<$System<V, A>> for $head where
                    $head: $Trait<V>, $System<V, A>: Dimensionless
                {
                    type Output = $System<<$head as $Trait<V>>::Output, A>;
                    fn $fun(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new($Trait::$fun(self, rhs.value))
                    }
                }
            );
            () => ();
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
    );

// Implement an XAssign trait with a scalar. ONLY FOR OPERATIONS THAT CAN BE DONE WITH SCALARS (e.g. Mul, Div)
    (@assign_scalar $System:ident, $Trait:ident, $fun:ident) => (
        impl<Vl, Al, Vr, Ar> $Trait<$System<Vr, Ar>> for $System<Vl, Al> where
            Vl: $Trait<Vr>,
            Vr: Dimensionless,
        {
            fn $fun(&mut self, rhs: $System<Vr, Ar>) {
                $Trait::$fun(&mut self.value, rhs.value)
            }
        }

        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
            Vl: $Trait<Vr>, Vr: NotDim,
        {
            fn $fun(&mut self, rhs: Vr) {
                $Trait::$fun(&mut self.value, rhs)
            }
        }

        macro_rules! prim {
            ($head:tt) => (
                #[cfg(not(feature = "oibit"))]
                impl<V, A> $Trait<$head> for $System<V, A> where
                    V: $Trait<$head>,
                {
                    fn $fun(&mut self, rhs: $head) {
                        $Trait::$fun(&mut self.value, rhs)
                    }
                }
            );
            () => ();
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
    );

// Implement a binary operator between something with units and a scalar.
// ONLY FOR OPERATIONS THAT CAN BE DONE WITH SCALARS (e.g. Mul, Div)
// Only for scalar on the rhs.
    (@binary_scalar $System:ident, $Trait:ident, $fun:ident) => (
        #[cfg(feature = "oibit")]
        impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
            Vl: $Trait<Vr>, Vr: NotDim,
        {
            type Output = $System<<Vl as $Trait<Vr>>::Output, A>;
            fn $fun(self, rhs: Vr) -> Self::Output {
                $System::new( $Trait::$fun(self.value, rhs) )
            }
        }

        macro_rules! prim {
            ($head:tt) => (
                #[cfg(not(feature = "oibit"))]
                impl<V, A> $Trait<$head> for $System<V, A> where
                    V: $Trait<$head>,
                {
                    type Output = $System<<V as $Trait<$head>>::Output, A>;
                    fn $fun(self, rhs: $head) -> Self::Output {
                        $System::new($Trait::$fun(self.value, rhs))
                    }
                }
            );
            () => ();
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

    ($System:ident, $Unitless:ident;
     base {
         $($Type:ident, $print_as:expr;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        make_units!{
            $System, $Unitless;
            base {
                $(P1, $Type, $print_as;)*
            }
            derived {
                $($Derived = ($($derived_rhs)+);)*
            }
            fmt = true;
        }

    );

        ($System:ident, $Unitless:ident;
     base {
         $($Type:ident, $print_as:expr;)+
     }
        ) => (
        make_units!{
            $System, $Unitless;
            base {
                $(P1, $Type, $print_as;)*
            }
            derived {
            }
            fmt = true;
        }

        );

    ($System:ident, $Unitless:ident;
     base {
         $($Type:ident, $print_as:expr;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     }
     fmt = $to_fmt:ident;
    ) => (
        make_units!{
            $System, $Unitless;
            base {
                $(P1, $Type, $print_as;)*
            }
            derived {
                $($Derived = ($($derived_rhs)+);)*
            }
            fmt = $to_fmt;
        }

    );

        ($System:ident, $Unitless:ident;
     base {
         $($Type:ident, $print_as:expr;)+
     }
         fmt = $to_fmt:ident;
        ) => (
        make_units!{
            $System, $Unitless;
            base {
                $(P1, $Type, $print_as;)*
            }
            derived {
            }
            fmt = $to_fmt;
        }

    );

    ($System:ident, $Unitless:ident;
     base {
         $($Root:ident, $Unit:ident, $print_as:expr;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     }
     fmt = $to_fmt:ident;
    ) => (
        use $crate::reexported::marker;

        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $System<V, A> {
            pub value: V,
            _marker: marker::PhantomData<A>,
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

        use $crate::Dimension;
        impl<V, A> Dimension<V> for $System<V, A> {}

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
// Formatting
        use $crate::reexported::fmt;
        use $crate::typenum::{Len, Length, TypeArray};
        use $crate::generic_array::{GenericArray, ArrayLength};
        use $crate::array::ToGA;

        make_units!(@fmt true S $System $(R $Root P $print_as;)* T Debug E "{:?} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Display E "{} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Octal E "{:o} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T LowerHex E "{:x} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T UpperHex E "{:X} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Pointer E "{:p} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T Binary E "{:b} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T LowerExp E "{:e} ");
        make_units!(@fmt $to_fmt S $System $(R $Root P $print_as;)* T UpperExp E "{:E} ");

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
// Unary operators

        use $crate::reexported::ops::{Neg, Not};
        use $crate::typenum::Same;
        make_units!(@unary $System, Neg, Same, neg);
        make_units!(@unary $System, Not, Same, not);

// --------------------------------------------------------------------------------
// Binary operators

        use $crate::reexported::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign,
                                      BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub,
                                      SubAssign};
        use $crate::typenum::{Prod, Quot};

        make_units!(@binary $System, Add, Same, add);
        make_units!(@assign $System, AddAssign, add_assign);
        make_units!(@binary_scalar_dimless $System, Add, add);

        make_units!(@binary $System, Sub, Same, sub);
        make_units!(@assign $System, SubAssign, sub_assign);
        make_units!(@binary_scalar_dimless $System, Sub, sub);

        make_units!(@binary $System, Div, Sub, div);
        make_units!(@binary_scalar $System, Div, div);
        make_units!(@assign_scalar $System, DivAssign, div_assign);

        make_units!(@binary $System, Mul, Add, mul);
        make_units!(@binary_scalar $System, Mul, mul);
        make_units!(@assign_scalar $System, MulAssign, mul_assign);

        macro_rules! lhs_ops {
            ($t: ty) => (
                impl<V, A> Mul<$System<V, A>> for $t where $t: Mul<V> {
                    type Output = $System<Prod<$t, V>, A>;
                    #[inline]
                    fn mul(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self*rhs.value)
                    }
                }

                impl<V, A> Div<$System<V, A>> for $t where $t: Div<V>, A: Neg {
                    type Output = $System<Quot<$t, V>, <A as Neg>::Output>;
                    #[inline]
                    fn div(self, rhs: $System<V, A>) -> Self::Output {
                        $System::new(self / rhs.value)
                    }
                }
            );
        }

        lhs_ops!(f32);
        lhs_ops!(f64);
        lhs_ops!(i8);
        lhs_ops!(i16);
        lhs_ops!(i32);
        lhs_ops!(i64);
        lhs_ops!(isize);
        lhs_ops!(u8);
        lhs_ops!(u16);
        lhs_ops!(u32);
        lhs_ops!(u64);
        lhs_ops!(usize);

// Bit operations probably aren't useful, but may as well define them.
        make_units!(@binary $System, BitAnd, Same, bitand);
        make_units!(@assign $System, BitAndAssign, bitand_assign);
        make_units!(@binary_scalar_dimless $System, BitAnd, bitand);

        make_units!(@binary $System, BitOr, Same, bitor);
        make_units!(@assign $System, BitOrAssign, bitor_assign);
        make_units!(@binary_scalar_dimless $System, BitOr, bitor);

        make_units!(@binary $System, BitXor, Same, bitxor);
        make_units!(@assign $System, BitXorAssign, bitxor_assign);
        make_units!(@binary_scalar_dimless $System, BitXor, bitxor);

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
