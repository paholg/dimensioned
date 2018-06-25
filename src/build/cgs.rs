use super::*;

pub fn new() -> System {
    System {
        name: "CGS",
        module: "cgs",
        doc_prelude: "The Gaussian CGS unit system

Note: this system is incomplete. More derived units and constants are coming.

",
        base: base_units!(
            SQRTCM: SqrtCentimeter, sqrtcm;
            SQRTG: SqrtGram, sqrtg;
            S: Second, s, Time;
        ),
        derived: derived_units!(
            CM: Centimeter = SqrtCentimeter * SqrtCentimeter, Length;
            G: Gram = SqrtGram * SqrtGram, Mass;

            CM2: Centimeter2 = Centimeter * Centimeter, Area;
            CM3: Centimeter3 = Centimeter2 * Centimeter, Volume;

            S2: Second2 = Second * Second;
            S3: Second3 = Second2 * Second;
            S4: Second4 = Second3 * Second;

            CMPS: CentimeterPerSecond = Centimeter / Second, Velocity;
            CMPS3: CentimeterPerSecond3 = Centimeter / Second3, Jerk;
            CMPS4: CentimeterPerSecond4 = Centimeter / Second4;

            CM2PS: Centimeter2PerSecond = Centimeter2 / Second;
            CM2PS2: Centimeter2PerSecond2 = Centimeter2 / Second2;
            CM2PS3: Centimeter2PerSecond3 = Centimeter2 / Second3;

            CM3PS: Centimeter3PerSecond = Centimeter3 / Second;
            CM3PS2: Centimeter3PerSecond2 = Centimeter3 / Second2;
            CM3PS3: Centimeter3PerSecond3 = Centimeter3 / Second3;

            GAL: Gal = Centimeter / Second2, Acceleration;
            DYN: Dyne = Gram *  Gal, Force;
            ERG: Erg = Dyne * Centimeter, Energy;
            ERGPS: ErgPerSecond = Erg / Second, Power;
            BA: Barye = Dyne / Centimeter2, Pressure;
            P: Poise = Gram / Centimeter / Second;
            ST: Stokes = Centimeter2 / Second;
            K: Kayser = Unitless / Centimeter, ReciprocalLength;

            STATC: StatCoulomb = SqrtGram * SqrtCentimeter * Centimeter / Second;
            STATA: StatAmpere = StatCoulomb / Second;
            STATV: StatVolt = Erg / StatCoulomb;
        ),
        constants: constants!(
            M: Centimeter = HECTO * CM.value_unsafe, "Meter";
        ),
        fmt: false,
        from: vec!["SI", "MKS"],
        refl_blacklist: Vec::new(),
    }
}
