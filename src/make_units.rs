/// Create a unit system!
///
/// As this macro makes some imports and creates a module named `inner`, it is highly recommended
/// that you place any call to it in its own module.
///
/// fixme: add example
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

    // Implement an XAssign trait. ONLY FOR OPERATIONS THAT PRESERVE UNITS.
    (@assign $System:ident, $Trait:ident, $fun:ident) => (
        impl<Vl, A, Vr> $Trait<$System<Vr, A>> for $System<Vl, A> where
            Vl: $Trait<Vr>,
        {
            fn $fun(&mut self, rhs: $System<Vr, A>) {
                $Trait::$fun(&mut self.value, rhs.value)
            }
        }
    );

    // Implement an XAssign trait with a scalar. ONLY FOR OPERATIONS THAT CAN BE DONE WITH SCALARS.
    (@assign_scalar $System:ident, $Trait:ident, $fun:ident) => (
        impl<Vl, A, Vr> $Trait<Vr> for $System<Vl, A> where
            Vl: $Trait<Vr>,
        {
            fn $fun(&mut self, rhs: Vr) {
                $Trait::$fun(&mut self.value, rhs)
            }
        }
    );

    // Implement a binary operator between something with units and a scalar.
    // Only for when the scalar is on the right.
    // ONLY FOR OPERATIONS THAT CAN BE DONE WITH SCALARS.
    // (@binary_scalar $System:ident, $Trait:ident, $fun:ident) => (
    //     impl<V, A, Rhs> $Trait<Rhs> for $System<V, A> where
    //         V: $Trait<Rhs>,
    //     {
    //         type Output = $System<<V as $Trait<Rhs>>::Output, A>;
    //         fn $fun(self, rhs: Rhs) -> Self::Output {
    //             $System::new( $Trait::$fun(self.value, rhs.value) )
    //         }
    //     }
    // );

    (@fmt S $System:ident $(R $Root:ident P $print_as:ident)* T $Trait:ident E $token:expr) => (
        impl<V, A> $crate::reexported::fmt::$Trait for $System<V, A> where
            V: $crate::reexported::fmt::$Trait,
            $crate::typenum::Length<A>: $crate::generic_array::ArrayLength<isize>,
            A: $crate::typenum::TypeArray + $crate::typenum::Len + $crate::array::ToGA<
                Output = $crate::generic_array::GenericArray<isize, $crate::typenum::Length<A>>
            >,
        {
            fn fmt(&self, f: &mut $crate::reexported::fmt::Formatter) -> Result<(), $crate::reexported::fmt::Error> {
                use $crate::typenum::consts::*;
                use $crate::typenum::Integer;
                let allowed_roots = [$($Root::to_isize()),*];
                let exponents = A::to_ga();
                let print_tokens = [$(stringify!($print_as)),*];

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
         $($Type:ident, $print_as:ident;)+
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
        }

    );

    ($System:ident, $Unitless:ident;
     base {
         $($Root:ident, $Unit:ident, $print_as:ident;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     }
    ) => (
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $System<V, A> {
            value: V,
            _marker: $crate::reexported::marker::PhantomData<A>,
        }

        impl<V, A> $System<V, A> {
            #[inline]
            pub fn new(v: V) -> Self {
                $System { value: v, _marker: $crate::reexported::marker::PhantomData }
            }

            /// Perform an operation on the contained value.
            ///
            /// Use of this function is discouraged, as the operation may be one that does not
            /// perserve units, and this function has no way to protect against that.
            #[inline]
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

        impl<V, A> $crate::Dimension for $System<V, A> {}

        #[doc(hidden)]
        pub mod inner {
            use $crate::typenum::consts::*;
            make_units!(@base_arrays $Unitless, $($Unit, $Root,)*);
            $(pub type $Derived = unit!(@commas $($derived_rhs)+);)*
        }

        pub type $Unitless<__TypeParameter> = $System<__TypeParameter, inner::$Unitless>;
        $(pub type $Unit<__TypeParameter> = $System<__TypeParameter, inner::$Unit>;)*

        impl<__TypeParameter> $crate::Dimensionless for $Unitless<__TypeParameter> {}

        $(pub type $Derived<__TypeParameter> = $System<__TypeParameter, inner::$Derived>);*;

        //--------------------------------------------------------------------------------
        // Formatting

        make_units!(@fmt S $System $(R $Root P $print_as)* T Display E "{} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T Debug E "{:?} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T Octal E "{:o} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T LowerHex E "{:x} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T UpperHex E "{:X} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T Pointer E "{:p} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T Binary E "{:b} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T LowerExp E "{:e} ");
        make_units!(@fmt S $System $(R $Root P $print_as)* T UpperExp E "{:E} ");

        //--------------------------------------------------------------------------------
        // Operator traits from this crate

        impl<V, A> $crate::Recip for $System<V, A> where V: $crate::Recip, A: $crate::reexported::ops::Neg, {
            type Output = $System<<V as $crate::Recip>::Output, $crate::typenum::Negate<A>>;
            fn recip(self) -> Self::Output { $System::new(self.value.recip()) }
        }

        impl<Exp, V, A> $crate::Pow<$System<V, A>> for Exp
            where Exp: $crate::Pow<V>,
                  A: $crate::reexported::ops::Mul<Exp>,
        {
            type Output = $System< <Exp as $crate::Pow<V>>::Output, $crate::typenum::Prod<A, Exp>>;
            fn pow(base: $System<V, A>) -> Self::Output {
                $System::new(Exp::pow(base.value))
            }
        }

        impl<Index, V, A> $crate::Root<$System<V, A>> for Index
            where Index: $crate::typenum::Integer + $crate::Root<V>,
                  A: $crate::reexported::ops::Div<Index>,
        {
            type Output = $System< <Index as $crate::Root<V>>::Output, $crate::typenum::Quot<A, Index>>;
            fn root(radicand: $System<V, A>) -> Self::Output {
                $System::new(Index::root(radicand.value))
            }
        }

        //--------------------------------------------------------------------------------
        // Unary operators

        use $crate::reexported::ops::{Neg, Not};
        use $crate::typenum::Same;
        make_units!(@unary $System, Neg, Same, neg);
        make_units!(@unary $System, Not, Same, not);

        //--------------------------------------------------------------------------------
        // Binary operators

        use $crate::reexported::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign,
                                      BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Rem,
                                      RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign};

        make_units!(@binary $System, Add, Same, add);
        make_units!(@assign $System, AddAssign, add_assign);

        make_units!(@binary $System, BitAnd, Same, bitand);
        make_units!(@assign $System, BitAndAssign, bitand_assign);

        make_units!(@binary $System, BitOr, Same, bitor);
        make_units!(@assign $System, BitOrAssign, bitor_assign);

        make_units!(@binary $System, BitXor, Same, bitxor);
        make_units!(@assign $System, BitXorAssign, bitxor_assign);

        make_units!(@binary $System, Div, Sub, div);
        make_units!(@assign_scalar $System, DivAssign, div_assign);

        make_units!(@binary $System, Mul, Add, mul);
        make_units!(@assign_scalar $System, MulAssign, mul_assign);


        make_units!(@binary $System, Sub, Same, sub);
        make_units!(@assign $System, SubAssign, sub_assign);

        //--------------------------------------------------------------------------------

        // impl<X, Y, A, B> $crate::reexported::ops::Mul<$System<Y, B>> for $System<X, A>
        // where A: $crate::reexported::ops::Add<B>,
        //       X: $crate::reexported::ops::Mul<Y>,
        //       $System<$crate::typenum::Prod<X, Y>, $crate::typenum::Sum<A, B>>: $crate::Dimension,
        // {
        //     type Output = $System<$crate::typenum::Prod<X, Y>, $crate::typenum::Sum<A, B>>;
        //     fn mul(self, rhs: $System<Y, B>) -> Self::Output {
        //         $System::new(self.value * rhs.value)
        //     }
        // }

        // impl<X, Y, A, B> $crate::reexported::ops::Div<$System<Y, B>> for $System<X, A>
        //     where A: $crate::reexported::ops::Sub<B>,
        //           X: $crate::reexported::ops::Div<Y>,
        //           $System<$crate::typenum::Quot<X, Y>, $crate::typenum::Diff<A, B>>: $crate::Dimension,
        // {
        //     type Output = $System<$crate::typenum::Quot<X, Y>, $crate::typenum::Diff<A, B>>;
        //     fn div(self, rhs: $System<Y, B>) -> Self::Output {
        //         $System::new(self.value / rhs.value)
        //     }
        // }
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
