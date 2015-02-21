#!/usr/bin/env python3

class Units:
    def __init__(self):
        self.name = "Units"
        self.filename = "units.rs"
        self.units = [] # List of names for the units in the type
        self.print_as = [] # Corresponding list of symbols to use for printing
        self.constants = [] # Corresponding list of constants to define
        self.vtype = "f64" # type to define constants for
        self.one = "1.0" # value to set constants to
        self.unitless = "" # name for the dimensionless constant 1
        self.allowed_root = 1 # allowed roots to take. Useful for Gaussian units and the
                              # like. If allowed_root == 2, then you can take sqrt() of
                              # units, etc.
        self.extra_constants = [] # list of tuples (NAME, EXPR) where NAME is the name
                                  # to give the constant and EXPR the expression (in
                                  # Rust) to assign it to. <THESE ARE NOT YET IMPLEMENTED>
    def make_units(self):
        if len(self.units) != len(self.print_as) or len(self.units) != len(self.constants):
            print("The lists of units, print_as, and constants must all be the same length.")
            exit(1)
        name = self.name
        ushort = ", ".join(self.units)
        ulong = ", ".join([u + ": Peano" for u in self.units])
        u1_list = [u + "1" for u in self.units]
        u2_list = [u + "2" for u in self.units]
        uboth = ", ".join(u1_list + u2_list)
        u1 = ", ".join(u1_list)
        u2 = ", ".join(u2_list)
        # ----------------------------------------------------------------------
        # File top
        text = """
// This is a generated file. It was created using unitsmaker.py.
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use dimensioned::*;

pub struct {name}<{ulong}>{{
""".format(**locals())
        for unit in self.units:
            text += "  _marker{unit}: PhantomData<{unit}>,".format(**locals())
        text +="""
}}
impl<{ulong}> Dimension for {name}<{ushort}> {{}}

""".format(**locals())
        # ----------------------------------------------------------------------
        # Operators
        for op in ["Keep", "Add", "Sub"]:
            u1_long = ", ".join([u1+": Peano + {}Peano<{}>".format(op, u2) for (u1, u2) in zip(u1_list, u2_list) ])
            u2_long = ", ".join([u+": Peano" for u in u2_list])
            outs = ", ".join(["<{} as {}Peano<{}>>::Output".format(u1, op, u2) for (u1, u2) in zip(u1_list, u2_list)])
            text += """
impl<{uboth}> {op}Dim<{name}<{u2}>> for {name}<{u1}>
where {u1_long}, {u2_long}
{{
  type Output = {name}<{outs}>;
}}
""".format(**locals())
        # ----------------------------------------------------------------------
        # Operators part two (Mul and Div)
        for op in ["Mul", "Div"]:
            ulonghere = ", ".join([u+": Peano + {}Peano<RHS>".format(op) for u in self.units])
            outs = ", ".join(["<{} as {}Peano<RHS>>::Output".format(u, op) for u in self.units])
            text += """
impl<{ushort}, RHS> {op}Dim<RHS> for {name}<{ushort}>
where {ulonghere}, RHS: Peano
{{
  type Output = {name}<{outs}>;
}}
""".format(**locals())
        # ----------------------------------------------------------------------
        # ToString
        utoint = ", ".join([u + ": ToInt" for u in self.units])
        text += """
impl<{ushort}> DimToString for {name}<{ushort}>
  where {utoint} {{
    fn to_string() -> String {{
""".format(**locals())
        allowed_root = self.allowed_root
        for (unit, prn) in zip(self.units, self.print_as):
            text += """
      let {prn}_str = match <{unit} as ToInt>::to_int() {{
            0 => ("", "".to_string()),
            {allowed_root} => ("{prn}", "".to_string()),
            n => ("{prn}^", (n/{allowed_root}).to_string())
          }};""".format(**locals())

        text += """
      format!(\""""
        for i in range(len(self.units)):
            text += "{}{}"
        text += "\", "
        text += ", ".join(["{0}_str.0, {0}_str.1".format(p) for p in self.print_as])
        text +=""")
  }
}
"""
        # ----------------------------------------------------------------------
        # Type aliases
        root_num = "Zero"
        for i in range(self.allowed_root):
            root_num = "Succ<" + root_num + ">"

        type_sig = ", ".join(["Zero" for u in self.units])
        text += "pub type Unitless = {name}<{type_sig}>;\n".format(**locals())
        text += "impl Dimensionless for Unitless {}\n"
        for i, u in enumerate(self.units):
            type_sig = []
            for j in range(i):
                type_sig += ["Zero"]
            type_sig += [root_num]
            for j in range(i+1, len(self.units)):
                type_sig += ["Zero"]
            type_sig = ", ".join(type_sig)
            text += "pub type {u} = {name}<{type_sig}>;\n".format(**locals())

        # ----------------------------------------------------------------------
        # Constants
        vtype = self.vtype
        one = self.one
        text += "\n"
        text += "pub const {}: Dim<Unitless, {}> = Dim({}, PhantomData);\n".format(self.unitless, vtype, one)
        for (c, u) in zip(self.constants, self.units):
            text += "pub const {c}: Dim<{u}, {vtype}> = Dim({one}, PhantomData);\n".format(**locals())
        # # ----------------------------------------------------------------------
        # # Extra Constants
        # text += "\npub trait {name}Extra {{\n".format(**locals())
        # for (n, e) in self.extra_constants:
        #     text += "  fn {n}(self) -> Self;\n".format(**locals())
        # text += "}\n\n"
        # text += "impl {name}Extra for {vtype} {{\n".format(**locals())
        # for (n, e) in self.extra_constants:
        #     text += "  fn {n}(self) -> Self {{ {e}*self }}\n".format(**locals())
        # text += "}\n\n"

        # ----------------------------------------------------------------------
        # Save file!
        f = open(self.filename, "w")
        f.write(text)
        f.close()


def main():
    si = Units()
    si.name = "SI"
    si.filename = "src/si.rs"
    si.unitless = "one_si"
    si.units = ["Meter", "Kilogram", "Second", "Amp", "Kelvin", "Candela", "Mole"]
    si.constants = ["meter", "kilogram", "second", "amp", "kelvin", "candela", "mole"]
    si.print_as = ["m", "kg", "s", "A", "K", "cd", "mol"]

    # not these don't do anything yet
    si.extra_constants = [
        ("hertz", "one/second"),
        ("newton", "kilogram*meter/second/second"),
        ("pascal", "kilogram/meter/second/second"),
        ("joule", "kilogram*meter*meter/second/second"),
        ("watt", "kilogram*meter*meter/second/second/second"),
        ("coulomb", "second*amp"),
        ("volt", "kilogram*meter*meter/second/second/second/amp"),
        ("farad", "amp*amp*second*second*second*second/kilogram/meter/meter"),
        ("ohm", "kilogram*meter*meter/second/second/second/amp/amp/amp"),
        ("siemens", "amp*amp*second*second*second/kilogram/meter/meter"),
        ("weber", "kilogram*meter*meter/second/second/amp"),
        ("tesla", "kilogram/second/second/amp"),
        ("henry", "kilogram*meter*meter/second/second/amp/amp"),
        ("lumen", "candela"),
        ("lux", "candela/meter/meter"),
        ("becquerel", "one/second"),
        ("gray", "meter*meter/second/second"),
        ("sievert", "meter*meter/second/second"),
        ("katal", "mole/second")
    ]
    si.make_units()

    cgs = Units()
    cgs.name = "CGS"
    cgs.filename = "src/cgs.rs"
    cgs.unitless = "one_cgs"
    cgs.units = ["Centimeter", "Gram", "Second"]
    cgs.constants = ["centimeter", "gram", "second"]
    cgs.print_as = ["cm", "g", "s"]

    # fixme: incomplete
    cgs.extra_constants = [
        ("galileo", "centimeter/second/second"),
        ("dyne", "centimeter*gram/second/second"),
        ("barye", "gram/centimeter/second/second"),
        ("erg", "centimeter*centimeter/gram/second/second"),
        ("poise", "gram/centimeter/second"),
        ("stokes", "centimeter*centimeter/second"),
        ("kayser", "one/centimeter"),
        ("statcoulomb", "(centimeter*centimeter*centimeter*gram/second/second).sqrt()"),
        ("statohm", "second/centimeter"),
        ("statmho", "centimeter/second"),
        ("statfarad", "centimeter")
    ]
    cgs.allowed_root = 2
    cgs.make_units()

    u = Units()
    u.name = "U"
    u.filename = "src/u.rs"
    u.unitless = "one_u"
    u.units = ["Unit"]
    u.constants = ["unit"]
    u.print_as = ["u"]
    u.make_units()



if __name__ == "__main__":
    main()
