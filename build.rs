
#[derive(Debug)]
struct BaseUnit {
    pub name: &'static str,
    pub constant: &'static str,
    pub token: &'static str,
    pub root: &'static str,
}

#[derive(Debug)]
struct DerivedUnit {
    pub name: &'static str,
    pub constant: &'static str,
    pub expression: &'static str,
}

#[derive(Debug)]
struct Constant {
    pub constant: &'static str,
    pub unit: &'static str,
    pub value: &'static str,
}

#[derive(Debug)]
struct System {
    pub module: &'static str,
    pub name: &'static str,
    pub doc_prelude: &'static str,
    pub base: Vec<BaseUnit>,
    pub derived: Vec<DerivedUnit>,
    pub constants: Vec<Constant>,
}

use std::fmt;

impl fmt::Display for System{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
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
}}")?;
        Ok(())
    }
}

macro_rules! base_units {
    ($($constant:ident: $unit:ident, $token:ident, $root:ident;)* ) => (
        vec![$(BaseUnit{name: stringify!($unit), constant: stringify!($constant), token: stringify!($token), root: stringify!($root)}),*];
    );
}

macro_rules! derived_units {
    ($($constant:ident: $unit:ident =  $e:expr;)* ) => (
        vec![$(DerivedUnit{name: stringify!($unit), constant: stringify!($constant), expression: stringify!($e)}),*];
    );
}

macro_rules! constants {
    ($($constant:ident: $unit:ident =  $e:expr;)* ) => (
        vec![$(Constant{unit: stringify!($unit), constant: stringify!($constant), value: stringify!($e)}),*];
    );
}

fn si() -> System {
    System {
        name: "SI", module: "si",
        doc_prelude: "this is si",
        base: base_units!(
            M: Meter, m, P1;
            KG: Kilogram, kg, P1;
            S: Second, s, P1;
            A: Ampere, A, P1;
            K: Kelvin, K, P1;
            CD: Candela, cd, P1;
            MOL: Mole, mol, P1;
        ),
        derived: derived_units!(
            HZ: Hertz = Unitless / Second;
            N: Newton = Kilogram * Meter / Second2;
            PA: Pascal = Newton / Meter2;
            J: Joule = Newton * Meter;
            W: Watt = Joule / Second;
            C: Coulomb = Second * Ampere;
            V: Volt = Watt / Ampere;
            F: Farad = Coulomb / Volt;
            OHM: Ohm = Volt / Ampere;
            SIEMENS: Siemens = Ampere / Volt;
            WB: Weber = Joule / Ampere;
            T: Tesla = Weber / Meter2;
            H: Henry = Ohm * Second;
            LM: Lumen = Candela;
            LX: Lux = Candela / Meter2;
            BQ: Becquerel = Hertz;
            GY: Gray = Joule / Kilogram;
            SV: Sievert = Gray;
            KAT: Katal = Mole / Second;

            M2: Meter2 = Meter * Meter;
            M3: Meter3 = Meter2 * Meter;

            PM: PerMeter = Unitless / Meter;
            PM2: PerMeter2 = PerMeter / Meter;
            PM3: PerMeter3 = PerMeter2 / Meter;

            S2: Second2 = Second * Second;
            S3: Second3 = Second2 * Second;
            S4: Second4 = Second3 * Second;

            MPS: MeterPerSecond = Meter / Second;
            MPS2: MeterPerSecond2 = Meter / Second2;
            MPS3: MeterPerSecond3 = Meter / Second3;
            MPS4: MeterPerSecond4 = Meter / Second4;

            M2PS: Meter2PerSecond = Meter2 / Second;
            M2PS2: Meter2PerSecond2 = Meter2 / Second2;
            M2PS3: Meter2PerSecond3 = Meter2 / Second3;

            M3PS: Meter3PerSecond = Meter3 / Second;
            M3PS2: Meter3PerSecond2 = Meter3 / Second2;
            M3PS3: Meter3PerSecond3 = Meter3 / Second3;

            NS: NewtonSecond = Newton * Second;
            JS: JouleSecond = Joule * Second;
            NPS: NewtonPerSecond = Newton / Second;
        ),
        constants: constants!(
            MIN: Second = 60.0;
            HR: Second = 60.0*60.0;
            DAY: Second = 24.0*60.0*60.0;
        )
    }

}

fn main() {

    let out_dir = std::env::var("OUT_DIR").unwrap();

    let dest = std::path::Path::new(&out_dir).join("si.rs");
    let mut f = std::fs::File::create(&dest).unwrap();

    use std::io::Write;
    write!(f, "{}", si()).unwrap();
}
