//! The Unified Code for Units of Measure (UCUM)
//!
//! This is an attempt to define a unit system for [The Unified Code for Units of
//! Measure](http://unitsofmeasure.org/ucum.html). Note that there are a few limitations:
//!
//! We don't define: symbols not allowed ('), non-linear conversion (log and shit), no definition
//! provided (homeopathic stuff, etc.) fixme: make this betterer
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
        SR:       Steradian   = ( Radian * Radian                );
        HZ:       Hertz       = ( Unitless / Second              );
        MILLIN:   MilliNewton = ( Gram * Meter / Second / Second );
        MILLIPA:  MilliPascal = ( MilliNewton / Meter / Meter    );
        MILLIJ:   MilliJoule  = ( MilliNewton * Meter            );
        MILLIW:   MilliWatt   = ( MilliJoule / Second            );
        A:        Ampere      = ( Coulomb / Second               );
        MILLV:    MilliVolt   = ( MilliJoule / Coulomb           );
        KILOF:    KiloFarad   = ( Coulomb / MilliVolt            );
        MILLIOHM: MilliOhm    = ( MilliVolt / Ampere             );
        KILOSIE:  KiloSiemens = ( Unitless / MilliOhm            );
        MILLIWB:  MilliWeber  = ( MilliVolt * Second             );
        CEL:      Celsius     = ( Kelvin                         );
        MILLIT:   MilliTesla  = ( MilliWeber / Meter / Meter     );
        MILLIH:   MilliHenry  = ( MilliWeber / Ampere            );
        LM:       Lumen       = ( Candela * Steradian            );
        LX:       Lux         = ( Lumen / Meter / Meter          );
        BQ:       Becquerel   = ( Unitless / Second              );
        GY:       Gray        = ( MilliJoule / Gram              );
        SV:       Sievert     = ( MilliJoule / Gram);

        // Miscellaneous useful type definitions:
        S2: Second2 = (Second * Second);
        S3: Second3 = (Second2 * Second);

        M2: Meter2 = (Meter * Meter);
        M3: Meter3 = (Meter2 * Meter);

        PM: PerMeter = (Unitless / Meter);
        PS: PerSecond = (Unitless / Second);

        MPS: MeterPerSecond = (Meter / Second);
        MPS2: MeterPerSecond2 = (Meter / Second2);
        M2PS: Meter2PerSecond = (Meter2 / Second);
        M2PS2: Meter2PerSecond2 = (Meter2 / Second2);

        APM: AmperePerMeter = (Ampere / Meter);
        CPM2: CandelaPerMeter2 = (Candela / Meter2);
        CPG: CoulombPerGram = (Coulomb / Gram);
        GPM: GramPerMeter = (Gram / Meter);
        GPM4S: GramPerMeter4Second = (Gram / Meter3 / Meter / Second);

        MILLIJS: MilliJouleSecond = (MilliJoule * Second);
        MILLIJPK: MilliJoulePerKelvin = (MilliJoule / Kelvin);
        KILOFPM: KiloFaradPerMeter = (KiloFarad / Meter);
        MILLINPA2: MilliNewtonPerAmpere2 = (MilliNewton / Ampere / Ampere);
        M3PGS2: Meter3PerGramSecond2 = (Meter3 / Gram / Second2);
        MILLIPS: MilliPascalSecond = (MilliPascal * Second);

        M3PSG: Meter3PerSecondGram = (Meter3 / Second / Gram);
    }

    constants {
        // Useful constants:
        KG: Gram = KILO;
        CM: Meter = CENTI;

        // SI Units continued (UCUM Section 30):
        MOL: Unitless = 6.0221367e23;
        N:   MilliNewton = KILO;
        PA:  MilliPascal = KILO;
        J:  MilliJoule = KILO;
        W:  MilliWatt = KILO;
        V:  MilliVolt = KILO;
        F:  KiloFarad = MILLI;
        OHM:  MilliOhm = KILO;
        SIE:  KiloSiemens = MILLI;
        WB:  MilliWeber = KILO;
        T:  MilliTesla = KILO;
        H:  MilliHenry = KILO;

        // Units from ISO 1000, ISO 2955, and ANSI X3.50 (UCUM Section 31):
        GON: Radian = 0.9 * DEG.value_unsafe;
        DEG: Radian = 2.0 * PI / 360.0;
        L: Meter3 = 0.1 * 0.1 * 0.1;
        AR: Meter2 = 100.0;
        MIN: Second = 60.0;
        HR: Second = 60.0 * MIN.value_unsafe;
        D: Second = 24.0 * HR.value_unsafe;
        ANN_T: Second = 365.24219 * D.value_unsafe;
        ANN_J: Second = 365.25 * D.value_unsafe;
        ANN_G: Second = 365.2425 * D.value_unsafe;
        ANN: Second = ANN_J.value_unsafe;
        WK: Second = 7.0 * D.value_unsafe;
        MO_S: Second = 29.53059 * D.value_unsafe;
        MO_J: Second = ANN_J.value_unsafe / 12.0;
        MO_G: Second = ANN_G.value_unsafe / 12.0;
        MO: Second = MO_J.value_unsafe;
        TNE: Gram = 1.0e3 * KG.value_unsafe;
        BAR: MilliPascal = 1.0e5 * PA.value_unsafe;
        AMU: Gram = 1.6605402e-24;
        EV: MilliJoule =  E_.value_unsafe * V.value_unsafe;
        ASU: Meter = 149597.870691e6;
        PRS: Meter = 3.085678e16;

        // Natural Units (UCUM Section 32):
        C_: MeterPerSecond = 299792458.0;
        H_: MilliJouleSecond = 6.6260755e-34 * J.value_unsafe;
        K_: MilliJoulePerKelvin = 1.380658e-23 * J.value_unsafe;
        EPS_0: KiloFaradPerMeter = 8.854187817e-12 * F.value_unsafe;
        MU_0: MilliNewtonPerAmpere2 = 4.0e-7 * PI * N.value_unsafe;
        E_: Coulomb = 1.60217733e-19;
        M_E: Gram = 9.1093897e-28;
        M_P: Gram = 1.6726231e-24;
        GC: Meter3PerGramSecond2 = 6.67259e-11 / KG.value_unsafe;
        G_: MeterPerSecond2 = 9.80665;
        ATM: MilliPascal = 101325.0 * PA.value_unsafe;
        LY: Meter = C_.value_unsafe * ANN_J.value_unsafe;
        GF: MilliNewton = G_.value_unsafe;
        LBF_AV: MilliNewton = LB_AV.value_unsafe * G_.value_unsafe;

        // CGS Units (UCUM Section 33):
        KY: PerMeter = 1.0 / CM.value_unsafe;
        GL: MeterPerSecond2 = CM.value_unsafe;
        DYN: MilliNewton = CM.value_unsafe;
        ERG: MilliJoule = DYN.value_unsafe * CM.value_unsafe;
        P: MilliPascalSecond = DYN.value_unsafe / CM.value_unsafe / CM.value_unsafe;
        BI: Ampere = 10.0;
        ST: Meter2PerSecond = CM.value_unsafe * CM.value_unsafe;
        MX: MilliWeber = 1.0e-8 * WB.value_unsafe;
        GS: MilliTesla = 1.0e-4 * T.value_unsafe;
        OE: AmperePerMeter = 250.0 / PI;
        GB: Ampere = OE.value_unsafe * CM.value_unsafe;
        SB: CandelaPerMeter2 = CD.value_unsafe / CM.value_unsafe / CM.value_unsafe;
        LMB: CandelaPerMeter2 = SB.value_unsafe / PI;
        PHT: Lux = 1.0e-4;
        CI: Becquerel = 3.7e10;
        ROE: CoulombPerGram = 2.58e-4 * C.value_unsafe / KG.value_unsafe;
        RAD_: Meter2PerSecond2 = 100.0 * ERG.value_unsafe / G.value_unsafe;
        REM_: Meter2PerSecond2 = RAD_.value_unsafe;

        // International Customary Units (UCUM Section 34):
        IN_I: Meter = 2.54 * CM.value_unsafe;
        FT_I: Meter = 12.0 * IN_I.value_unsafe;
        YD_I: Meter = 3.0 * FT_I.value_unsafe;
        MI_I: Meter = 5280.0 * FT_I.value_unsafe;
        FTH_I: Meter = 6.0 * FT_I.value_unsafe;
        NMI_I: Meter = 1852.0;
        KN_I: MeterPerSecond = NMI_I.value_unsafe / HR.value_unsafe;
        SIN_I: Meter2 = IN_I.value_unsafe * IN_I.value_unsafe;
        SFT_I: Meter2 = FT_I.value_unsafe * FT_I.value_unsafe;
        SYD_I: Meter2 = YD_I.value_unsafe * YD_I.value_unsafe;
        CIN_I: Meter3 = IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
        CFT_I: Meter3 = FT_I.value_unsafe * FT_I.value_unsafe * FT_I.value_unsafe;
        CYD_I: Meter3 = YD_I.value_unsafe * YD_I.value_unsafe * YD_I.value_unsafe;
        BF_I: Meter3 = 144.0 * CIN_I.value_unsafe;
        CR_I: Meter3 = 128.0 * CFT_I.value_unsafe;
        MIL_I: Meter = 1.0e-3 * IN_I.value_unsafe;
        CML_I: Meter2 = PI/4.0 * MIL_I.value_unsafe * MIL_I.value_unsafe;
        HD_I: Meter = 4.0 * IN_I.value_unsafe;

        // US Servey Lengths (UCUM Section 35):
        FT_US: Meter = 1200.0 / 3937.0;
        YD_US: Meter = 3.0 * FT_US.value_unsafe;
        IN_US: Meter = FT_US.value_unsafe / 12.0;
        RD_US: Meter = 16.5 * FT_US.value_unsafe;
        CH_US: Meter = 4.0 * RD_US.value_unsafe;
        LK_US: Meter = CH_US.value_unsafe / 100.0;
        RCH_US: Meter = 100.0 * FT_US.value_unsafe;
        RLK_US: Meter = RCH_US.value_unsafe / 100.0;
        FTH_US: Meter = 6.0 * FT_US.value_unsafe;
        FUR_US: Meter = 40.0 * RD_US.value_unsafe;
        MI_US: Meter = 8.0 * FUR_US.value_unsafe;
        ACR_US: Meter2 = 160.0 * RD_US.value_unsafe * RD_US.value_unsafe;
        SRD_US: Meter2 = RD_US.value_unsafe * RD_US.value_unsafe;
        SMI_US: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe;
        SCT: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe;
        TWP: Meter2 = 36.0 * SCT.value_unsafe;
        MIL_US: Meter = 1.0e-3 * IN_US.value_unsafe;

        // British Imperial Lengths (UCUM Section 36):
        IN_BR: Meter = 2.539998 * CM.value_unsafe;
        FT_BR: Meter = 12.0 * IN_BR.value_unsafe;
        RD_BR: Meter = 16.5 * FT_BR.value_unsafe;
        CH_BR: Meter = 4.0 * RD_BR.value_unsafe;
        LK_BR: Meter = CH_BR.value_unsafe / 100.0;
        FTH_BR: Meter = 6.0 * FT_BR.value_unsafe;
        PC_BR: Meter = 2.5 * FT_BR.value_unsafe;
        YD_BR: Meter = 3.0 * FT_BR.value_unsafe;
        MI_BR: Meter = 5280.0 * FT_BR.value_unsafe;
        NMI_BR: Meter = 6080.0 * FT_BR.value_unsafe;
        KN_BR: MeterPerSecond = NMI_BR.value_unsafe / HR.value_unsafe;
        ACR_BR: Meter2 = 4840.0 * YD_BR.value_unsafe * YD_BR.value_unsafe;

        // US Volumes (UCUM Section 37):
        GAL_US: Meter3 = 231.0 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
        BBL_US: Meter3 = 42.0 * GAL_US.value_unsafe;
        QT_US: Meter3 = GAL_US.value_unsafe / 4.0;
        PT_US: Meter3 = QT_US.value_unsafe / 2.0;
        GIL_US: Meter3 = PT_US.value_unsafe / 4.0;
        FOZ_US: Meter3 = GIL_US.value_unsafe / 4.0;
        FDR_US: Meter3 = FOZ_US.value_unsafe / 8.0;
        MIN_US: Meter3 = FDR_US.value_unsafe / 60.0;
        CRD_US: Meter3 = CR_I.value_unsafe;
        BU_US: Meter3 = 2150.42 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe;
        GAL_WI: Meter3 = BU_US.value_unsafe / 8.0;
        PK_US: Meter3 = BU_US.value_unsafe / 4.0;
        DQT_US: Meter3 = PK_US.value_unsafe / 8.0;
        DPT_US: Meter3 = DQT_US.value_unsafe / 2.0;
        TBS_US: Meter3 = FOZ_US.value_unsafe / 2.0;
        TSP_US: Meter3 = TBS_US.value_unsafe / 3.0;
        CUP_US: Meter3 = 16.0 * TBS_US.value_unsafe;
        FOZ_M: Meter3 = 30.0 * MILLI*L.value_unsafe;
        CUP_M: Meter3 = 240.0 * MILLI*L.value_unsafe;
        TSP_M: Meter3 = 5.0 * MILLI*L.value_unsafe;
        TBS_M: Meter3 = 15.0 * MILLI*L.value_unsafe;

        // British Imperial Volumes (UCUM Section 38):
        GAL_BR: Meter3 = 4.54609 * L.value_unsafe;
        PK_BR: Meter3 = 2.0 * GAL_BR.value_unsafe;
        BU_BR: Meter3 = 4.0 * PK_BR.value_unsafe;
        QT_BR: Meter3 = GAL_BR.value_unsafe / 4.0;
        PT_BR: Meter3 = QT_BR.value_unsafe / 2.0;
        GIL_BR: Meter3 = PT_BR.value_unsafe / 4.0;
        FOZ_BR: Meter3 = GIL_BR.value_unsafe / 5.0;
        FDR_BR: Meter3 = FOZ_BR.value_unsafe / 8.0;
        MIN_BR: Meter3 = FDR_BR.value_unsafe / 60.0;

        // Avoirdupois Weights (UCUM Section 39):
        GR: Gram = 64.79891 * MILLI;
        LB_AV: Gram = 7000.0 * GR.value_unsafe;
        OZ_AV: Gram = LB_AV.value_unsafe / 16.0;
        DR_AV: Gram = OZ_AV.value_unsafe / 16.0;
        SCWT_AV: Gram = 100.0 * LB_AV.value_unsafe;
        LCWT_AV: Gram = 112.0 * LB_AV.value_unsafe;
        STON_AV: Gram = 20.0 * SCWT_AV.value_unsafe;
        LTON_AV: Gram = 20.0 * LCWT_AV.value_unsafe;
        STONE_AV: Gram = 14.0 * LB_AV.value_unsafe;

        // Troy Weights (UCUM Section 40):
        PWT_TR: Gram = 24.0 * GR.value_unsafe;
        OZ_TR: Gram = 20.0 * PWT_TR.value_unsafe;
        LB_TR: Gram = 12.0 * OZ_TR.value_unsafe;

        // Apothecaries Weights (UCUM Section 41):
        SC_AP: Gram = 20.0 * GR.value_unsafe;
        DR_AP: Gram = 3.0 * SC_AP.value_unsafe;
        OZ_AP: Gram = 8.0 * DR_AP.value_unsafe;
        LB_AP: Gram = 12.0 * OZ_AP.value_unsafe;
        OZ_M: Gram = 28.0;

        // Typesetter's Lengths (UCUM Section 42):
        LNE: Meter = IN_I.value_unsafe / 12.0;
        PNT: Meter = LNE.value_unsafe / 6.0;
        PCA: Meter = 12.0 * PNT.value_unsafe;
        PNT_PR: Meter = 0.013837 * IN_I.value_unsafe;
        PCA_PR: Meter = 12.0 * PNT_PR.value_unsafe;
        PIED: Meter = 32.48 * CM.value_unsafe;
        POUNCE: Meter = PIED.value_unsafe / 12.0;
        LIGNE: Meter = POUNCE.value_unsafe / 12.0;
        DIDOT: Meter = LIGNE.value_unsafe / 6.0;
        CICERO: Meter = 12.0 * DIDOT.value_unsafe;

        // Legacy Units for Heat and Temperature (UCUM Section 43):
        DEGF: Kelvin = 5.0 / 9.0;
        DEGR: Kelvin = 5.0 / 9.0;
        DEGRE: Kelvin = 4.0 / 5.0;
        CAL_15: MilliJoule = 4.18580 * J.value_unsafe;
        CAL_20: MilliJoule = 4.18190 * J.value_unsafe;
        CAL_M: MilliJoule = 4.19002 * J.value_unsafe;
        CAL_IT: MilliJoule = 4.1868 * J.value_unsafe;
        CAL_TH: MilliJoule = 4.184 * J.value_unsafe;
        CAL: MilliJoule = CAL_TH.value_unsafe;
        CAL_: MilliJoule = KILO*CAL.value_unsafe;
        BTU_39: MilliJoule = 1.05967 * KILO*J.value_unsafe;
        BTU_59: MilliJoule = 1.05480 * KILO*J.value_unsafe;
        BTU_60: MilliJoule = 1.05468 * KILO*J.value_unsafe;
        BTU_M: MilliJoule = 1.05587 * KILO*J.value_unsafe;
        BTU_IT: MilliJoule = 1.05505585262 * KILO*J.value_unsafe;
        BTU_TH: MilliJoule = 1.054350 * KILO*J.value_unsafe;
        BTU: MilliJoule = BTU_TH.value_unsafe;
        HP: MilliWatt = 550.0 * FT_I.value_unsafe * LBF_AV.value_unsafe / S.value_unsafe;
        TEX: GramPerMeter = 1.0 / KILO;
        DEN: GramPerMeter = TEX.value_unsafe / 9.0;

        // Units used predominantly in clinical medicine (UCUM Section 44):
        MH20: MilliPascal = 9.80665 * KILO*PA.value_unsafe;
        MHG: MilliPascal = 133.3220 * KILO*PA.value_unsafe;
        PRU: GramPerMeter4Second = MHG.value_unsafe * S.value_unsafe / L.value_unsafe;
        DIOP: PerMeter = 1.0;
        MESH_I: PerMeter = 1.0 / IN_I.value_unsafe;
        CH: Meter = 0.001 / 3.0;
        DRP: Meter3 = 0.001 * L.value_unsafe / 20.0;
        MET: Meter3PerSecondGram = 3.5 * MILLI*L.value_unsafe / MIN.value_unsafe / KG.value_unsafe;

        // Chemical and Biochemical Units (UCUM Section 45):
        EQ: Unitless = MOL.value_unsafe;
        OSM: Unitless = MOL.value_unsafe;
        S_: Second = 1.0e-13;
        HPF: Unitless = 1.0;
        LPF: Unitless = 100.0;
        KAT: PerSecond = MOL.value_unsafe;
        U: PerSecond = MICRO*MOL.value_unsafe / MIN.value_unsafe;

        // Levels (UCUM Section 46):

        // Miscellaneous Units (UCUM Section 47):
        STR: Meter3 = 1.0;
        AO: Meter = 0.1 * NANO;
        BRN: Meter2 = 100.0 * FEMTO * FEMTO;
        ATT: MilliPascal = KILO*GF.value_unsafe / CM.value_unsafe / CM.value_unsafe;
        MHO: KiloSiemens = MILLI;
        PSI: MilliPascal = LBF_AV.value_unsafe / IN_I.value_unsafe / IN_I.value_unsafe;
        CIRC: Radian = 2.0 * PI;
        SPH: Steradian = 4.0 * PI;
        CAR_M: Gram = 0.2;
        CAR_AU: Unitless = 1.0 / 24.0;
        SMOOT: Meter = 67.0 * IN_I.value_unsafe;
    }

    fmt = true;
}

pub use self::f64consts::*;


mod conversion {
    // Convert from SI
    use super::UCUM;
    use typenum::{Integer, Diff, Prod, Z0};
    use core::convert::From;
    use core::ops::{Mul, Sub};
    use si;
    use f64prefixes::*;

    impl<V, Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole> From<
            si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>>
        for UCUM<Prod<V, f64>, tarr![Meter, Diff<Second, Ampere>, Kilogram, Z0, Kelvin, Ampere, Candela]> where
        Meter: Integer, Kilogram: Integer, Second: Integer + Sub<Ampere>, Ampere: Integer, Kelvin: Integer, Candela: Integer, Mole: Integer,
        V: Mul<f64>,
    {
        fn from(other: si::SI<V, tarr![Meter, Kilogram, Second, Ampere, Kelvin, Candela, Mole]>) -> Self {
            let kgfac = KILO.powi(Kilogram::to_i32());
            let molfac = (super::MOL.value_unsafe).powi(Mole::to_i32());

            let fac = kgfac * molfac;

            UCUM::new( other.value_unsafe * fac )
        }
    }
}
