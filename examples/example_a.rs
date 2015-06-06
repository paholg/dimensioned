#[macro_use]
extern crate dimensioned;
extern crate vector3;

use vector3::vector3a::Vector3;
use dimensioned::si::{one, m, kg, s};

use dimensioned::{Dim, Dimension, KeepDim, MulDim};

dim_impl_unary!(Norm, norm, KeepDim, Vector3 -> f64);
dim_impl_unary!(Norm2, norm2, MulDim, Vector3 -> f64);

dim_impl_binary!(Dot, dot, MulDim, Vector3 -> f64);
dim_impl_binary!(Cross, cross, MulDim, Vector3 -> Vector3);

fn main() {
    let xhat = one * Vector3::new(1.0, 0.0, 0.0);
    let yhat = one * Vector3::new(0.0, 1.0, 0.0);
    let zhat = one * Vector3::new(0.0, 0.0, 1.0);


    let start = -13.0*xhat*m + 33.0*yhat*m;
    let end = 26.0*xhat*m - 19.0*yhat*m;

    let displace = end - start;
    let time = 26.0*s;
    let vel = displace/time;


    let speed = vel.norm();
    println!("
A physicist was standing at {}.
Then she walked to {}, for a displacement of {}.
The walk took her {}, so she must have had a velocity of {}.
That's a speed of {}!", start, end, displace, time, vel, speed);

    let center = 24.0*xhat*m - 17.0*yhat*m;
    let force = 500.0*xhat*kg*m/s/s;
    let r = end - center;
    println!("
Now, she's standing next to a merry-go-round, centered at {}.
That is {} away from her. She decides to spin it, pushing with a force of {}.
That's a torque of {}!", center, r.norm(), force, r.cross(force).norm());
}

