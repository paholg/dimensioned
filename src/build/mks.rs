use super::*;

pub fn new() -> System {
    System {
        name: MKS, module: "mks",
        doc_prelude: "The Gaussian MKS unit system

Note: Incomplete - fixme.

",
        base: base_units!(
            SQRTM: SqrtMeter, sqrtm;
            SQRTKG: SqrtKilogram, sqrtkg;
            S: Second, s, Time;
        ),
        derived: derived_units!(
            M: Meter = SqrtMeter * SqrtMeter, Length;
            KG: Kilogram = SqrtKilogram * SqrtKilogram, Mass;

            MPS: MeterPerSecond = Meter / Second, Velocity;
        ),
        constants: constants!(
        ),
        fmt: false,
        from: vec![
            SI,
            CGS,
        ],
        refl_blacklist: Vec::new(),
    }
}
