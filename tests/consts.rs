extern crate dimensioned as dim;

use dim::{si, ucum};

macro_rules! cmp_consts {
    ($($c:ident),*) => (
        $({
            let a = ucum::$c;
            let b = si::$c;

            assert_eq!(a, ucum::UCUM::from(si::SI::from(a)));
            assert_eq!(b, si::SI::from(ucum::UCUM::from(b)));
            assert_eq!(a, ucum::UCUM::from(b));
            assert_eq!(b, si::SI::from(a));
        }
        )*
    );
}

#[test]
fn test_consts() {
    cmp_consts!(M, A, C, S, V, F, H, T, J, KG);
}
