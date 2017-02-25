use super::*;

pub fn new() -> System {
    System {
        name: "SI", module: "si",
        doc_prelude: "The SI unit system.

Note: Incomplete - fixme.

It was generated using the `make_units!` macro.

",
        base: base_units!(
            M: Meter, m, Length;
            KG: Kilogram, kg, Mass;
            S: Second, s, Time;
            A: Ampere, A, Current;
            K: Kelvin, K, Temperature;
            CD: Candela, cd, LuminousIntensity;
            MOL: Mole, mol;
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
            SIEMENS: Siemens = Ampere / Volt, Conductance;
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
        ),
        constants: constants!(
            MIN: Second = 60.0 * S.value_unsafe;
            HR: Second = 60.0 * MIN.value_unsafe;
            DAY: Second = 24.0 * HR.value_unsafe;
        )
    }
}
