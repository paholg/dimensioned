
use super::*;

pub fn new() -> System {
    System {
        name: "MKS", module: "mks",
        doc_prelude: "The Gaussian MKS unit system.

Note: Incomplete - fixme.

It was generated using the `make_units!` macro.

",
        base: base_units!(
            M: Meter, m, P2, Length;
            KG: Kilogram, kg, P2, Mass;
            S: Second, s, P1, Time;
        ),
        derived: derived_units!(
            MPS: MeterPerSecond = Meter / Second, Velocity;
        ),
        constants: constants!(
        )
    }
}
