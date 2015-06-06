#[macro_use]
extern crate dimensioned;

make_units! {
    Fruit, Unitless, one;
    base {
        Apple, apple, A;
        Banana, banana, B;
        Cucumber, cucumber, C;
    }
    derived {
    }
}


#[test]
fn test_making_units() {
    let x = apple * banana;
    println!("x: {}", x);
}
