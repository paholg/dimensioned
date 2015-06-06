#![feature(core)]
#[macro_use]
extern crate dimensioned;

fn print_type_of<T>(_: &T) -> () {
    let type_name =
        unsafe {
            std::intrinsics::type_name::<T>()
        };
    println!("{}", type_name);
}

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
    print_type_of(&apple);
    print_type_of(&banana);
    print_type_of(&cucumber);
}
