
trait Sqr {
    fn sqr(self) -> Self;
}

macro_rules! impl_sqr {
    ($t:ty) => (
        impl Sqr for $t {
            fn sqr(self) -> Self {
                self * self
            }
        }
    );
}
