//! The `si` module provides a unit system for use with SI units. It was generated using
//! the `make_units!` macro. See its documentation for more information.
//!

#![allow(missing_docs)]

make_units! {
    UCUM;
    ONE: Unitless;

    base {
        M:   Meter,    "m",   P1;
        S:   Second,   "s",   P1;
        G:   Gram,     "g",   P1;
        RAD: Radian,   "rad", P1;
        K:   Kelvin,   "K",   P1;
        C:   Coulomb,  "C",   P1;
        CD:  Candela,  "cd",  P1;
    }

    derived {
        SR: Steradian = (Radian * Radian);
        HZ: Hertz = (Unitless / Second);
        PA: Pascal = (Newton / Meter / Meter);
        J:  Joule = (Newton * Meter);
        W:  Watt = (Joule / Second);
        A:  Ampere = (Coulomb / Second);
        V:  Volt = (Joule / Coulomb);
        F:  Farad = (Coulomb / Volt);
        OHM: Ohm = (Volt / Ampere);
        SIE: Siemens = (Unitless / Ohm);
        WB:  Weber = (Volt * Second);
        T: Tesla = (Weber / Meter / Meter);
        H: Henry = (Weber / Ampere);
        LM: Lumen = (Candela * Steradian);
        LX: Lux = (Lumen / Meter / Meter);
        BQ: Becquerel = (Unitless / Second);
        GY: Gray = (Joule / Kilogram);
        SV: Sievert = (Joule / Kilogram);

        MN:  MilliNewton = (Gram * Meter / Second / Second);

        M2: Meter2 = (Meter * Meter);
        M3: Meter3 = (Meter * Meter * Meter);
    }

    constants {
        MOL: Unitless = 6.0221367e23;
        N:   MilliNewton = 1000.0;
        CEL: Kelven = 1.0;
        GON: Radian = 0.9 * 2.0 * PI / 360.0;
        DEG: Radian = 2.0 * PI / 360.0;

        L: Meter3 = 0.001;
        AR: Meter2 = 100.0;
        MIN: Second = 60.0;
        HR: Second = 60.0 * 60.0;
        D: Second = 24.0 * 60.0 * 60.0;
        ANN_T: Second = 365.24219 * 24.0 * 60.0 * 60.0;
        ANN_J: Second = 365.25 * 24.0 * 60.0 * 60.0;
        ANN_G: Second = 365.2425 * 24.0 * 60.0 * 60.0;
        ANN: Second = 365.25 * 24.0 * 60.0 * 60.0;
        WK: Second = 7.0 * 24.0 * 60.0 * 60.0;
        MO_S: Second = 29.53059 * 24.0 * 60.0 * 60.0;
        MO_J: Second = 365.25 * 24.0 * 60.0 * 60.0 / 12.0;
        MO_G: Second = 365.2425 * 24.0 * 60.0 * 60.0 / 12.0;
        MO: Second = 365.25 * 24.0 * 60.0 * 60.0 / 12.0;

        TNE: Gram = 1.0e6;
        BAR: Pascal = 1.0e5;
        AMU: Gram = 1.6605402e-24;
        EV: Joule = 1.602176565e-19;
        ASU: Meter = 149597.870691e6;
        PRS: Meter = 3.085678e16;

        // Up to Natural units (section 32)
    }

    fmt = true;
}

pub use self::f64consts::*;
