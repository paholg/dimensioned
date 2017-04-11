use std::fmt;
use dim::Dimensioned;
use approx::ApproxEq;

pub trait CmpConsts<B> {
    fn test_eq(self, b: B);
}

#[cfg(feature = "spec")]
impl<A, B> CmpConsts<B> for A {
    default fn test_eq(self, _: B) {
    }
}

impl<A, B> CmpConsts<B> for A where A: From<B> + fmt::Debug + Dimensioned<Value=f64> + ApproxEq<Epsilon=Self> {
    fn test_eq(self, b: B) {
        assert_ulps_eq!(self, b.into(), epsilon = A::new(0.0), max_ulps = 2);
    }
}
