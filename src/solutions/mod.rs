mod day01;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Integer(i64),
    // String(String),
}

pub type PuzzleSolution = Option<fn(&str) -> Solution>;

const DAYS: [(PuzzleSolution, PuzzleSolution); 1] = [(Some(day01::part_a), Some(day01::part_b))];

#[must_use]
pub fn get_solution(day: u8) -> (PuzzleSolution, PuzzleSolution) {
    if day > 0 && day <= MAX_SOLVED_DAY {
        DAYS[day as usize - 1]
    } else {
        (None, None)
    }
}

pub const MAX_SOLVED_DAY: u8 = DAYS.len() as u8;
