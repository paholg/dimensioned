use std::fmt;

pub trait CmpConsts<B> {
    fn test_eq(self, b: B);
}

#[cfg(feature = "spec")]
impl<A, B> CmpConsts<B> for A {
    default fn test_eq(self, _: B) {
    }
}

impl<A, B> CmpConsts<B> for A where A: PartialEq + From<B> + fmt::Debug {
    fn test_eq(self, b: B) {
        assert_eq!(self, A::from(b));
    }
}

