pub fn haha() -> &'static str {
    return "Surely you're joking!";
}

pub fn rude() -> &'static str {
    return filthy();
}

fn filthy() -> &'static str {
    return "What's brown and sticky? ... A STICK!";
}

pub fn util1() -> String {
    "util1".to_string()
}
