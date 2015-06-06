extern crate dimensioned;
extern crate num;

use num::traits::Float;

pub struct A<V>(pub V);

pub trait Sqrt {
    fn sqrt(self) -> Self;
}

impl<V> Sqrt for A<V> where V: Float {
    fn sqrt(self) -> Self { A( (self.0).sqrt()) }
}

fn main() {
    let x: A<f64> = A(1.0);
    x.sqrt();
}
