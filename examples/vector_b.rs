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

    // fixable?
    // m * xhat;

    let start = -13.0 * xhat * m + 33.0 * yhat * m;
    let end = 26.0 * xhat * m - 19.0 * yhat * m;

    let displace = end - start;
    let time = 26.0 * s;
    let vel = displace/time;


    let speed = vel.norm();
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}!", start, end, displace, time, vel, speed);

    let center = 24.0 * xhat * m - 17.0 * yhat * m;
    let force = 500.0 * xhat * kg*m/s/s;
    let r = end - center;
    println!("
Now, she's standing next to a merry-go-round, centered at {}.
That is {} away from her. She decides to spin it, pushing with a force of {}.
That's a torque of {}!", center, r.norm2(), force, r.cross(force).norm());
}
