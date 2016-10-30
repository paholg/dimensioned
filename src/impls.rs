

macro_rules! dim_scalar_mult {
    ($t: ty) => (
        /// Scalar multiplication with scalar on LHS
        impl<D, V> Mul<Dim<D, V>> for $t where $t: Mul<V> {
            type Output = Dim<D, <$t as Mul<V>>::Output>;
            #[inline]
            fn mul(self, rhs: Dim<D, V>) -> Self::Output {
                Dim( self * rhs.0, PhantomData )
            }
        }

        /// Scalar multiplication with scalar on RHS
        #[cfg(not(feature = "nightly"))]
        impl<D, V> Mul<$t> for Dim<D, V> where V: Mul<$t> {
            type Output = Dim<D, <V as Mul<$t>>::Output>;
            #[inline]
            fn mul(self, rhs: $t) -> Self::Output {
                Dim( self.0 * rhs, PhantomData )
            }
        }

    );
}

dim_scalar_mult!(f32);
dim_scalar_mult!(f64);
dim_scalar_mult!(i8);
dim_scalar_mult!(i16);
dim_scalar_mult!(i32);
dim_scalar_mult!(i64);
dim_scalar_mult!(isize);
dim_scalar_mult!(u8);
dim_scalar_mult!(u16);
dim_scalar_mult!(u32);
dim_scalar_mult!(u64);
dim_scalar_mult!(usize);


macro_rules! dim_scalar_div {
    ($t: ty) => (
        /// Scalar division with scalar on LHS
        impl<D, V> Div<Dim<D, V>> for $t
            where D: Recip,
                  <D as Recip>::Output: Dimension,
                  $t: Div<V>
        {
            type Output = Dim<<D as Recip>::Output, <$t as Div<V>>::Output>;
            #[inline]
            fn div(self, rhs: Dim<D, V>) -> Self::Output {
                Dim( self / rhs.0, PhantomData )
            }
        }

        /// Scalar division with scalar on RHS
        #[cfg(not(feature = "nightly"))]
        impl<D, V> Div<$t> for Dim<D, V> where V: Div<$t> {
            type Output = Dim<D, <V as Div<$t>>::Output>;
            #[inline]
            fn div(self, rhs: $t) -> Self::Output {
                Dim( self.0 / rhs, PhantomData )
            }
        }

        );
}
dim_scalar_div!(f32);
dim_scalar_div!(f64);
dim_scalar_div!(i8);
dim_scalar_div!(i16);
dim_scalar_div!(i32);
dim_scalar_div!(i64);
dim_scalar_div!(isize);
dim_scalar_div!(u8);
dim_scalar_div!(u16);
dim_scalar_div!(u32);
dim_scalar_div!(u64);
dim_scalar_div!(usize);


// fixme: figure this out
// impl<D, V, Idx> Index<Idx> for Dim<D, V>
//     where D: Dimension, V: Index<Idx>, <V as Index<Idx>>::Output: Sized
// {
//     type Output = Dim<D, <V as Index<Idx>>::Output>;
//     fn index<'a>(&'a self, index: Idx) -> &'a Self::Output {
//         &Dim::new((self.0)[index])
//     }
// }

// {

//     trait Sqr {
//         fn sqr(self) -> Self;
//     }

//     macro_rules! impl_sqr {
//         ($t:ty) => (
//             impl Sqr for $t {
//                 fn sqr(self) -> Self {
//                     self * self
//                 }
//             }
//         );
//     }
// }
