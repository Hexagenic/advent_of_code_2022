mod day01;
mod day02;
mod day03;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Integer(i64),
    // String(String),
}

pub type PuzzleSolution = Option<fn(&str) -> Solution>;

const DAYS: [(PuzzleSolution, PuzzleSolution); 3] = [
    (Some(day01::part_a), Some(day01::part_b)),
    (Some(day02::part_a), Some(day02::part_b)),
    (Some(day03::part_a), Some(day03::part_b)),
];

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    if day > 0 && day <= MAX_SOLVED_DAY {
        DAYS[day as usize - 1]
    } else {
        (None, None)
    }
}

#[allow(clippy::cast_possible_truncation)]
pub const MAX_SOLVED_DAY: u8 = DAYS.len() as u8;
