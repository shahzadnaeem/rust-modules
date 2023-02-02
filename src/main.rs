mod arithmetic; // arithmetic/mod.rs
mod funs; // funs.rs
mod util; // utils.rs exports util/{cafe,math,point}

//NOTE: Not idiomatic rust!
use crate::funs::*; // Make all exports visible directly

// NOTE: This is idiomatic rust
use crate::util::{cafe, math};

// NOTE: For structs, this is OK
use crate::util::point::Point;

fn main() {
    // `arithmetic`
    println!("1 + 2 = {}", arithmetic::add(1, 2));

    // Direct use of `funs` exports
    println!("It's true. {}\n{}", haha(), rude());
    println!("A util: {}\n", util1());

    // `util`...
    println!("rough PI = {}", math::rough_pi());
    println!("math version = {}", math::version());

    (1..8).for_each(|mins| {
        println!("Brewing for {} mins is {}", mins, cafe::brew(mins));
    });

    let p = Point::new();

    println!("new Point = {} [Debug: {:?}]", p, p);
}
