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
        Fruit, Unitless;
        base {
            Apple, a;
            Banana, b;
            Cucumber, c;
            Mango, m;
            Watermelon, w;
        }
        derived {
        }
    }
}
use fruit::{Apple, Banana, Cucumber, Mango, Watermelon};

fn main() {
    let fruit_salad = Apple::new(1.0) * Banana::new(1.0) * Mango::new(1.0) * Mango::new(1.0) * Watermelon::new(1.0);
    println!("Mmmm, delicious: {}", fruit_salad);
    assert_eq!(format!("{}", fruit_salad), "1 a*b*m^2*w");
}
```

The line `Fruit, Unitless;` names the unit system `Fruit` and names its type for
unitless data `Unitless`.

The `base` block is used to define the base units of this system. The line `Apple, a;` creates the unit `Apple` and will use
the token "a" to print `Apple`s.

The `derived` block is not yet implemented, but will be used to define derived units and
constants.
*/
#[macro_export]
macro_rules! make_units {
    ($System:ident, $Unitless:ident;
     base {
         $($Type:ident, $print_as:ident;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        make_units_adv!{
            $System, $Unitless;
            base {
                $(P1, $Type, $print_as;)*
            }
            derived {
                $($Derived = ($($derived_rhs)+);)*
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
        CGS, Unitless;
        base {
            P2, Centimeter, cm;
            P2, Gram, g;
            P1, Second, s;
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
that all constants will be of type `Dim<D, f64>` and will be initialized to a value of
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
    ($System:ident, $Unitless:ident;
     base {
         $($Root:ident, $Type:ident, $print_as:ident;)+
     }
     derived {
         $($Derived:ident = ($($derived_rhs:tt)+);)*
     } ) => (
        #[allow(unused_imports)]
        use $crate::{Z0, P1, P2, P3, P4, P5, P6, P7, P8, P9, N1, N2, N3, N4, N5, N6, N7, N8, N9};
        #[allow(unused_imports)]
        use $crate::Integer;
        #[allow(unused_imports)]
        use $crate::{Dimension, Dimensionless, Dim, Pow, Root, Recip, FmtDim};
        #[allow(unused_imports)]
        use $crate::reexported::ops::{Add, Neg, Sub, Mul, Div};
        #[allow(unused_imports)]
        use $crate::reexported::marker::PhantomData;
        #[allow(unused_imports)]
        use $crate::reexported::fmt;

        #[derive(Copy, Clone)]
        #[allow(non_snake_case)]
        pub struct $System<$($Type: Integer = Z0),*> {
            $($print_as: PhantomData<$Type>),*
        }
        impl<$($Type: Integer),*> Dimension for $System<$($Type),*> {}

        // using $Type and $print_as for these traits is confusing. It should really be $Type_Left
        // and $Type_Right or something, but that is not yet supported by Rust
        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($print_as),*> Mul<$System<$($print_as),*>> for $System<$($Type),*>
        where $($Type: Integer + Add<$print_as>),*,
              $($print_as: Integer),*,
              $(<$Type as Add<$print_as>>::Output: Integer),*,
              $System<$(<$Type as Add<$print_as>>::Output),*>: Dimension,
        {
            type Output = $System<$(<$Type as Add<$print_as>>::Output),*>;
            fn mul(self, _: $System<$($print_as),*>) -> Self::Output { unreachable!()  }
        }

        #[allow(non_camel_case_types)]
        impl<$($Type),*, $($print_as),*> Div<$System<$($print_as),*>> for $System<$($Type),*>
            where $($Type: Integer + Sub<$print_as>),*,
                  $($print_as: Integer),*,
                  $(<$Type as Sub<$print_as>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Sub<$print_as>>::Output),*>;
            fn div(self, _: $System<$($print_as),*>) -> Self::Output { unreachable!()  }
        }

        // Note that this is backwards from the definition of `Pow`. We should be doing:
        // impl<$($Type),*, RHS> Pow<$System<$($Type),*>> for RHS as RHS is really the exponent,
        // but it's in the place of the base. Rust won't let us do that generically, so we've
        // switched them, as the operation on dimensions is multiplication so it's commutative.
        impl<$($Type),*, RHS> Pow<RHS> for $System<$($Type),*>
            where $($Type: Integer + Mul<RHS>),*, RHS: Integer,
                  $(<$Type as Mul<RHS>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Mul<RHS>>::Output),*>;
            fn pow(_: RHS) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*, RHS> Root<RHS> for $System<$($Type),*>
            where $($Type: Integer + Div<RHS>),*, RHS: Integer,
                  $(<$Type as Div<RHS>>::Output: Integer),*
        {
            type Output = $System<$(<$Type as Div<RHS>>::Output),*>;
            fn root(_: RHS) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*> Recip for $System<$($Type),*> where
            $($Type: Integer + Neg),*, $(<$Type as Neg>::Output: Integer),* {
            type Output = $System<$(<$Type as Neg>::Output),*>;
            fn recip(self) -> Self::Output { unreachable!() }
        }

        impl<$($Type),*> FmtDim for $System<$($Type),*>
            where $($Type: Integer),*
        {
            fn fmt(f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                let allowed_roots = [$($Root::to_i32()),*];
                let exponents = [$($Type::to_i32()),*];
                let print_tokens = [$(stringify!($print_as)),*];

                let mut first = true;

                for ((&root, &exp), &token) in
                    allowed_roots.iter()
                    .zip(exponents.iter())
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

        #[doc(hidden)]
        pub mod inner {
            #[allow(unused_imports)]
            use $crate::typenum::consts::*;
            #[allow(unused_imports)]
            pub use $crate::{Dim, Dimensionless, Root};
            #[allow(unused_imports)]
            use super::$System;
            #[allow(unused_imports)]
            pub use $crate::typenum::{Quot, Prod};


            pub type $Unitless = $System;
            impl Dimensionless for $Unitless {}

            __make_inner_base_types!($System, $($Type, $Root),+ |);

            $(pub type $Derived = unit!(@commas $($derived_rhs)+);)*
        }

        pub type $Unitless<__TypeParameter> = Dim<inner::$Unitless, __TypeParameter>;

        $(pub type $Type<__TypeParameter> = Dim<inner::$Type, __TypeParameter>;)*

        $(pub type $Derived<__TypeParameter> = Dim<inner::$Derived, __TypeParameter>;)*
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! __make_inner_base_types {
    ($System:ident, $Type:ident, $Root:ident, $($Types:ident, $Roots:ident),+ | $($Zeros:ident),*)
        => (
        pub type $Type = $System< $($Zeros,)* $Root>;
        __make_inner_base_types!($System, $($Types, $Roots),+ | Z0 $(, $Zeros)*);
        );
    ($System:ident, $Type:ident, $Root:ident | $($Zeros:ident),*) => (
        pub type $Type = $System<$($Zeros,)* $Root>;
        );
}

/// Creates a derived unit based on existing ones.
/// Currently only supports the operations * and /.
///
/// Note that it is important that you place calls to this macro inside their own module, and that
/// module must import `inner::*` from whichever module your unit system is in.
///
/// The ideal way to create derived units is inside the make_units! macro but this also lets you
/// create derived units for systems that are already defined.
///
/// # Example
/// ```rust
/// #[macro_use]
/// extern crate dimensioned as dim;
/// use dim::si::{Meter, Second};
///
/// mod derived {
///     use dim::si::inner::*;
///     unit!(MPS = Meter / Second);
/// }
/// use derived::MPS;
///
/// fn speed(dist: Meter<f64>, time: Second<f64>) -> MPS<f64> {
///     dist / time
/// }
///
/// fn main() {
///     let x = Meter::new(5.0);
///     let t = Second::new(2.0);
///     let v = speed(x, t);
///     let v2 = MPS::new(2.5);
///     assert_eq!(v, v2);
/// }
/// ```
#[macro_export]
macro_rules! unit {
    (@eval $a:ty,) => ($a);
    (@eval $a:ty, *, $b:ty, $($tail:tt)*) =>
        (unit!(@eval $crate::typenum::Prod<$a, $b>, $($tail)* ));
    (@eval $a:ty, /, $b:ty, $($tail:tt)*) =>
        (unit!(@eval $crate::typenum::Quot<$a, $b>, $($tail)* ));
    (@commas $t:ty) => ($t);
    (@commas $($tail:tt)*) => (unit!(@eval $($tail,)*));
    ($name:ident = $($tail:tt)*) => ( pub type $name<T> = Dim<unit!(@commas $($tail)*), T>;);
}
