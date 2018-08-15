use approx::UlpsEq;
use dim::Dimensioned;
use std::fmt;

pub trait CmpConsts<B> {
    fn test_eq(self, b: B);
}

#[cfg(feature = "spec")]
impl<A, B> CmpConsts<B> for A {
    default fn test_eq(self, _: B) {}
}

impl<A, B> CmpConsts<B> for A
where
    A: From<B> + fmt::Debug + UlpsEq,
    A::Epsilon: Dimensioned<Value = f64>,
{
    fn test_eq(self, b: B) {
        assert_ulps_eq!(self, b.into(), epsilon = A::Epsilon::new(0.0), max_ulps = 2);
    }
}
