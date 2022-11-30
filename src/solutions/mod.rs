mod day01;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Integer(i64),
    // String(String),
}

pub type PuzzleSolution = Option<fn(&str) -> Solution>;

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    match day {
        1 => (Some(day01::part_a), Some(day01::part_b)),
        _ => (None, None),
    }
}

pub const MAX_SOLVED_DAY: u8 = 1;
