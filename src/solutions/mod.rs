mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;

#[derive(Debug, PartialEq, Eq)]
pub enum Solution {
    Integer(i64),
    String(String),
}

pub type PuzzleSolution = Option<fn(&str) -> Solution>;

const DAYS: [(PuzzleSolution, PuzzleSolution); 10] = [
    (Some(day01::part_a), Some(day01::part_b)),
    (Some(day02::part_a), Some(day02::part_b)),
    (Some(day03::part_a), Some(day03::part_b)),
    (Some(day04::part_a), Some(day04::part_b)),
    (Some(day05::part_a), Some(day05::part_b)),
    (Some(day06::part_a), Some(day06::part_b)),
    (Some(day07::part_a), Some(day07::part_b)),
    (Some(day08::part_a), Some(day08::part_b)),
    (Some(day09::part_a), Some(day09::part_b)),
    (Some(day10::part_a), Some(day10::part_b)),
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
