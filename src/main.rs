mod arithmetic;
mod funs;
mod util;

use crate::util::math;

fn main() {
    println!("1 + 2 = {}", arithmetic::add(1, 2));

    println!("rough PI = {}", math::rough_pi());

    println!("math version = {}", math::version());

    println!("It's true. {}\n{}", funs::haha(), funs::rude());

    println!("A util: {}\n", util::util1());
}
