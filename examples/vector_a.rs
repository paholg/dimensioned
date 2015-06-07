#[macro_use]
extern crate dimensioned as dim;
extern crate vector3;

use vector3::vector3a::Vector3;
use dim::si::{one, m, kg, s};

use dim::{Dim, Dimension, KeepDim, MulDim};

dim_impl_unary!(Norm, norm, KeepDim, Vector3 => f64);
dim_impl_unary!(Norm2, norm2, MulDim, Vector3 => f64);

dim_impl_binary!(Dot, dot, MulDim, Vector3 => f64);
dim_impl_binary!(Cross, cross, MulDim, Vector3 => Vector3);

fn main() {
    let xhat = one * Vector3::new(1.0, 0.0, 0.0);
    let yhat = one * Vector3::new(0.0, 1.0, 0.0);
    let zhat = one * Vector3::new(0.0, 0.0, 1.0);


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

