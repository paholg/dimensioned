/// The purpose of this build script is two-fold.
///
/// First, it lets us make unit systems with nice, matching documentation that is guaranteed to be
/// correct.
///
/// Second, it lets us generate some tests that we could not do otherwide. For example, if we
/// create a constant `MIN` that has unit `Second` and value `60.0 * S.value_unsafe`, we can
/// generate a test to ensure that `MIN == 60.0 * S`. This would not be necessary if we could
/// perform these operations at compile time.

use std::fmt;

#[derive(Debug)]
pub struct BaseUnit {
    pub name: &'static str,
    pub constant: &'static str,
    pub token: &'static str,
    pub root: &'static str,
    pub dim: &'static str,
}

#[derive(Debug)]
pub struct DerivedUnit {
    pub name: &'static str,
    pub constant: &'static str,
    pub expression: &'static str,
    pub dim: &'static str,
}

#[derive(Debug)]
pub struct Constant {
    pub constant: &'static str,
    pub unit: &'static str,
    pub value: &'static str,
}

impl Constant {
    fn nice_value(&self) -> String {
        self.value.split(".value_unsafe").collect()
    }
}

#[derive(Debug)]
pub struct System {
    pub module: &'static str,
    pub name: &'static str,
    pub doc_prelude: &'static str,
    pub base: Vec<BaseUnit>,
    pub derived: Vec<DerivedUnit>,
    pub constants: Vec<Constant>,
}

impl System {
    fn unit_tables(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {

        write!(f, "

Following, we list all of the [base units](#base-units), [derived units](#derived-units), and
[constants](#constants) that are defined in this unit system.

")?;


        write!(f, "# Base Units\n")?;
        write!(f, "Constant | Unit | Dimension | Print Token | Allowed Root\n")?;
        write!(f, "---|---|---|---|---\n")?;
        for b in &self.base {
            write!(f, "{} | {} | {} | {} | {}\n", b.constant, b.name, b.dim, b.token, b.root)?;
        }

        write!(f, "# Derived Units\n")?;
        write!(f, "Constant | Unit | Unit Definition | Dimension\n")?;
        write!(f, "---|---|---|---\n")?;
        for d in &self.derived {
            write!(f, "{} | {} | {} | {}\n", d.constant, d.name, d.expression, d.dim)?;
        }

        write!(f, "# Constants\n")?;
        write!(f, "Constant | Value | Unit | Dimension\n")?;
        write!(f, "---|---|---|---\n")?;
        for b in &self.base {
            let mut newline = false;
            for c in self.constants.iter().filter(|c| c.unit == b.name) {
                write!(f, "{} | {} | {} | {}\n", c.constant, c.nice_value(), c.unit, b.dim)?;
                newline = true;
            }
            if newline {
                write!(f, "|\n")?;
            }
        }

        for d in &self.derived {
            let mut newline = false;
            for c in self.constants.iter().filter(|c| c.unit == d.name) {
                write!(f, "{} | {} | {} | {}\n", c.constant, c.nice_value(), c.unit, d.dim)?;
                newline = true;
            }
            if newline {
                write!(f, "|\n")?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "/**\n{}\n\n", self.doc_prelude)?;

        self.unit_tables(f)?;

        write!(f, "*/")?;

        write!(f, "
pub mod {} {{
    make_units! {{
        {};
        ONE: Unitless;

        base {{\n", self.module, self.name)?;
        for unit in &self.base {
            write!(f, "            {}: {}, \"{}\", {};\n", unit.constant, unit.name, unit.token, unit.root)?;
        }

        write!(f, "        }}

        derived {{\n")?;

        for unit in &self.derived {
            write!(f, "            {}: {} = ({});\n", unit.constant, unit.name, unit.expression)?;
        }

        write!(f, "        }}

        constants {{\n")?;

        for c in &self.constants {
            write!(f, "            {}: {} = {};\n", c.constant, c.unit, c.value)?;
        }

        write!(f, "        }}

        fmt = true;
    }}

    pub use self::f64consts::*;

")?;

        for u in &self.base {
            if u.dim.len() > 0 {
                write!(f, "
    impl<T> ::dimensions::{} for {}<T> {{}}\n", u.dim, u.name)?;
            }
        }

        for u in &self.derived {
            if u.dim.len() > 0 {
                write!(f, "
    impl<T> ::dimensions::{} for {}<T> {{}}\n", u.dim, u.name)?;
            }
        }

        write!(f, "
    #[test]
    fn test_{}_constants() {{
        #[allow(unused_imports)]
        use f64prefixes::*;
        #[allow(unused_imports)]
        use core::f64::consts::*;
", self.module)?;

        for c in &self.constants {
            write!(f, "
        assert_eq!({}, {});", c.constant, c.nice_value())?;
        }
        write!(f, "
    }}
}}")?;
        Ok(())
    }
}

macro_rules! base_units {
    ($($constant:ident: $unit:ident, $token:ident, $root:ident $(, $dim:ident)*;)*) => (
        vec![$(BaseUnit{name: stringify!($unit), constant: stringify!($constant), token: stringify!($token), root: stringify!($root), dim: stringify!($($dim)*)}),*];
    );
}

macro_rules! derived_units {
    ($($constant:ident: $unit:ident =  $e:expr $(, $dim:ident)*;)* ) => (
        vec![$(DerivedUnit{name: stringify!($unit), constant: stringify!($constant), expression: stringify!($e), dim: stringify!($($dim)*)}),*];
    );
}

macro_rules! constants {
    ($($constant:ident: $unit:ident =  $e:expr;)* ) => (
        vec![$(Constant{unit: stringify!($unit), constant: stringify!($constant), value: stringify!($e)}),*];
    );
}

fn make_system(s: &System) {
    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dest = std::path::Path::new(&out_dir).join(format!("{}.rs", s.module));
    let mut f = std::fs::File::create(&dest).unwrap();

    use std::io::Write;
    write!(f, "{}", s).unwrap();
}

mod si;
mod ucum;
mod mks;
mod cgs;

fn main() {
    let systems = [si::new(), ucum::new(), mks::new(), cgs::new()];
    for s in &systems {
        make_system(s);
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dest = std::path::Path::new(&out_dir).join("unit_systems.rs");
    let mut f = std::fs::File::create(&dest).unwrap();

    use std::io::Write;
    write!(f, "
/// The unit systems with which dimensioned ships

pub mod unit_systems {{").unwrap();

    for s in &systems {
        write!(f, "
    include!(concat!(env!(\"OUT_DIR\"), \"/{}.rs\"));", s.module).unwrap();
    }

    write!(f, "
}}").unwrap();
}
