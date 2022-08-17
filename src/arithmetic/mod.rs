type Arg = i64;

pub fn add(a: Arg, b: Arg) -> Arg {
    return a + b;
}

// How to define separate test file
#[cfg(test)]
#[path = "./test.rs"]
mod test;
