make_units! {
    SI, Unitless, one;
    base {
        Meter, m, m;
        Kilogram, kg, kg;
        Second, s, s;
        Ampere, amp, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
        newton: Newton = kg*m/s/s;
    }
}
