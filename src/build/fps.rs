use super::*;

pub fn new() -> System {
    System {
        name: "FPS",
        module: "fps",
        doc_prelude: "The foot, pound, second system, using the mass pound as a base unit.

Note: this system is incomplete. More derived units and constants are coming.

",
        base: base_units!(
            SQRTFT: SqrtFoot, sqrtft;
            SQRTLB: SqrtPound, sqrtlb;
            S: Second, s, Time;
        ),
        derived: derived_units!(
            FT: Foot = SqrtFoot * SqrtFoot, Length;
            LB: Pound = SqrtPound * SqrtPound, Mass;
        ),
        constants: constants!(),
        fmt: false,
        from: vec![
            // "SI",
            // "MKS",
        ],
        refl_blacklist: Vec::new(),
    }
}
