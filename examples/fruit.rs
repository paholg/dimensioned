#[macro_use]
extern crate dimensioned as dim;

make_units!{
    Fruit, Unitless;
    base {
        Apple, a;
        Banana, b;
        Cucumber, c;
        Mango, m;
        Watermelon, w;
    }
    derived {
    }
}

fn main() {
    let fruit_salad = Apple::new(1.0) * Banana::new(1.0) * Cucumber::new(1.0) * Mango::new(1.0) * Mango::new(1.0) * Watermelon::new(1.0);
    // prints "Mmmm, delicious: 1 a*b*m^2*w":
    println!("Mmmm, delicious: {}", fruit_salad);
}
