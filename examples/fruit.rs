#[macro_use]
extern crate dimensioned;

make_units!{
    Fruit, Unitless, one;
    base {
        Apple, apple, a;
        Banana, banana, b;
        Cucumber, cuke, c;
        Mango, mango, m;
        Watermelon, watermelon, w;
    }
    derived {
    }
}

fn main() {
    let fruit_salad = apple * banana * mango * mango * watermelon;
    println!("Mmmm, delicious: {}", fruit_salad);
}
