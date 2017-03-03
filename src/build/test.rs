use std::fmt;
use dim::Dimensioned;

pub trait CmpConsts<B> {
    fn test_eq(self, b: B);
}

#[cfg(feature = "spec")]
impl<A, B> CmpConsts<B> for A {
    default fn test_eq(self, _: B) {
    }
}

impl<A, B> CmpConsts<B> for A where A: PartialEq + From<B> + fmt::Debug + Dimensioned<Value=f64> {
    fn test_eq(self, b: B) {
        let x = self.value_unsafe();
        let temp = A::from(b);
        let y = temp.value_unsafe();

        assert_ulps_eq!(x, y, epsilon = 0.0, max_ulps = 2);
        assert_relative_eq!(x, y, epsilon = 0.0);
    }
}
