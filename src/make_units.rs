/** Create a new unit system

This macro is the heart of this library and is used to create the unit systems with which it
ships.

If you find yourself using this macro, please think about whether the unit system you are creating
would be useful to others; if so, submit an issue to get it added to dimensioned.

Rather than try to parse the definition above, we will show an example of calling it, and then
walk through it line by line.

```rust
# #![cfg_attr(feature = "oibit", feature(auto_traits))]
# #![cfg_attr(feature = "oibit", feature(negative_impls))]
#[macro_use]
extern crate dimensioned as dim;

pub mod ms {
    make_units! {
        MS;
        ONE: Unitless;

        base {
            M: Meter, "m", Length;
            S: Second, "s", Time;
        }

        derived {
            MPS: MeterPerSecond = (Meter / Second), Velocity;
            HZ: Hertz = (Unitless / Second), Frequency;

            M3: Meter3 = (Meter * Meter * Meter), Volume;
            M5: Meter5 = (Meter3 * Meter * Meter);
        }

        constants {
            FT: Meter = 0.3048;
            CM: Meter = CENTI * M.value_unsafe;

            MIN: Second = 60.0;
            HR: Second = 60.0 * MIN.value_unsafe;

            PI: Unitless = consts::PI;
        }

        fmt = true;
    }
    pub use self::f64consts::*;
}
# fn main() {}
```

Okay, now let's walk through it.

The macro performs some imports and defines quite a few things, so it's strongly recommended to put it
in its own module.

```ignore
pub mod ms {
```

The first line after calling the macro is just the name we want to give the unit system followed
by a semi-colon. This will be the name of the only type we define; all other type definitions are
aliases to this type with different parameters.

```ignore
    make_units! {
        MS;
```

The next line is the name of the constant and type alias we want for a dimensionless quantity in
your system. That is, when all units have power of 0.

```ignore
        ONE: Unitless;
```

In the `base` block, we define the base units for our system. Each line is of the format `CONST:
Type, "token", Dimension;` where `CONST` is the constant we create, `Type` is the type alias that
we'll make for this unit, `token` is what will show up when we print it, and `Dimension` is
optional. If present, the macro will implement said dimension from the `dimensions` module for this unit.

```ignore
        base {
            M: Meter, "m", Length;
            S: Second, "s", Time;
        }
```

In the `derived` block, we can make derived units from our base units. The beginning is similar; we
have `CONST: Type`. After the equal signs, we have a formula to define this unit. The parentheses
are required, and the only things that can be inside them are the names of other units, and the *
and / operators. Hopefully, this part of the macro will be made more flexibile in the
future. Finally, we again end with an optional dimension. Note that there is none present for the
`M5` line.

```ignore
        derived {
            MPS: MeterPerSecond = (Meter / Second), Velocity;
            HZ: Hertz = (Unitless / Second), Frequency;

            M3: Meter3 = (Meter * Meter * Meter), Volume;
            M5: Meter5 = (Meter3 * Meter * Meter);
        }

```

In the `constants` block, we can define constants of whatever values we wish. Note that the
constants in the `base` and `derived` blocks are always created with a value of 1.0.

All constants are created in both `f32` and `f64` flavors, in the submodules `f32consts` and
`f64consts`, respectively.

In addition, the modules for all integer constants are created. However, these only include
constants for base and derived units. The full list of integer modules is `i8consts`, `i16consts`,
`i32consts`, `i64consts`, `isize_consts`, `u8consts`, `u16consts`, `u32consts`, `u64consts`,
`usize_consts`.

If you would like non-unary integer constants, you will have to construct them yourself, like so

```rust
# extern crate dimensioned as dim;
# use std::marker::PhantomData;
use dim::si;
const MIN: si::Second<u32> = si::Second::new(60);
# fn main() {}
```

Support for making this better is in the works.

In these submodules, the consts from the respective version of `f32prefixes` or `f64prefixes` are in
scope, hence the use of `CENTI` in the `CM` definition.

In addition, the respective version of `core::f32::consts` or `core::f64::consts` is in scope, which
allows the use of `consts::PI` in the `PI definition.

```ignore
        constants {
            FT: Meter = 0.3048;
            CM: Meter = CENTI * M.value_unsafe;

            MIN: Second = 60.0;
            HR: Second = 60.0 * MIN.value_unsafe;

            PI: Unitless = consts::PI;
        }
```

Finally, we have the `fmt` line. This line can either be `fmt = true;` or `fmt = false;`. In either
case, the trait `core::fmt::Debug` is implemented for your unit system, but all of the other `fmt`
traits are implemented only if this is true. Setting it to `false` allows you to have custom
printing for your system.

```ignore
        fmt = true;
    }
```

This line isn't part of the macro, but I wanted to include it as it is in all of the unit systems
defined in dimensioned. It lets us use the `f64` flavor of constants much easier. E.g. we can now
type `ms::M` instead of `ms::f64consts::M`.

```ignore
    pub use self::f64consts::*;
}
```

And that's it! The macro may seem complicated at first, but if you end up using it, I hope that it
starts seeming intuitive fairly quickly.

---

In addition to creating a type, type aliases, and constants, this macro implements many traits for
your unit system, including (but not limited to) the traits in the `traits` module and arithmetic operations.
*/

#[macro_export]
macro_rules! make_units {
    ($System:ident;
     $one:ident: $Unitless:ident;
     base {
         $($base:ident: $Unit:ident, $print_as:expr $(, $base_dim:ident)*;)+
     }
     derived {
         $($derived_const:ident: $Derived:ident = ($($derived_rhs:tt)+) $(, $derived_dim:ident)*;)*
     }
     constants {
         $($constant:ident: $ConstantUnit:ident = $constant_value:expr;)*
     }
     fmt = $to_fmt:ident;
    ) => (
        use $crate::dimcore::marker::PhantomData;
        use $crate::{Dimensioned, Dimensionless};

        /// The struct for this unit system
        #[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash)]
        pub struct $System<V, U> {
            /// This is the value of whatever type we're giving units. Using it directly bypasses
            /// all of the dimensional analysis that having a unit system provides, and should be
            /// avoided whenever possible.
            ///
            /// If using this member is necessary, it is strongly encouraged to wrap the
            /// calculation in a dimensionally-safe interface.
            pub value_unsafe: V,

            _marker: PhantomData<U>,
        }

        impl<V, U> $System<V, U> {

            /// Create a new quantity in the $System unit system
            #[inline]
            pub const fn new(v: V) -> Self {
                $System { value_unsafe: v, _marker: PhantomData }
            }
        }

        impl<V, U> $System<V, U>
        where
            Length<U>: ArrayLength<isize>,
            U: TypeArray + Len + ToGA<Output = GenericArray<isize, Length<U>>>,
        {
            /// Format just the units of this type.
            #[inline]
            fn fmt_units(f: &mut fmt::Formatter) -> Result<(), fmt::Error>
            {
                let exponents = U::to_ga();
                let print_tokens = [$($print_as),+];

                fn write_unit(f: &mut fmt::Formatter, exp: isize, token: &str) -> Result<(), fmt::Error> {
                    if exp == 1 {
                        write!(f, "{}", token)
                    } else {
                        write!(f, "{}^{}", token, exp)
                    }
                }

                let mut units = exponents.into_iter()
                    .zip(print_tokens.iter()).filter(|(exp, _)| *exp != 0);
                if let Some((exp, token)) = units.next() {
                    write_unit(f, exp, token)?;
                }
                for (exp, token) in units {
                    write!(f, "*")?;
                    write_unit(f, exp, token)?;
                }
                Ok(())
            }

            /// Output the units of this type in the same format as in the
            /// formatting traits.
            #[inline]
            #[cfg(feature = "std")]
            pub fn to_string() -> String {
                struct Displayer<U2>(PhantomData<U2>);
                impl<U2> fmt::Display for Displayer<U2>
                where
                    Length<U2>: ArrayLength<isize>,
                    U2: TypeArray + Len + ToGA<Output = GenericArray<isize, Length<U2>>>,
                {
                    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
                    {
                        <$System<f32, U2>>::fmt_units(f)
                    }
                }

                format!("{}", Displayer::<U>(PhantomData))
            }
        }

        // --------------------------------------------------------------------------------
        // Implement traits defined in dim::traits

        impl<V, U> Dimensioned for $System<V, U> {
            type Value = V;
            type Units = U;
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
        impl<ValueIn, UnitsIn, ValueOut, UnitsOut> MapUnsafe<ValueOut, UnitsOut>
            for $System<ValueIn, UnitsIn>
        {
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
        impl<V, U> !NotDim for $System<V, U> {}

        #[doc(hidden)]
        pub mod inner {
            #[allow(unused_imports)]
            use $crate::traits::*;
            #[allow(unused_imports)]
            use $crate::typenum::consts::*;
            __make_units_internal!(@base_arrays $Unitless $($Unit)+);
            $(#[allow(missing_docs)] pub type $Derived =
              __derived_internal!(@mu commas $($derived_rhs)+);)*
        }

        #[allow(missing_docs)]
        pub type $Unitless<V> = $System<V, inner::$Unitless>;
        $(#[allow(missing_docs)]
          pub type $Unit<V> = $System<V, inner::$Unit>;
          $(impl<V> $crate::dimensions::$base_dim for $Unit<V> {})*
        )+

        impl<Value> $crate::Dimensionless for $Unitless<Value> {
            #[inline]
            fn value(&self) -> &Value {
                &self.value_unsafe
            }
        }

        $(#[allow(missing_docs)] pub type $Derived<V> = $System<V, inner::$Derived>;
          $(impl<V> $crate::dimensions::$derived_dim for $Derived<V> {})*
        )*

        // --------------------------------------------------------------------------------
        // Define consts
        macro_rules! define_consts {
            ($module:ident, $prefixes:ident, $t:ident) => (
                /// Constants defined for this system
                pub mod $module {
                    use super::*;
                    use $crate::dimcore::marker::PhantomData;
                    #[allow(unused_imports)] use $crate::dimcore::$t::consts;
                    #[allow(unused_imports)] use $crate::$prefixes::*;
                    #[allow(dead_code, missing_docs)]
                    pub const $one: $Unitless<$t> =
                        $System { value_unsafe: 1.0, _marker: PhantomData };
                    $(#[allow(dead_code, missing_docs)]
                      pub const $base: $Unit<$t> =
                          $System { value_unsafe: 1.0, _marker: PhantomData };)+
                    $(#[allow(dead_code, missing_docs)]
                      pub const $derived_const: $Derived<$t> =
                          $System { value_unsafe: 1.0, _marker: PhantomData };)*
                    $(#[allow(dead_code, missing_docs)]
                      pub const $constant: $ConstantUnit<$t> =
                          $System { value_unsafe: $constant_value, _marker: PhantomData };)*
                }
            );
        }
        define_consts!(f32consts, f32prefixes, f32);
        define_consts!(f64consts, f64prefixes, f64);

        macro_rules! define_int_consts {
            ($module:ident, $t:ident) => (
                /// Constants defined for this system
                pub mod $module {
                    use super::*;
                    use $crate::dimcore::marker::PhantomData;
                    #[allow(dead_code, missing_docs)]
                    pub const $one: $Unitless<$t> =
                        $System { value_unsafe: 1, _marker: PhantomData };
                    $(#[allow(dead_code, missing_docs)]
                      pub const $base: $Unit<$t> =
                          $System { value_unsafe: 1, _marker: PhantomData };)+
                    $(#[allow(dead_code, missing_docs)]
                      pub const $derived_const: $Derived<$t> =
                          $System { value_unsafe: 1, _marker: PhantomData };)*
                }
            );
        }
        define_int_consts!(i8consts, i8);
        define_int_consts!(i16consts, i16);
        define_int_consts!(i32consts, i32);
        define_int_consts!(i64consts, i64);
        define_int_consts!(isize_consts, isize);

        define_int_consts!(u8consts, u8);
        define_int_consts!(u16consts, u16);
        define_int_consts!(u32consts, u32);
        define_int_consts!(u64consts, u64);
        define_int_consts!(usize_consts, usize);


        // --------------------------------------------------------------------------------
        // Formatting
        use $crate::dimcore::fmt;
        use $crate::typenum::{Len, Length, TypeArray};
        use $crate::generic_array::{GenericArray, ArrayLength};
        use $crate::array::ToGA;

        __make_units_internal!(@fmt true S $System $(P $print_as;)+ T Debug E "{:?}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T Display E "{}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T Octal E "{:o}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T LowerHex E "{:x}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T UpperHex E "{:X}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T Pointer E "{:p}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T Binary E "{:b}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T LowerExp E "{:e}");
        __make_units_internal!(@fmt $to_fmt S $System $(P $print_as;)+ T UpperExp E "{:E}");

        // --------------------------------------------------------------------------------
        // Operator traits from this crate

        impl<V, U> $crate::Recip for $System<V, U>
            where V: $crate::Recip, U: $crate::dimcore::ops::Neg,
        {
            type Output = $System<<V as $crate::Recip>::Output, $crate::typenum::Negate<U>>;
            #[inline]
            fn recip(self) -> Self::Output { $System::new(self.value_unsafe.recip()) }
        }

        impl<V: $crate::Abs, U> $crate::Abs for $System<V,U>
        {
            #[inline]
            fn abs(self) -> Self { $System::new(self.value_unsafe.abs()) }
        }

        use $crate::typenum::Pow;
        impl<Exp, V, U> Pow<Exp> for $System<V, U>
            where V: Pow<Exp>,
                  U: $crate::dimcore::ops::Mul<Exp>,
        {
            type Output = $System< <V as Pow<Exp>>::Output, $crate::typenum::Prod<U, Exp>>;
            #[inline]
            fn powi(self, exp: Exp) -> Self::Output {
                $System::new( self.value_unsafe.powi(exp) )
            }
        }

        impl<Index, V, U> $crate::Root<Index> for $System<V, U>
            where V: $crate::Root<Index>,
                  U: $crate::typenum::PartialDiv<Index>,
        {
            type Output = $System<
                <V as $crate::Root<Index>>::Output,
                $crate::typenum::PartialQuot<U, Index>
            >;
            #[inline]
            fn root(self, idx: Index) -> Self::Output {
                $System::new( self.value_unsafe.root(idx) )
            }
        }

        use $crate::typenum::P2;
        impl<V, U> $crate::Sqrt for $System<V, U>
            where V: $crate::Sqrt,
                  U: $crate::typenum::PartialDiv<P2>,
        {
            type Output = $System<
                <V as $crate::Sqrt>::Output, $crate::typenum::PartialQuot<U, P2>>;
            #[inline]
            fn sqrt(self) -> Self::Output {
                $System::new( self.value_unsafe.sqrt() )
            }
        }

        use $crate::typenum::P3;
        impl<V, U> $crate::Cbrt for $System<V, U>
            where V: $crate::Cbrt,
                  U: $crate::typenum::PartialDiv<P3>,
        {
            type Output = $System<
                <V as $crate::Cbrt>::Output,
                $crate::typenum::PartialQuot<U, P3>
            >;
            #[inline]
            fn cbrt(self) -> Self::Output {
                $System::new( self.value_unsafe.cbrt() )
            }
        }

        // --------------------------------------------------------------------------------
        // Operators

        use $crate::dimcore::ops::{Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign,
                        BitXor, BitXorAssign, Div, DivAssign, Mul, MulAssign, Sub,
                        SubAssign, Rem, RemAssign, Neg, Not, Shl, ShlAssign, Shr,
                        ShrAssign};
        use $crate::typenum::{Prod, Quot};

        __make_units_internal!(@ops $System, $Unitless);

        // --------------------------------------------------------------------------------
        // Deref only for dimensionless things

        use $crate::dimcore::ops::Deref;
        impl<V, U> Deref for $System<V, U> where $System<V, U>: Dimensionless {
            type Target = V;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.value_unsafe
            }
        }

        // --------------------------------------------------------------------------------
        // Index

        use $crate::dimcore::ops::Index;
        impl<V, U, Idx> Index<Idx> for $System<V, U>
            where V: Index<Idx>,
        <V as Index<Idx>>::Output: Sized,
        {
            type Output = $System<<V as Index<Idx>>::Output, U>;
            #[inline]
            fn index(&self, index: Idx) -> &Self::Output {
                unsafe {
                    &*(&self.value_unsafe[index] as *const V::Output as *const Self::Output)
                }
            }
        }

        use $crate::dimcore::ops::IndexMut;
        impl<V, U, Idx> IndexMut<Idx> for $System<V, U>
            where $System<V, U>: Index<Idx>,
                  V: Index<Idx> + IndexMut<Idx>,
        <V as Index<Idx>>::Output: Sized,
        <$System<V, U> as Index<Idx>>::Output: Sized
        {
            #[inline]
            fn index_mut(&mut self, index: Idx) -> &mut Self::Output{
                unsafe {
                    &mut *(self.value_unsafe.index_mut(index) as *mut V::Output as *mut Self::Output)
                }
            }
        }

        // --------------------------------------------------------------------------------
        // Default

        use $crate::dimcore::default::Default;
        impl<V, U> Default for $System<V,U> where V: Default {
            fn default() -> Self {
                $System::new( V::default() )
            }
        }

        // --------------------------------------------------------------------------------
        // Sum

        use $crate::dimcore::iter::{Iterator, Sum};
        impl<V, U> Sum for $System<V, U> where V: Sum {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                let sum = iter.map(|item| item.value_unsafe).sum();
                $System::new(sum)
            }
        }

        // --------------------------------------------------------------------------------
        // ApproxEq
        #[cfg(feature = "approx")]
        impl<V, U> $crate::approx::AbsDiffEq for $System<V, U> where V: $crate::approx::AbsDiffEq, U: PartialEq {
            type Epsilon = $System<V::Epsilon, U>;

            fn default_epsilon() -> Self::Epsilon {
                $System::new(V::default_epsilon())
            }

            fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
                true || self.value_unsafe.abs_diff_eq(&other.value_unsafe, epsilon.value_unsafe)
            }
        }

        #[cfg(feature = "approx")]
        impl<V, U> $crate::approx::RelativeEq for $System<V, U> where V: $crate::approx::RelativeEq, U: PartialEq {
            fn default_max_relative() -> Self::Epsilon {
                $System::new(V::default_max_relative())
            }

            fn relative_eq(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon
            ) -> bool {
                self.value_unsafe.relative_eq(
                    &other.value_unsafe,
                    epsilon.value_unsafe,
                    max_relative.value_unsafe,
                )
            }

            fn relative_ne(
                &self,
                other: &Self,
                epsilon: Self::Epsilon,
                max_relative: Self::Epsilon
            ) -> bool {
                self.value_unsafe.relative_ne(
                    &other.value_unsafe,
                    epsilon.value_unsafe,
                    max_relative.value_unsafe,
                )
            }
        }

        #[cfg(feature = "approx")]
        impl<V, U> $crate::approx::UlpsEq for $System<V, U> where V: $crate::approx::UlpsEq, U: PartialEq {
            fn default_max_ulps() -> u32 {
                V::default_max_ulps()
            }

            fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.value_unsafe.ulps_eq(&other.value_unsafe, epsilon.value_unsafe, max_ulps)
            }

            fn ulps_ne(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
                self.value_unsafe.ulps_ne(&other.value_unsafe, epsilon.value_unsafe, max_ulps)
            }
        }
        // --------------------------------------------------------------------------------
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_units_internal {
    (@ops $System:ident, $Unitless:ident) => (
        // -------------------------------------------------------------------------------
        // Unary: Neg, Not

        macro_rules! unary_op {
            ($Trait:ident, $fun:ident) => (
                impl<V, U> $Trait for $System<V, U> where
                    V: $Trait,
                {
                    type Output = $System<<V as $Trait>::Output, U>;
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
                impl<Vl, U, Vr> $Trait<$System<Vr, U>> for $System<Vl, U> where
                    Vl: $Trait<Vr>,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, U>;
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, U>) -> Self::Output {
                        $System::new($Trait::$fun(self.value_unsafe, rhs.value_unsafe))
                    }
                }

                // Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, U, Vr> $Trait<Vr> for $System<Vl, U> where
                    Vl: $Trait<Vr>, Vr: NotDim, $System<Vl, U>: Dimensionless
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, U>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new($Trait::$fun(self.value_unsafe, rhs))
                    }
                }

                // Assign: Both have units
                impl<Vl, U, Vr> $TraitAssign<$System<Vr, U>> for $System<Vl, U> where
                    Vl: $TraitAssign<Vr>,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, U>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Unitless on lhs, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, U, Vr> $TraitAssign<Vr> for $System<Vl, U> where
                    Vl: $TraitAssign<Vr>, Vr: NotDim, $System<Vl, U>: Dimensionless
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
                impl<Vl, Ul, Vr, Ur> $Trait<$System<Vr, Ur>> for $System<Vl, Ul> where
                    Vl: $Trait<Vr>, Ul: $op<Ur>,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, <Ul as $op<Ur>>::Output>;
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, Ur>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs.value_unsafe) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, U, Vr> $Trait<Vr> for $System<Vl, U> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, U>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Ul, Vr, Ur> $TraitAssign<$System<Vr, Ur>> for $System<Vl, Ul> where
                    Vl: $TraitAssign<Vr>,
                $System<Vr, Ur>: Dimensionless,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ur>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, U, Vr> $TraitAssign<Vr> for $System<Vl, U> where
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
        impl<Vl, Ul, Vr, Ur> Rem<$System<Vr, Ur>> for $System<Vl, Ul> where
            Vl: Rem<Vr>
        {
            type Output = $System<<Vl as Rem<Vr>>::Output, Ul>;
            #[inline]
            fn rem(self, rhs: $System<Vr, Ur>) -> Self::Output {
                $System::new( self.value_unsafe % rhs.value_unsafe )
            }
        }

        // Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, U, Vr> Rem<Vr> for $System<Vl, U> where
            Vl: Rem<Vr>, Vr: NotDim,
        {
            type Output = $System<<Vl as Rem<Vr>>::Output, U>;
            #[inline]
            fn rem(self, rhs: Vr) -> Self::Output {
                $System::new( self.value_unsafe % rhs )
            }
        }

        // Assign, both have units
        impl<Vl, Ul, Vr, Ur> RemAssign<$System<Vr, Ur>> for $System<Vl, Ul> where
            Vl: RemAssign<Vr>,
        {
            #[inline]
            fn rem_assign(&mut self, rhs: $System<Vr, Ur>) {
                self.value_unsafe %= rhs.value_unsafe
            }
        }

        // Assign: Lhs has units, scalar on rhs
        #[cfg(feature = "oibit")]
        impl<Vl, U, Vr> RemAssign<Vr> for $System<Vl, U> where
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
                impl<Vl, Ul, Vr, Ur> $Trait<$System<Vr, Ur>> for $System<Vl, Ul> where
                    Vl: $Trait<Vr>, $System<Vr, Ur>: Dimensionless
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, Ul>;
                    #[inline]
                    fn $fun(self, rhs: $System<Vr, Ur>) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs.value_unsafe) )
                    }
                }

                // Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, Ul, Vr> $Trait<Vr> for $System<Vl, Ul> where
                    Vl: $Trait<Vr>, Vr: NotDim,
                {
                    type Output = $System<<Vl as $Trait<Vr>>::Output, Ul>;
                    #[inline]
                    fn $fun(self, rhs: Vr) -> Self::Output {
                        $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                    }
                }

                // Assign: Lhs has units, rhs unitless
                impl<Vl, Ul, Vr, Ur> $TraitAssign<$System<Vr, Ur>> for $System<Vl, Ul> where
                    Vl: $TraitAssign<Vr>,
                $System<Vr, Ur>: Dimensionless,
                {
                    #[inline]
                    fn $fun_assign(&mut self, rhs: $System<Vr, Ur>) {
                        $TraitAssign::$fun_assign(&mut self.value_unsafe, rhs.value_unsafe)
                    }
                }

                // Assign: Lhs has units, scalar on rhs
                #[cfg(feature = "oibit")]
                impl<Vl, U, Vr> $TraitAssign<Vr> for $System<Vl, U> where
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
                        impl<V, U> $Trait<$t> for $System<V, U> where
                            V: $Trait<$t>, $System<V, U>: Dimensionless
                        {
                            type Output = $System<<V as $Trait<$t>>::Output, U>;
                            #[inline]
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                            }
                        }

                        // Primitive on lhs, unitless on rhs
                        impl<V, U> $Trait<$System<V, U>> for $t where
                            $t: $Trait<V>, $System<V, U>: Dimensionless
                        {
                            type Output = $System<<$t as $Trait<V>>::Output, U>;
                            #[inline]
                            fn $fun(self, rhs: $System<V, U>) -> Self::Output {
                                $System::new( $Trait::$fun(self, rhs.value_unsafe) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, U> $TraitAssign<$t> for $System<V, U> where
                            V: $TraitAssign<$t>, $System<V, U>: Dimensionless
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
                        impl<V, U> $Trait<$t> for $System<V, U> where
                            V: $Trait<$t>
                        {
                            type Output = $System<<V as $Trait<$t>>::Output, U>;
                            #[inline]
                            fn $fun(self, rhs: $t) -> Self::Output {
                                $System::new( $Trait::$fun(self.value_unsafe, rhs) )
                            }
                        }

                        // Assign: Unitless on lhs, primitive on rhs
                        #[cfg(not(feature = "oibit"))]
                        impl<V, U> $TraitAssign<$t> for $System<V, U> where
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
                impl<V, U> Mul<$System<V, U>> for $t where $t: Mul<V> {
                    type Output = $System<Prod<$t, V>, U>;
                    #[inline]
                    fn mul(self, rhs: $System<V, U>) -> Self::Output {
                        $System::new(self * rhs.value_unsafe)
                    }
                }

                // Div: Primitive on lhs, units on rhs
                impl<V, U> Div<$System<V, U>> for $t where $t: Div<V>, U: Neg {
                    type Output = $System<Quot<$t, V>, <U as Neg>::Output>;
                    #[inline]
                    fn div(self, rhs: $System<V, U>) -> Self::Output {
                        $System::new(self / rhs.value_unsafe)
                    }
                }

                // Rem: Primitive on lhs, units on rhs
                impl<V, U> Rem<$System<V, U>> for $t where $t: Rem<V> {
                    type Output = $Unitless<<$t as Rem<V>>::Output>;
                    #[inline]
                    fn rem(self, rhs: $System<V, U>) -> Self::Output {
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

    (@fmt true S $System:ident $(P $print_as:expr;)* T $Trait:ident E $token:expr) => (
        impl<V, U> fmt::$Trait for $System<V, U> where
            V: fmt::$Trait,
        Length<U>: ArrayLength<isize>,
            U: TypeArray + Len + ToGA<Output = GenericArray<isize, Length<U>>>,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
            {
                self.value_unsafe.fmt(f)?;
                if U::to_ga().iter().any(|&exp| exp != 0) {
                    write!(f, " ")?;
                    Self::fmt_units(f)?;
                }
                Ok(())
            }
        }
    );

    (@fmt false S $System:ident $(P $print_as:expr;)* T $Trait:ident E $token:expr) => ();

    // define arrays for all the base units
    (@base_arrays $Unitless:ident $Unit:ident $($Units:ident)*) => (
        pub type $Unitless = tarr![Z0, $(__make_units_internal!(@convert_to_zero $Units)),*];
        __make_units_internal!(@next_array U $Unit $(U $Units)* $(E $Units)*);
    );

    (@next_array U $Unit:ident $(U $Units:ident)*
     $(F $FrontZeros:ident)* E $Zero:ident $(E $EndZeros:ident)*) => (
        pub type $Unit = tarr![
            $(__make_units_internal!(@convert_to_zero $FrontZeros),)*
                P1,
            Z0 $(, __make_units_internal!(@convert_to_zero $EndZeros))*
        ];
        __make_units_internal!(@next_array $(U $Units)* $(F $FrontZeros)* F $Zero $(E $EndZeros)*);
    );

    (@next_array U $Unit:ident $(F $FrontZeros:ident)*) => (
        pub type $Unit = tarr![
            $(__make_units_internal!(@convert_to_zero $FrontZeros),)*
                P1
        ];
    );

    (@convert_to_zero $Unit:ident) => ( Z0 );
    (@convert_to_zero) => ();
}

/// Create a derived unit based on existing ones
///
/// This macro creates a type, so it is useful when you need to directly express the type of a
/// derived unit that is not defined in its unit system.
///
/// If you need a variable of some derived unit, then the easiest way is to manipulate constants,
/// like so:
///
/// ```rust
/// # extern crate dimensioned as dim;
/// use dim::si::M;
///
/// # fn main() {
/// let inverse_volume = 3.0 / M / M / M;
/// # }
/// ```
///
/// This macro is a bit fragile. It only supports the operators `*` and `/` and no parentheses. It
/// requires the base type of your unit system and the module it was defined in to be in scope.
///
/// Use it like so:
///
/// ```rust
/// # #[macro_use] extern crate dimensioned as dim;
/// use dim::si::{self, SI};
/// derived!(si, SI: InverseMeter3 = Unitless / Meter3);
/// # fn main() {}
/// ```
///
/// You may use any of the base or derived units that come with a unit system (but none created by
/// this macro) on the right-hand side of the expression.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
///
/// use dim::si::{self, SI};
///
/// derived!(si, SI: InverseMeter3 = Unitless / Meter3);
/// derived!(si, SI: Newton2PerSecond = Newton * Newton / Second);
///
/// use dim::Recip;
/// fn invert_volume(v: si::Meter3<f64>) -> InverseMeter3<f64> {
///     v.recip()
/// }
///
/// fn main() {
///     let v = 12.0 * si::M3;
///
///     let inverse_volume = invert_volume(v);
///     assert_eq!(1.0 / v, inverse_volume);
/// }
/// ```
#[macro_export]
macro_rules! derived {
    ($module:ident, $System:ident: $name:ident = $($tail:tt)*) => (
        pub type $name<V> = $System<V, __derived_internal!(@commas $module, $($tail)*)>;
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __derived_internal {
    //------------------------------------------
    // For derived:

    (@eval $module:ident, $a:ty,) => ($a);

    // Both qualify as identifiers
    (@eval $module:ident, $a:ident, /, $b:ident, $($tail:tt)*) => (
        __derived_internal!(
            @eval $module,
            $crate::typenum::Diff<$module::inner::$a, $module::inner::$b>,
            $($tail)*
        )
    );
    (@eval $module:ident, $a:ident, *, $b:ident, $($tail:tt)*) => (
        __derived_internal!(
            @eval $module,
            $crate::typenum::Sum<$module::inner::$a, $module::inner::$b>,
            $($tail)*
        )
    );

    // $a is an intermediate result:
    (@eval $module:ident, $a:ty, /, $b:ident, $($tail:tt)*) => (
        __derived_internal!(
            @eval $module,
            $crate::typenum::Diff<$a, $module::inner::$b>,
            $($tail)*
        )
    );
    (@eval $module:ident, $a:ty, *, $b:ident, $($tail:tt)*) => (
        __derived_internal!(@eval $module, $crate::typenum::Sum<$a, $module::inner::$b>, $($tail)* )
    );

    (@commas $module:ident, $t:ty) => ($t);
    (@commas $module:ident, $($tail:tt)*) => (__derived_internal!(@eval $module, $($tail,)*));


    //------------------------------------------
    // For make_units:

    (@mu eval $a:ty,) => ($a);
    (@mu eval $a:ty, *, $b:ty, $($tail:tt)*) =>
        (__derived_internal!(@mu eval $crate::typenum::Sum<$a, $b>, $($tail)* ));
    (@mu eval $a:ty, /, $b:ty, $($tail:tt)*) =>
        (__derived_internal!(@mu eval $crate::typenum::Diff<$a, $b>, $($tail)* ));
    (@mu commas $t:ty) => ($t);
    (@mu commas $($tail:tt)*) => (__derived_internal!(@mu eval $($tail,)*));
}

/// Implement rand traits for a unit system.
///
/// You will want to use this macro if you want to generate random
/// values with this type.  Specifically, this allows you to use the
/// `Standard` and `Uniform` distributions to generate dimensioned
/// types, provided your underlying data type supports those
/// distributions.
///
/// ```rust,ignore
/// impl_rand!(UnitSystem);
/// ```
#[cfg(feature = "rand")]
#[macro_export]
macro_rules! impl_rand {
    ($System:ident) => {
        use $crate::rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformSampler};
        use $crate::rand::distributions::{Distribution, Standard};
        use $crate::rand::Rng;

        impl<V, U> Distribution<$System<V, U>> for Standard
        where
            Standard: Distribution<V>,
        {
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> $System<V, U> {
                $System::new(self.sample(rng))
            }
        }

        /// This is an internal implementation detail that the rand
        /// crate requires us to expose.  Sorry about that.
        #[derive(Clone, Copy, Debug)]
        pub struct MyUniformSampler<V: SampleUniform, U> {
            inner: V::Sampler,
            _marker: PhantomData<U>,
        }
        impl<V: SampleUniform, U> UniformSampler for MyUniformSampler<V, U> {
            type X = $System<V, U>;
            fn new<B1, B2>(low: B1, high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                MyUniformSampler {
                    inner: V::Sampler::new(
                        low.borrow().value_unsafe(),
                        high.borrow().value_unsafe(),
                    ),
                    _marker: PhantomData,
                }
            }
            fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
            where
                B1: SampleBorrow<Self::X> + Sized,
                B2: SampleBorrow<Self::X> + Sized,
            {
                UniformSampler::new(low, high)
            }
            fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
                $System::new(self.inner.sample(rng))
            }
        }
        impl<V: SampleUniform, U> SampleUniform for $System<V, U> {
            type Sampler = MyUniformSampler<V, U>;
        }
    };
}

/// Implement serde traits for a unit system.
///
/// The implementations generated by this macro only serialize the
/// numeric values - not the actual units. Therefore, serialization is
/// dimensionally unsafe, but it does not add any overhead over using
/// plain numeric types.
///
/// All of the unit systems defined in this crate implement
/// `Serialize` and `Deserialize` using this macro. If you define your
/// own system you may use this macro to implement those traits
/// automatically, or define them yourself:
///
/// ```rust,ignore
/// impl_serde!(UnitSystem);
/// ```
#[cfg(feature = "serde")]
#[macro_export]
macro_rules! impl_serde {
    ($System:ident) => {
        use $crate::serde::{Deserialize, Deserializer, Serialize, Serializer};

        impl<'de, V, U> Deserialize<'de> for $System<V, U>
        where
            V: Deserialize<'de>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                let value_unsafe = V::deserialize(deserializer)?;
                Ok($System {
                    value_unsafe,
                    _marker: PhantomData,
                })
            }
        }

        impl<V, U> Serialize for $System<V, U>
        where
            V: Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
            {
                self.value_unsafe.serialize(serializer)
            }
        }
    };
}

/// Implement ClapMe trait for a unit system.
///
/// ```rust,ignore
/// impl_clapme!(UnitSystem);
/// ```
#[cfg(feature = "clapme")]
#[macro_export]
macro_rules! impl_clapme {
    ($System:ident) => {
        use $crate::clapme::ClapMe;

        impl<V: ClapMe, U> ClapMe for $System<V, U> {
            fn with_clap<T>(
                info: ::clapme::ArgInfo,
                app: ::clapme::clap::App,
                f: impl FnOnce(::clapme::clap::App) -> T,
            ) -> T {
                V::with_clap(info, app, f)
            }
            fn from_clap(name: &str, matches: &::clapme::clap::ArgMatches) -> Option<Self> {
                V::from_clap(name, matches).map(|v| $System {
                    value_unsafe: v,
                    _marker: PhantomData,
                })
            }
            fn requires_flags(name: &str) -> Vec<String> {
                V::requires_flags(name)
            }
        }
    };
}

/// Implement AutoArgs trait for a unit system.
///
/// ```rust,ignore
/// impl_auto_args!(UnitSystem);
/// ```
#[cfg(feature = "auto-args")]
#[macro_export]
macro_rules! impl_auto_args {
    ($System:ident) => {
        impl<V: auto_args::AutoArgs, U> auto_args::AutoArgs for $System<V, U>
        where
            Length<U>: ArrayLength<isize>,
            U: TypeArray + Len + ToGA<Output = GenericArray<isize, Length<U>>>,
        {
            fn parse_internal(
                key: &str,
                args: &mut Vec<std::ffi::OsString>,
            ) -> Result<Self, auto_args::Error> {
                let v = V::parse_internal(key, args)?;
                Ok($System {
                    value_unsafe: v,
                    _marker: PhantomData,
                })
            }
            const REQUIRES_INPUT: bool = true;
            fn tiny_help_message(key: &str) -> String {
                if key == "" {
                    format!("FLOAT in units {}", <$System<V, U>>::to_string())
                } else {
                    format!("{} FLOAT in units {}", key, <$System<V, U>>::to_string())
                }
            }
        }
    };
}

// tests to see if we can rewrite derived! macro better
// #[test]
// fn derived2_test() {
//     use si;
//     use typenum::Prod;

//     type MeterSecond<V> = Prod<si::Meter<V>, si::Second<V>>;

//     let x = MeterSecond::new(1.0);

//     assert_eq!(x, si::M * si::S);
// }
