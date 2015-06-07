extern crate dimensioned as dim;
extern crate vector3;
extern crate num;

use vector3::vector3b::{Vector3, Norm2, Cross, Dot};

use num::Float;

use dim::Sqrt;
use dim::si::{one, m, kg, s};

pub trait Norm {
    type Output;
    fn norm(self) -> Self::Output;
}

impl<N> Norm for Vector3<N> where Vector3<N>: Norm2, <Vector3<N> as Norm2>::Output: Sqrt {
    type Output = <<Vector3<N> as Norm2>::Output as Sqrt>::Output;
    #[inline]
    fn norm(self) -> Self::Output { self.norm2().sqrt() }
}


fn main() {
    let xhat = Vector3::new(one, 0.0*one, 0.0*one);
    let yhat = Vector3::new(0.0*one, one, 0.0*one);
    let zhat = Vector3::new(0.0*one, 0.0*one, one);


    let start = -22.0*xhat*m + 5.0*yhat*m + 6.0*zhat*m;
    println!("A physicist was standing on a hill at position {}.", start);

    let end = 26.0*xhat*m - 19.0*yhat*m;
    println!("Then she walked down the hill to {}.", end);

    let displace = end - start;
    println!("So, her displacement vector was {}.", displace);

    let time = 30.0*s;
    println!("The walk took her {}.", time);

    let velocity = displace/time;
    println!("She must have had an average velocity of {}.", velocity);

    let speed = velocity.norm();
    println!("So, her average speed was {}.", speed);

    let center = 28.0*xhat*m - 21.0*yhat*m;
    println!("Now, she's standing next to a merry-go-round, centered at {}.", center);

    let force = 500.0*xhat*kg*m/s/s;
    println!("She decides to spin it, pushing with a force of {}", force);

    let r = end - center;
    let torque = r.cross(force);
    println!("That's a torque of {}!", torque);
}
