//! The Unified Code for Units of Measure (UCUM)
//!
//! This is an attempt to define a unit system for [The Unified Code for Units of
//! Measure](http://unitsofmeasure.org/ucum.html). Note that there are a few limitations:
//!
//! 1. Some units defined by UCUM have symbols outside those allowed for Rust constants. For
//! example, minutes and seconds for angles have the tokens ' and '', respectively. We opt to not
//! define these units.
//!
//! 2. For all unit systems in dimensioned, we define units as derived units wherever
//! possible. Currently, that is limited to those with a value of 1.0 in the appropriate
//! combination of the system's base units. Otherwise, we would end up with some inconsistencies
//! with the `new` function. For example, we don't want `Foot::new(1.0)` to do the same thing as
//! `Meter::new(1.0)`, which it would. This is especially relevent for UCUM as it has gram and not
//! kilogram as a base unit, so, for example, Newton cannot be a derived unit and so cannot be
//! used, for example, in a function signature. This becomes an issue of any unit having to do with
//! energy, so we define many derived units with `Milli` in front. For example, `MilliNewton` with
//! constant `MILLIN` and `MilliJoule` with constant `MILLIJ`. Note that the constants `N` and `J`
//! are still defined with the appropriate values.
//!
//!
//! This module was generated using the `make_units!` macro. See its documentation for more information.

#![allow(missing_docs)]

make_units! {
    UCUM;
    ONE: Unitless;

    base {
        // Base Units (UCUM Section 28):
        M:   Meter,    "m",   P1;
        S:   Second,   "s",   P1;
        G:   Gram,     "g",   P1;
        RAD: Radian,   "rad", P1;
        K:   Kelvin,   "K",   P1;
        C:   Coulomb,  "C",   P1;
        CD:  Candela,  "cd",  P1;
    }

    derived {
        // SI Units (UCUM Section 30):

        SR: Steradian = (Radian * Radian);
        HZ: Hertz = (Unitless / Second);
        MILLIN:  MilliNewton = (Gram * Meter / Second / Second);
        MILLIPA: MilliPascal = (MilliNewton / Meter / Meter);
        MILLIJ:  MilliJoule = (MilliNewton * Meter);
        MILLIW:  MilliWatt = (MilliJoule / Second);
        A:  Ampere = (Coulomb / Second);
        MILLV:  MilliVolt = (MilliJoule / Coulomb);
        KILOF:  KiloFarad = (Coulomb / MilliVolt);
        MILLIOHM: MilliOhm = (MilliVolt / Ampere);
        KILOSIE: KiloSiemens = (Unitless / MilliOhm);
        MILLIWB:  MilliWeber = (MilliVolt * Second);
        CEL: Celsius = (Kelvin);
        MILLIT: MilliTesla = (MilliWeber / Meter / Meter);
        MILLIH: MilliHenry = (MilliWeber / Ampere);
        LM: Lumen = (Candela * Steradian);
        LX: Lux = (Lumen / Meter / Meter);
        BQ: Becquerel = (Unitless / Second);
        GY: Gray = (MilliJoule / Gram);
        SV: Sievert = (MilliJoule / Gram);

        // Miscellaneous useful type definitions:

        S2: Second2 = (Second * Second);
        S3: Second3 = (Second2 * Second);

        M2: Meter2 = (Meter * Meter);
        M3: Meter3 = (Meter2 * Meter);

        IM: InverseMeter = (Unitless/Meter);

        MPS: MeterPerSecond = (Meter / Second);
        MPS2: MeterPerSecond2 = (Meter / Second2);
        M2PS: Meter2PerSecond = (Meter2 / Second);

        APM: AmperePerMeter = (Ampere / Meter);

        // MILLIJS: MilliJouleSecond = (MilliJoule * Second);
        // MILLIJPK: MilliJoulePerKelvin = (MilliJoule / Kelvin);
        // KILOFPM: KiloFaradPerMeter = (KiloFarad / Meter);
        // MILLINPA2: MilliNewtonPerAmpere2 = (MilliNewton / Ampere / Ampere);
        // M3PGS2: Meter3PerGramSecond2 = (Meter3 / Gram / Second2);
        MILLIPS: MilliPascalSecond = (MilliPascal * Second);
    }

    constants {
        // SI Units continued (UCUM Section 30):

        MOL: Unitless = 6.0221367e23;
        N:   MilliNewton = 1000.0;
        PA:  MilliPascal = 1000.0;
        J:  MilliJoule = 1000.0;
        W:  MilliWatt = 1000.0;
        V:  MilliVolt = 1000.0;
        F:  KiloFarad = 0.001;
        OHM:  MilliOhm = 1000.0;
        SIE:  KiloSiemens = 0.001;
        WB:  MilliWeber = 1000.0;
        T:  MilliTesla = 1000.0;
        H:  MilliHenry = 1000.0;

        // Units from ISO 1000, ISO 2955, and ANSI X3.50 (UCUM Section 31):

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
        BAR: MilliPascal = 1.0e8;
        AMU: Gram = 1.6605402e-24;
        EV: MilliJoule = 1.602176565e-16;
        ASU: Meter = 149597.870691e6;
        PRS: Meter = 3.085678e16;

        // Natural Units (UCUM Section 32):

        // C: MeterPerSecond = 299792458.0;
        // H: MilliJouleSecond = 6.6260755e-31;
        // K: MilliJoulePerKelvin = 1.380658e-20;
        // EPS_0: KiloFaradPerMeter = 8.854187817e-15;
        // MU_0: MilliNewtonPerAmpere2 = PI * 4.0e-4;
        // E: Coulomb = 1.60217733e-19;
        // M_E: Gram = 9.1093897e-28;
        // M_P: Gram = 1.6726231e-24;
        // GC: Meter3PerGramSecond2 = 6.67259e-14;
        // G: MeterPerSecond2 = 9.80665;
        // ATM: MilliPascal = 101325000.0;
        // LY: Meter = 299792458.0 * 365.25 * 24.0 * 60.0 * 60.0;
        // GF: MilliNewton = 9.80665;
        // LBF_AV: MilliNewton = 4448.22;

        // CGS Units (UCUM Section 33):

        KY: InverseMeter = 100.0;
        GL: MeterPerSecond2 = 0.01;
        DYN: MilliNewton = 0.01;
        ERG: MilliJoule = 1.0;
        P: MilliPascalSecond = 100.0;
        BI: Ampere = 10.0;
        ST: Meter2PerSecond = 1.0e-4;
        MX: MilliWeber = 1.0e-5;
        GS: MilliTesla = 0.01;
        OE: AmperePerMeter = 250.0 / PI;
        // GB:

        // International Customary Units (UCUM Section 34):

        // US Servey Lengths (UCUM Section 35):

        // British Imperial Lengths (UCUM Section 36):

        // US Volumes (UCUM Section 37):

        // British Imperial Volumes (UCUM Section 38):

        // Avoirdupois Weights (UCUM Section 39):

        // Troy Weights (UCUM Section 40):

        // Apothecaries Weights (UCUM Section 41):

        // Typesetter's Lengths (UCUM Section 42):

        // Legacy Units for Heat and Temperature (UCUM Section 43):

        // Units used predominantly in clinical medicine (UCUM Section 44):

        // Chemical and Biochemical Units (UCUM Section 45):

        // Levels (UCUM Section 46):

        // Miscellaneous Units (UCUM Section 47):

    }

    fmt = true;
}

pub use self::f64consts::*;
