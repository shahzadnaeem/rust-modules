mod arithmetic; // arithmetic/mod.rs
mod funs; // funs.rs
pub mod util; // exports util/{cafe,math}

fn main() {
    println!("1 + 2 = {}", arithmetic::add(1, 2));

    println!("rough PI = {}", util::math::rough_pi());

    println!("math version = {}", util::math::version());

    println!("It's true. {}\n{}", funs::haha(), funs::rude());

    println!("A util: {}\n", funs::util1());

    (1..8).for_each(|mins| {
        println!("Brewing for {} mins is {}", mins, util::cafe::brew(mins));
    });
}
