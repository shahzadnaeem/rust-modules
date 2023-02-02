pub const LO: i32 = 3;
pub const HI: i32 = 5;

pub fn brew(i: i32) -> bool {
    LO <= i && i <= HI
}
