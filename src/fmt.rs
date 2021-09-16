use core::fmt;
use typenum::Integer;

macro_rules! format_cgs_like {
    ($System:ident; $tokens:expr; $($Trait:ident)*) => (
        $(impl<V, U1, U2, U3> fmt::$Trait for $System<V, tarr![U1, U2, U3]> where
            V: fmt::$Trait, U1: Integer, U2: Integer, U3: Integer,
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
            {
                // double U3 so we can treat them all the same, as sqrts
                let exponents = [U1::to_isize(), U2::to_isize(), U3::to_isize()*2];
                let print_tokens = $tokens;

                self.value_unsafe.fmt(f)?;

                let mut first = true;
                for (&exp, token) in
                    exponents.iter()
                    .zip(print_tokens.iter())
                {
                    if first {
                        if exp != 0 {
                            first = false;
                            write!(f, " ")?;
                        }
                    } else if exp != 0 {
                        write!(f, "*")?;
                    }

                    match exp {
                        0 => (),
                        2 => write!(f, "{}", token)?,
                        _ if exp % 2 == 0 => write!(f, "{}^{}", token, exp/2)?,
                        _ => write!(f, "{}^{}", token, (exp as f32)/2.0)?,
                    }
                }
                Ok(())
            }
        })*
    );
}

use crate::unit_systems::cgs::CGS;
format_cgs_like!(CGS; ["cm", "g", "s"]; Display Octal LowerHex UpperHex Pointer Binary LowerExp UpperExp);

use crate::unit_systems::mks::MKS;
format_cgs_like!(MKS; ["m", "k", "s"]; Display Octal LowerHex UpperHex Pointer Binary LowerExp UpperExp);

use crate::unit_systems::fps::FPS;
format_cgs_like!(FPS; ["ft", "lb", "s"]; Display Octal LowerHex UpperHex Pointer Binary LowerExp UpperExp);
