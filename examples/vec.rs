#[macro_use]
extern crate dimensioned;
extern crate nalgebra;

use dimensioned::dimensioned::*;

use nalgebra::Vec2;

make_units! {
    SI, One;
    base {
        Meter, meter, m;
        Kilogram, kilogram, kg;
        Second, second, s;
        Ampere, ampere, A;
        Kelvin, kelvin, K;
        Candela, candela, cd;
        Mole, mole, mol;
    }
    derived {
        Newton(newton) = Kilogram * Meter / Second / Second;
    }
}


fn main() {
    let xhat: Dim<Unitless, Vec2<f64>> = Dim::new(Vec2{x: 1.0, y: 0.0});
    let yhat: Dim<Unitless, Vec2<f64>> = Dim::new(Vec2{x: 1.0, y: 0.0});

    xhat*meter;


//     let start = -xhat*13.0*meter + yhat*33.0*meter;
//     let end = xhat*26.0*meter - yhat*19.0*meter;

//     let displace = end - start;
//     let time = second*26.0;
//     let vel = displace/time;
//     // Because we put norm() in a trait and implemented it for both Vector2d and Dim,
//     // calling vel.norm() works as we want it to (returning Dim<Meter, ff64>). This is
//     // the recommended way of accessing values inside a Dim.
//     let speed = vel.norm();
//     // Had we been unable or unwilling to implement norm() inside a trait, we could have
//     // achieved the same behavior using the wrap() function, as follows:
//     let speed2 = vel.wrap((vel.0).norm());
//     println!("
// A physicist was standing at {}.
// Then she walked to {}, for a displacement of {}.
// The walk took her {}, so she must have had a velocity of {}.
// That's a speed of {}! Again, that's {}!", start, end, displace, time, vel, speed, speed2);

//     let center = xhat*meter*24.0 - yhat*meter*17.0;
//     let force = xhat*500.0*kilogram*meter/second/second;
//     let r = end-center;
//     println!("
// Now, she's standing next to a merry-go-round, centered at {}.
// That is {} away from her. She decides to spin it, pushing with a force of {}.
// That's a torque of {}!", center, r.norm(), force, r.cross(force));
}
