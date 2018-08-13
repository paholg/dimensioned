use super::*;

pub fn new() -> System {
    System {
        name: "SI",
        module: "si",
        doc_prelude: "The International System of Units (SI)

Where experimental values are used for constants, the values are obtained from the 2014 values from
[The NIST Reference on Constants, Unit, and Uncertainty](http://physics.nist.gov/cuu/Constants/).

",
        base: base_units!(
            M: Meter, m, Length;
            KG: Kilogram, kg, Mass;
            S: Second, s, Time;
            A: Ampere, A, Current;
            K: Kelvin, K, Temperature;
            CD: Candela, cd, LuminousIntensity;
            MOL: Mole, mol, AmountOfSubstance;
        ),
        derived: derived_units!(
            HZ: Hertz = Unitless / Second, Frequency;
            N: Newton = Kilogram * Meter / Second2, Force;
            PA: Pascal = Newton / Meter2, Pressure;
            J: Joule = Newton * Meter, Energy;
            W: Watt = Joule / Second, Power;
            C: Coulomb = Second * Ampere, Charge;
            V: Volt = Watt / Ampere, ElectricPotential;
            F: Farad = Coulomb / Volt, Capacitance;
            OHM: Ohm = Volt / Ampere, Resistance;
            SIE: Siemens = Ampere / Volt, Conductance;
            WB: Weber = Joule / Ampere, MagneticFlux;
            T: Tesla = Weber / Meter2;
            H: Henry = Ohm * Second, Inductance;
            LM: Lumen = Candela;
            LX: Lux = Candela / Meter2;
            BQ: Becquerel = Hertz;
            GY: Gray = Joule / Kilogram;
            SV: Sievert = Gray;
            KAT: Katal = Mole / Second;

            M2: Meter2 = Meter * Meter, Area;
            M3: Meter3 = Meter2 * Meter, Volume;

            PM: PerMeter = Unitless / Meter, ReciprocalLength;
            PM2: PerMeter2 = PerMeter / Meter;
            PM3: PerMeter3 = PerMeter2 / Meter;

            S2: Second2 = Second * Second;
            S3: Second3 = Second2 * Second;
            S4: Second4 = Second3 * Second;

            PS2: PerSecond2 = Unitless / Second2;
            PS3: PerSecond3 = Unitless / Second3;
            PS4: PerSecond4 = Unitless / Second4;

            MPS: MeterPerSecond = Meter / Second, Velocity;
            MPS2: MeterPerSecond2 = Meter / Second2, Acceleration;
            MPS3: MeterPerSecond3 = Meter / Second3, Jerk;
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

            KGPM: KilogramPerMeter = Kilogram / Meter;
            KGPM2: KilogramPerMeter2 = Kilogram / Meter2;
            KGPM3: KilogramPerMeter3 = Kilogram / Meter3;

            MPKG: MeterPerKilogram = Meter / Kilogram;
            M2PKG: Meter2PerKilogram = Meter2 / Kilogram;
            M3PKG: Meter3PerKilogram = Meter3 / Kilogram;

            MOLPM: MolePerMeter = Mole / Meter;
            MOLPM2: MolePerMeter2 = Mole / Meter2;
            MOLPM3: MolePerMeter3 = Mole / Meter3;

            MPMOL: MeterPerMole = Meter / Mole;
            M2PMOL: Meter2PerMole = Meter2 / Mole;
            M3PMOL: Meter3PerMole = Meter3 / Mole;

            JPK: JoulePerKelvin = Joule / Kelvin;
            JPKMOL: JoulePerKelvinMole = JoulePerKelvin / Mole;
            JPKGK: JoulePerKilogramKelvin = JoulePerKelvin / Kilogram;
            JPMOL: JoulePerMole = Joule / Mole;
            JPKG: JoulePerKilogram = Joule / Kilogram;
            JPM2: JoulePerMeter2 = Joule / Meter2;
            JPM3: JoulePerMeter3 = Joule / Meter3;

            NPM: NewtonPerMeter = JoulePerMeter2;
            WPM2: WattPerMeter2 = Watt / Meter2;
            WPMK: WattPerMeterKelvin = Watt / Meter / Kelvin;

            PAS: PascalSecond = Pascal * Second;

            CPM2: CoulombPerMeter2 = Coulomb / Meter2;
            CPM3: CoulombPerMeter3 = Coulomb / Meter3;
            APM2: AmperePerMeter2 = Ampere / Meter2;
            SIEPM: SiemensPerMeter = Siemens / Meter;
            SIEM2PMOL: SiemensMeter2PerMole = Siemens * Meter2 / Mole;
            FPM: FaradPerMeter = Farad / Meter;
            HPM: HenryPerMeter = Henry / Meter;
            VPM: VoltPerMeter = Volt / Meter;
            APM: AmperePerMeter = Ampere / Meter;
            CDPM2: CandelaPerMeter2 = Candela / Meter2;
            LMS: LumenSecond = Lumen * Second;
            LXS: LuxSecond = Lux * Second;
            CPKG: CoulombPerKilogram = Coulomb / Kilogram;
            GYPS: GrayPerSecond = Gray / Second;
            OHMM: OhmMeter = Ohm * Meter;
            CPM: CoulombPerMeter = Coulomb / Meter;
            MOLPKG: MolePerKilogram = Mole / Kilogram;
            KGPMOL: KilogramPerMole = Kilogram / Mole;
            KGPS: KilogramPerSecond = Kilogram / Second;
            JPT: JoulePerTesla = Joule / Tesla;
            WPM3: WattPerMeter3 = Watt / Meter3;
            KPW: KelvinPerWatt = Kelvin / Watt;
            PK: PerKelvin = Unitless / Kelvin;
            KPM: KelvinPerMeter = Kelvin / Meter;
            M2PVS: Meter2PerVoltSecond = Meter2 / Volt / Second;
            JPM2S: JoulePerMeter2Second = JoulePerMeter2 / Second;
            PPA: PerPascal = Unitless / Pascal;
            PH: PerHenry = Unitless / Henry;
            WBPM: WeberPerMeter = Weber / Meter;
            WBM: WeberMeter = Weber * Meter;
            TM: TeslaMeter = Tesla * Meter;
            M3PMOLS: Meter3PerMoleSecond = Meter3 / Mole / Second;
            NMSPKG: NewtonMeterSecondPerKilogram = Newton * Meter * Second / Kilogram;
            LMPW: LumenPerWatt = Lumen / Watt;
            MPH: MeterPerHenry = Meter / Henry;
            WPM: WattPerMeter = Watt / Meter;
        ),
        constants: constants!(
            RAD: Unitless = 1.0 * ONE.value_unsafe, "Radian";
            SR: Unitless = 1.0 * ONE.value_unsafe, "Steradian";

            MIN: Second = 60.0 * S.value_unsafe, "Minute";
            HR: Second = 60.0 * MIN.value_unsafe, "Hour";
            DAY: Second = 24.0 * HR.value_unsafe, "Day";

            DEG: Unitless = consts::PI / 180.0 * RAD.value_unsafe, "Degree";
            HA: Meter2 = 10000.0 * M2.value_unsafe, "Hectare";
            L: Meter3 = 0.001 * M3.value_unsafe, "Liter";
            TNE: Kilogram = 1.0e3 * KG.value_unsafe, "Tonne";
            AU: Meter = 149_597_870_700.0 * M.value_unsafe, "Astronomical unit";

            E: Coulomb = 1.6021766208e-19 * C.value_unsafe, "Elementary charge";
            EV: Joule = E.value_unsafe * V.value_unsafe, "Electronvolt";
            U: Kilogram = 1.660539040e-27 * KG.value_unsafe, "Unified atomic mass unit; dalton";

            C0: MeterPerSecond = 299_792_458.0 * MPS.value_unsafe, "Speed of light in a vacuum";

            HBAR: JouleSecond = 1.054571800e-34 * JS.value_unsafe, "Reduced Planck constant";
            M_E: Kilogram = 9.10938356e-31 * KG.value_unsafe, "Electron mass";
            R_BOHR: Meter = 0.52917721067e-10 * M.value_unsafe, "Bohr radius";
            EH: Joule = 4.359744650e-18 * J.value_unsafe, "Hartree energy";
            AO: Meter = 1e-10 * M.value_unsafe, "Ångström";
            ARE: Meter2 = 100.0 * M2.value_unsafe, "Are";
            BARN: Meter2 = 1e-28 * M2.value_unsafe, "Barn";
            BAR: Pascal = 1e5 * PA.value_unsafe, "Bar";
            MBAR: Pascal = 100.0 * PA.value_unsafe, "Millibar";
            ATM: Pascal = 101_325.0 * PA.value_unsafe, "Atmosphere";
            BA: Pascal = 0.1 * PA.value_unsafe, "Barye";
            MMHG: Pascal = 133.322387415 * PA.value_unsafe, "Millimeter of mercury";
            TORR: Pascal = ATM.value_unsafe / 760.0, "Torr";

            DYN: Newton = 1.0e-5 * N.value_unsafe, "Dyne";

            FT: Meter = 0.3048 * M.value_unsafe, "Foot";
            IN: Meter = FT.value_unsafe / 12.0, "Inch";
            YD: Meter = 3.0 * FT.value_unsafe, "Yard";
            MI: Meter = 5_280.0 * FT.value_unsafe, "Mile";

            LB: Kilogram = 0.45359237 * KG.value_unsafe, "Pound mass";
            OZ: Kilogram = LB.value_unsafe / 16.0, "Ounce";

            LBF: Newton = 4.4482216152605 * N.value_unsafe, "Pound force";
        ),
        fmt: true,
        from: vec!["UCUM"],
        refl_blacklist: vec![
            "MOL",
            "KAT",
            "MOLPM",
            "MPMOL",
            "MOLPKG",
            "KGPMOL",
            "M2PMOL",
            "M3PMOL",
            "SIEM2PMOL",
            "JPKMOL",
            "JPMOL",
            "MOLPM2",
            "MOLPM3",
            "M3PMOLS",
        ],
    }
}
