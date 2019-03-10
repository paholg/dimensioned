use super::*;

pub fn new() -> System {
    System {
        name: "UCUM",
        module: "ucum",
        doc_prelude: "

The Unified Code for Units of Measure (UCUM)

This is an attempt to define a unit system for [The Unified Code for Units of
Measure](http://unitsofmeasure.org/ucum.html). It does not perfectly match the specification in the
ways mentioned below.

---

The UCUM specification uses NIST values from 1980 for experimentally determined constants. For some
of these, we use the 2014 values instead, which can be found
[here](http://physics.nist.gov/cuu/Constants/).

---

There are a few classifications of units that UCUM defines but that it does not make sense to
define here, and so we do not. They are as follows:

* Units defined in terms of characters that we can't use. For example, the
symbol for minutes as measure of angle is given by a single quote, '.

* Units that require conversions that involve more than multiplication. These include some
temperature units (such as degrees Celcius) and logrithmic units (such as decibels).

---

Conflicts:

While the UCUM specification aims to minimize conflicts, there are still some. Units that may
conflict are indicated by the UCUM spec by square brackets. When there is a conflict, for the unit
in square brackets, we name it with a trailing underscore. For example, the speed of light
conflicts with coulomb, so we use `C_` for the speed of light.

---

A note on derived units:

Mass is involved in a great many units. As the SI unit of mass is the kilogram, there are many
units defined in terms of tffhe kilogram. To create a derived unit in dimensioned, it must have a
value of 1.0. Since UCUM uses the gram and not the kilogram as a base unit, that leads to many
common derived units being off by a factor of 1000. As a result, you will see many derived units
with prefixes, such as `MilliNewton` and `KiloFarad`. The constants for the more common units (`N`
and `F` for the two mentioned) are still defined, but if you need to refer to their types (such as
in function signatures), this is something to bear in mind.

---
",
        base: base_units!(
            M:   Meter,    m,   Length;
            S:   Second,   s,   Time;
            G:   Gram,     g,   Mass;
            RAD: Radian,   rad;
            K:   Kelvin,   K,   Temperature;
            C:   Coulomb,  C,   Charge;
            CD:  Candela,  cd,  LuminousIntensity;
        ),
        derived: derived_units!(
            SR:       Steradian   = Radian * Radian;
            HZ:       Hertz       = Unitless / Second, Frequency;
            MILLIN:   MilliNewton = Gram * Meter / Second / Second, Force;
            MILLIPA:  MilliPascal = MilliNewton / Meter / Meter, Pressure;
            MILLIJ:   MilliJoule  = MilliNewton * Meter, Energy;
            MILLIW:   MilliWatt   = MilliJoule / Second, Power;
            A:        Ampere      = Coulomb / Second, Current;
            MILLIV:   MilliVolt   = MilliJoule / Coulomb, ElectricPotential;
            KILOF:    KiloFarad   = Coulomb / MilliVolt, Capacitance;
            MILLIOHM: MilliOhm    = MilliVolt / Ampere, Resistance;
            KILOSIE:  KiloSiemens = Unitless / MilliOhm, Conductance;
            MILLIWB:  MilliWeber  = MilliVolt * Second, MagneticFlux;
            MILLIT:   MilliTesla  = MilliWeber / Meter / Meter;
            MILLIH:   MilliHenry  = MilliWeber / Ampere, Inductance;
            LM:       Lumen       = Candela * Steradian;
            LX:       Lux         = Lumen / Meter / Meter;
            BQ:       Becquerel   = Unitless / Second;
            GY:       Gray        = MilliJoule / Gram;
            SV:       Sievert     = MilliJoule / Gram;

            // Miscellaneous useful type definitions:
            S2: Second2 = Second * Second;
            S3: Second3 = Second2 * Second;

            M2: Meter2 = Meter * Meter, Area;
            M3: Meter3 = Meter2 * Meter, Volume;

            PM: PerMeter = Unitless / Meter, ReciprocalLength;

            MPS: MeterPerSecond = Meter / Second, Velocity;
            MPS2: MeterPerSecond2 = Meter / Second2, Acceleration;
            MPS3: MeterPerSecond3 = Meter / Second3, Jerk;
            M2PS: Meter2PerSecond = Meter2 / Second;
            M2PS2: Meter2PerSecond2 = Meter2 / Second2;

            APM: AmperePerMeter = Ampere / Meter;
            CPM2: CandelaPerMeter2 = Candela / Meter2;
            CPG: CoulombPerGram = Coulomb / Gram;
            GPM: GramPerMeter = Gram / Meter;
            GPM4S: GramPerMeter4Second = Gram / Meter3 / Meter / Second;

            MILLIJS: MilliJouleSecond = MilliJoule * Second;
            MILLIJPK: MilliJoulePerKelvin = MilliJoule / Kelvin;
            KILOFPM: KiloFaradPerMeter = KiloFarad / Meter;
            MILLINPA2: MilliNewtonPerAmpere2 = MilliNewton / Ampere / Ampere;
            M3PGS2: Meter3PerGramSecond2 = Meter3 / Gram / Second2;
            MILLIPS: MilliPascalSecond = MilliPascal * Second;


            M3PSG: Meter3PerSecondGram = Meter3 / Second / Gram;
        ),
        constants: constants!(
            // Useful constants:
            KG: Gram = KILO*G.value_unsafe, "Kilogram";
            CM: Meter = CENTI*M.value_unsafe, "Centimeter";

            // SI Units continued (UCUM Section 30):
            MOL: Unitless = 6.0221367e23*ONE.value_unsafe, "Mole";
            N:   MilliNewton = KILO*MILLIN.value_unsafe, "Newton";
            PA:  MilliPascal = KILO*MILLIPA.value_unsafe, "Pascal";
            J:  MilliJoule = KILO*MILLIJ.value_unsafe, "Joule";
            W:  MilliWatt = KILO*MILLIW.value_unsafe, "Watt";
            V:  MilliVolt = KILO*MILLIV.value_unsafe, "Volt";
            F:  KiloFarad = MILLI*KILOF.value_unsafe, "Farad";
            OHM:  MilliOhm = KILO*MILLIOHM.value_unsafe, "Ohm";
            SIE:  KiloSiemens = MILLI*KILOSIE.value_unsafe, "Siemens";
            WB:  MilliWeber = KILO*MILLIWB.value_unsafe, "Weber";
            T:  MilliTesla = KILO*MILLIT.value_unsafe, "Tesla";
            H:  MilliHenry = KILO*MILLIH.value_unsafe, "Henry";

            // Units from ISO 1000, ISO 2955, and ANSI X3.50 (UCUM Section 31):
            GON: Radian = 0.9 * DEG.value_unsafe, "Gon, grade";
            DEG: Radian = 2.0 * consts::PI / 360.0 * RAD.value_unsafe, "Degree";
            L: Meter3 = 0.1 * 0.1 * 0.1 * M3.value_unsafe, "Liter";
            AR: Meter2 = 100.0 * M2.value_unsafe, "Are";
            MIN: Second = 60.0 * S.value_unsafe, "Minute";
            HR: Second = 60.0 * MIN.value_unsafe, "Hour";
            D: Second = 24.0 * HR.value_unsafe, "Day";
            ANN_T: Second = 365.24219 * D.value_unsafe, "Tropical year";
            ANN_J: Second = 365.25 * D.value_unsafe, "Mean Julian year";
            ANN_G: Second = 365.2425 * D.value_unsafe, "Mean Gregorian year";
            ANN: Second = ANN_J.value_unsafe, "year";
            WK: Second = 7.0 * D.value_unsafe, "week";
            MO_S: Second = 29.53059 * D.value_unsafe, "Synodal month";
            MO_J: Second = ANN_J.value_unsafe / 12.0, "Mean Julian month";
            MO_G: Second = ANN_G.value_unsafe / 12.0, "Mean Gregorian month";
            MO: Second = MO_J.value_unsafe, "Month";
            TNE: Gram = 1.0e3 * KG.value_unsafe, "Tonne";
            BAR: MilliPascal = 1.0e5 * PA.value_unsafe, "Bar";
            AMU: Gram = 1.6605402e-24*G.value_unsafe, "Unified atomic mass unit";
            EV: MilliJoule =  E.value_unsafe * V.value_unsafe, "Electronvolt";
            ASU: Meter = 149597.870691e6*M.value_unsafe, "Astronomic unit";
            PRS: Meter = 3.085678e16*M.value_unsafe, "Parsec";

            // Natural Units (UCUM Section 32):
            C_: MeterPerSecond = 299792458.0 * MPS.value_unsafe, "Speed of light in a vacuum";
            H_: MilliJouleSecond = 6.6260755e-34 * J.value_unsafe * S.value_unsafe, "Planck constant";
            K_: MilliJoulePerKelvin = 1.380658e-23 * J.value_unsafe / K.value_unsafe, "Boltzmann constant";
            EPS_0: KiloFaradPerMeter = 8.854187817e-12 * F.value_unsafe / M.value_unsafe, "Permittivity of vacuum";
            MU_0: MilliNewtonPerAmpere2 = 4.0e-7 * consts::PI * N.value_unsafe / A.value_unsafe / A.value_unsafe, "Permeability of vacuum";
            E: Coulomb = 1.6021766208e-19 * C.value_unsafe, "Elementary charge";
            M_E: Gram = 9.10938356e-31 * KG.value_unsafe, "Electron mass";
            M_P: Gram = 1.6726231e-24 * G.value_unsafe, "Proton mass";
            GC: Meter3PerGramSecond2 = 6.67259e-11 * M3.value_unsafe / KG.value_unsafe / S2.value_unsafe, "Newtonian constant of gravitation";
            G_: MeterPerSecond2 = 9.80665 * M.value_unsafe / S2.value_unsafe, "Standard acceleration of free fall";
            ATM: MilliPascal = 101325.0 * PA.value_unsafe, "Standard atmosphere";
            LY: Meter = C_.value_unsafe * ANN_J.value_unsafe, "Light-year";
            GF: MilliNewton = G.value_unsafe * G_.value_unsafe, "Gram force";
            LBF_AV: MilliNewton = LB_AV.value_unsafe * G_.value_unsafe, "Pound force";

            // CGS Units (UCUM Section 33):
            KY: PerMeter = 1.0 / CM.value_unsafe, "Kayser";
            GL: MeterPerSecond2 = CM.value_unsafe / S2.value_unsafe, "Gal";
            DYN: MilliNewton = G.value_unsafe * CM.value_unsafe / S2.value_unsafe, "Dyne";
            ERG: MilliJoule = DYN.value_unsafe * CM.value_unsafe, "Erg";
            P: MilliPascalSecond = DYN.value_unsafe * S.value_unsafe / CM.value_unsafe / CM.value_unsafe, "Poise";
            BI: Ampere = 10.0 * A.value_unsafe, "Biot";
            ST: Meter2PerSecond = CM.value_unsafe * CM.value_unsafe / S.value_unsafe, "Stokes";
            MX: MilliWeber = 1.0e-8 * WB.value_unsafe, "Maxwell";
            GS: MilliTesla = 1.0e-4 * T.value_unsafe, "Gauss";
            OE: AmperePerMeter = 250.0 / consts::PI * A.value_unsafe / M.value_unsafe, "Oersted";
            GB: Ampere = OE.value_unsafe * CM.value_unsafe, "Gilbert";
            SB: CandelaPerMeter2 = CD.value_unsafe / CM.value_unsafe / CM.value_unsafe, "Stilb";
            LMB: CandelaPerMeter2 = SB.value_unsafe / consts::PI, "Lambert";
            PHT: Lux = 1.0e-4 * LX.value_unsafe, "Phot";
            CI: Becquerel = 3.7e10 * BQ.value_unsafe, "Curie";
            ROE: CoulombPerGram = 2.58e-4 * C.value_unsafe / KG.value_unsafe, "Roentgen";
            RAD_: Meter2PerSecond2 = 100.0 * ERG.value_unsafe / G.value_unsafe, "Radiation absorbed dose";
            REM_: Meter2PerSecond2 = RAD_.value_unsafe, "Radiation equivalent man";

            // International Customary Units (UCUM Section 34):
            IN_I: Meter = 2.54 * CM.value_unsafe, "International inch";
            FT_I: Meter = 12.0 * IN_I.value_unsafe, "International foot";
            YD_I: Meter = 3.0 * FT_I.value_unsafe, "International yard";
            MI_I: Meter = 5280.0 * FT_I.value_unsafe, "International mile";
            FTH_I: Meter = 6.0 * FT_I.value_unsafe, "International fathom";
            NMI_I: Meter = 1852.0 * M.value_unsafe, "International nautical mile";
            KN_I: MeterPerSecond = NMI_I.value_unsafe / HR.value_unsafe, "International knot";
            SIN_I: Meter2 = IN_I.value_unsafe * IN_I.value_unsafe, "International square inch";
            SFT_I: Meter2 = FT_I.value_unsafe * FT_I.value_unsafe, "International square foot";
            SYD_I: Meter2 = YD_I.value_unsafe * YD_I.value_unsafe, "International square yard";
            CIN_I: Meter3 = IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe, "International cubic inch";
            CFT_I: Meter3 = FT_I.value_unsafe * FT_I.value_unsafe * FT_I.value_unsafe, "International cubic foot";
            CYD_I: Meter3 = YD_I.value_unsafe * YD_I.value_unsafe * YD_I.value_unsafe, "International cubic yard";
            BF_I: Meter3 = 144.0 * CIN_I.value_unsafe, "International board foot";
            CR_I: Meter3 = 128.0 * CFT_I.value_unsafe, "International cord";
            MIL_I: Meter = 1.0e-3 * IN_I.value_unsafe, "International mil";
            CML_I: Meter2 = consts::PI/4.0 * MIL_I.value_unsafe * MIL_I.value_unsafe, "International circular mil";
            HD_I: Meter = 4.0 * IN_I.value_unsafe, "International hand";

            // US Servey Lengths (UCUM Section 35):
            FT_US: Meter = 1200.0 / 3937.0 * M.value_unsafe, "US foot";
            YD_US: Meter = 3.0 * FT_US.value_unsafe, "US yard";
            IN_US: Meter = FT_US.value_unsafe / 12.0, "US inch";
            RD_US: Meter = 16.5 * FT_US.value_unsafe, "US rod";
            CH_US: Meter = 4.0 * RD_US.value_unsafe, "US Gunter's chain";
            LK_US: Meter = CH_US.value_unsafe / 100.0, "US Gunter's chain";
            RCH_US: Meter = 100.0 * FT_US.value_unsafe, "US Ramden's chain";
            RLK_US: Meter = RCH_US.value_unsafe / 100.0, "US Link for Ramden's chain";
            FTH_US: Meter = 6.0 * FT_US.value_unsafe, "US fathom";
            FUR_US: Meter = 40.0 * RD_US.value_unsafe, "US furlong";
            MI_US: Meter = 8.0 * FUR_US.value_unsafe, "US mile";
            ACR_US: Meter2 = 160.0 * RD_US.value_unsafe * RD_US.value_unsafe, "US acre";
            SRD_US: Meter2 = RD_US.value_unsafe * RD_US.value_unsafe, "US square rod";
            SMI_US: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe, "US square mile";
            SCT: Meter2 = MI_US.value_unsafe * MI_US.value_unsafe, "Section";
            TWP: Meter2 = 36.0 * SCT.value_unsafe, "Township";
            MIL_US: Meter = 1.0e-3 * IN_US.value_unsafe, "US mil";

            // British Imperial Lengths (UCUM Section 36):
            IN_BR: Meter = 2.539998 * CM.value_unsafe, "British inch";
            FT_BR: Meter = 12.0 * IN_BR.value_unsafe, "British foot";
            RD_BR: Meter = 16.5 * FT_BR.value_unsafe, "British rod";
            CH_BR: Meter = 4.0 * RD_BR.value_unsafe, "British Gunter's chain";
            LK_BR: Meter = CH_BR.value_unsafe / 100.0, "British link for Gunter's chain";
            FTH_BR: Meter = 6.0 * FT_BR.value_unsafe, "British fathom";
            PC_BR: Meter = 2.5 * FT_BR.value_unsafe, "British pace";
            YD_BR: Meter = 3.0 * FT_BR.value_unsafe, "British yard";
            MI_BR: Meter = 5280.0 * FT_BR.value_unsafe, "British mile";
            NMI_BR: Meter = 6080.0 * FT_BR.value_unsafe, "British nautical mile";
            KN_BR: MeterPerSecond = NMI_BR.value_unsafe / HR.value_unsafe, "British knot";
            ACR_BR: Meter2 = 4840.0 * YD_BR.value_unsafe * YD_BR.value_unsafe, "British acre";

            // US Volumes (UCUM Section 37):
            GAL_US: Meter3 = 231.0 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe, "Queen Anne's wine gallon";
            BBL_US: Meter3 = 42.0 * GAL_US.value_unsafe, "US barrel";
            QT_US: Meter3 = GAL_US.value_unsafe / 4.0, "US quart";
            PT_US: Meter3 = QT_US.value_unsafe / 2.0, "US pint";
            GIL_US: Meter3 = PT_US.value_unsafe / 4.0, "US gill";
            FOZ_US: Meter3 = GIL_US.value_unsafe / 4.0, "US fluid Ounce";
            FDR_US: Meter3 = FOZ_US.value_unsafe / 8.0, "US fluid Dram";
            MIN_US: Meter3 = FDR_US.value_unsafe / 60.0, "US minim";
            CRD_US: Meter3 = CR_I.value_unsafe, "US cord";
            BU_US: Meter3 = 2150.42 * IN_I.value_unsafe * IN_I.value_unsafe * IN_I.value_unsafe, "US bushel";
            GAL_WI: Meter3 = BU_US.value_unsafe / 8.0, "Historical winchester gallon";
            PK_US: Meter3 = BU_US.value_unsafe / 4.0, "US peck";
            DQT_US: Meter3 = PK_US.value_unsafe / 8.0, "US dry quart";
            DPT_US: Meter3 = DQT_US.value_unsafe / 2.0, "US dry pint";
            TBS_US: Meter3 = FOZ_US.value_unsafe / 2.0, "US tablespoon";
            TSP_US: Meter3 = TBS_US.value_unsafe / 3.0, "US teaspoon";
            CUP_US: Meter3 = 16.0 * TBS_US.value_unsafe, "US cup";
            FOZ_M: Meter3 = 30.0 * MILLI*L.value_unsafe, "Metric fluid ounce";
            CUP_M: Meter3 = 240.0 * MILLI*L.value_unsafe, "Metric cup";
            TSP_M: Meter3 = 5.0 * MILLI*L.value_unsafe, "Metric teaspoon";
            TBS_M: Meter3 = 15.0 * MILLI*L.value_unsafe, "Metric tablespoon";

            // British Imperial Volumes (UCUM Section 38):
            GAL_BR: Meter3 = 4.54609 * L.value_unsafe, "British gallon";
            PK_BR: Meter3 = 2.0 * GAL_BR.value_unsafe, "British peck";
            BU_BR: Meter3 = 4.0 * PK_BR.value_unsafe, "British bushel";
            QT_BR: Meter3 = GAL_BR.value_unsafe / 4.0, "British quart";
            PT_BR: Meter3 = QT_BR.value_unsafe / 2.0, "British pint";
            GIL_BR: Meter3 = PT_BR.value_unsafe / 4.0, "British gill";
            FOZ_BR: Meter3 = GIL_BR.value_unsafe / 5.0, "British fluid ounce";
            FDR_BR: Meter3 = FOZ_BR.value_unsafe / 8.0, "British fluid dram";
            MIN_BR: Meter3 = FDR_BR.value_unsafe / 60.0, "British minim";

            // Avoirdupois Weights (UCUM Section 39):
            GR: Gram = 64.79891 * MILLI*G.value_unsafe, "Grain";
            LB_AV: Gram = 7000.0 * GR.value_unsafe, "Avoirdupois pound";
            OZ_AV: Gram = LB_AV.value_unsafe / 16.0, "Avoirdupois ounce";
            DR_AV: Gram = OZ_AV.value_unsafe / 16.0, "Avoirdupois dram";
            SCWT_AV: Gram = 100.0 * LB_AV.value_unsafe, "Avoirdupois short hundredweight";
            LCWT_AV: Gram = 112.0 * LB_AV.value_unsafe, "Avoirdupois long hundredweight";
            STON_AV: Gram = 20.0 * SCWT_AV.value_unsafe, "Avoirdupois short ton";
            LTON_AV: Gram = 20.0 * LCWT_AV.value_unsafe, "Avoirdupois long ton";
            STONE_AV: Gram = 14.0 * LB_AV.value_unsafe, "Avoirdupois stone";

            // Troy Weights (UCUM Section 40):
            PWT_TR: Gram = 24.0 * GR.value_unsafe, "Troy pennyweight";
            OZ_TR: Gram = 20.0 * PWT_TR.value_unsafe, "Troy ounce";
            LB_TR: Gram = 12.0 * OZ_TR.value_unsafe, "Troy pound";

            // Apothecaries Weights (UCUM Section 41):
            SC_AP: Gram = 20.0 * GR.value_unsafe, "Apothecary scruple";
            DR_AP: Gram = 3.0 * SC_AP.value_unsafe, "Apothecary dram";
            OZ_AP: Gram = 8.0 * DR_AP.value_unsafe, "Apothecary ounce";
            LB_AP: Gram = 12.0 * OZ_AP.value_unsafe, "Apothecary pound";
            OZ_M: Gram = 28.0 * G.value_unsafe, "Apothecary ounce";

            // Typesetter's Lengths (UCUM Section 42):
            LNE: Meter = IN_I.value_unsafe / 12.0, "Line";
            PNT: Meter = LNE.value_unsafe / 6.0, "Point";
            PCA: Meter = 12.0 * PNT.value_unsafe, "Pica";
            PNT_PR: Meter = 0.013837 * IN_I.value_unsafe, "Printer's point";
            PCA_PR: Meter = 12.0 * PNT_PR.value_unsafe, "Printer's pica";
            PIED: Meter = 32.48 * CM.value_unsafe, "Pied";
            POUNCE: Meter = PIED.value_unsafe / 12.0, "Pounce";
            LIGNE: Meter = POUNCE.value_unsafe / 12.0, "Ligne";
            DIDOT: Meter = LIGNE.value_unsafe / 6.0, "Didot";
            CICERO: Meter = 12.0 * DIDOT.value_unsafe, "Cicero";

            // Legacy Units for Heat and Temperature (UCUM Section 43):
            DEGR: Kelvin = 5.0 / 9.0 * K.value_unsafe, "Degree Rankine";
            CAL_15: MilliJoule = 4.18580 * J.value_unsafe, "Calorie at 15 °C";
            CAL_20: MilliJoule = 4.18190 * J.value_unsafe, "Calorie at 20 °C";
            CAL_M: MilliJoule = 4.19002 * J.value_unsafe, "Mean calorie";
            CAL_IT: MilliJoule = 4.1868 * J.value_unsafe, "International table calorie";
            CAL_TH: MilliJoule = 4.184 * J.value_unsafe, "Thermochemical calorie";
            CAL: MilliJoule = CAL_TH.value_unsafe, "Calorie";
            CAL_: MilliJoule = KILO*CAL.value_unsafe, "Nutrition label calorie";
            BTU_39: MilliJoule = 1.05967 * KILO*J.value_unsafe, "British thermal unit at 39 °F";
            BTU_59: MilliJoule = 1.05480 * KILO*J.value_unsafe, "British thermal unit at 59 °F";
            BTU_60: MilliJoule = 1.05468 * KILO*J.value_unsafe, "British thermal unit at 60 °F";
            BTU_M: MilliJoule = 1.05587 * KILO*J.value_unsafe, "Mean British thermal unit";
            BTU_IT: MilliJoule = 1.05505585262 * KILO*J.value_unsafe, "International table British thermal unit";
            BTU_TH: MilliJoule = 1.054350 * KILO*J.value_unsafe, "Thermochemical British thermal unit";
            BTU: MilliJoule = BTU_TH.value_unsafe, "British thermal unit";
            HP: MilliWatt = 550.0 * FT_I.value_unsafe * LBF_AV.value_unsafe / S.value_unsafe, "Horsepower";
            TEX: GramPerMeter = 1.0 * G.value_unsafe / (KILO*M.value_unsafe), "Tex";
            DEN: GramPerMeter = TEX.value_unsafe / 9.0, "Denier";

            // Units used predominantly in clinical medicine (UCUM Section 44):
            MH2O: MilliPascal = 9.80665 * KILO*PA.value_unsafe, "Meter of water column";
            MHG: MilliPascal = 133.3220 * KILO*PA.value_unsafe, "Meter of mercury column";
            PRU: GramPerMeter4Second = MHG.value_unsafe * S.value_unsafe / L.value_unsafe, "Peripheral vascular resistance unit";
            DIOP: PerMeter = 1.0 / M.value_unsafe, "Diopter";
            MESH_I: PerMeter = 1.0 / IN_I.value_unsafe, "Mesh";
            CH: Meter = 1.0 / 3.0 * MILLI*M.value_unsafe, "Charrière";
            DRP: Meter3 = MILLI*L.value_unsafe / 20.0, "Drop";
            MET: Meter3PerSecondGram = 3.5 * MILLI*L.value_unsafe / MIN.value_unsafe / KG.value_unsafe, "Metabolic equivalent";

            // Chemical and Biochemical Units (UCUM Section 45):
            EQ: Unitless = MOL.value_unsafe, "Equivalents";
            OSM: Unitless = MOL.value_unsafe, "Osmole";
            S_: Second = 1.0e-13 * S.value_unsafe, "Svedberg unit";
            HPF: Unitless = 1.0 * ONE.value_unsafe, "High power field";
            LPF: Unitless = 100.0 * ONE.value_unsafe, "Low power field";
            KAT: Hertz = MOL.value_unsafe / S.value_unsafe, "Katal";
            U: Hertz = MICRO*MOL.value_unsafe / MIN.value_unsafe, "Unit";


            // Levels (UCUM Section 46):

            // Miscellaneous Units (UCUM Section 47):
            STR: Meter3 = 1.0 * M3.value_unsafe, "Stere";
            AO: Meter = 0.1 * NANO*M.value_unsafe, "Ångström";
            BRN: Meter2 = 100.0 * FEMTO*M.value_unsafe * FEMTO*M.value_unsafe, "Barn";
            ATT: MilliPascal = KILO*GF.value_unsafe / CM.value_unsafe / CM.value_unsafe, "Technical atmosphere";
            MHO: KiloSiemens = MILLI*SIE.value_unsafe, "Mho";
            PSI: MilliPascal = LBF_AV.value_unsafe / IN_I.value_unsafe / IN_I.value_unsafe, "Pound per square inch";
            CIRC: Radian = 2.0 * consts::PI * RAD.value_unsafe, "Circle";
            SPH: Steradian = 4.0 * consts::PI * SR.value_unsafe, "Sphere";
            CAR_M: Gram = 0.2 * G.value_unsafe, "Metric carat";
            CAR_AU: Unitless = 1.0 / 24.0 * ONE.value_unsafe, "Carat of gold alloys";
            SMOOT: Meter = 67.0 * IN_I.value_unsafe, "Smoot";
        ),
        fmt: true,
        from: vec!["SI"],
        refl_blacklist: vec!["RAD", "SR", "GON", "DEG", "CIRC", "LM", "SPH", "PHT", "LX"],
    }
}
