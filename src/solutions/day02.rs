use crate::solutions::Solution;

const SCORE_WIN: i64 = 6;
const SCORE_DRAW: i64 = 3;
const SCORE_LOSE: i64 = 0;

const SCORE_ROCK: i64 = 1;
const SCORE_PAPER: i64 = 2;
const SCORE_SCISSOR: i64 = 3;

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(file.lines().map(score_a).sum())
}

//noinspection DuplicatedCode
fn score_a(line: &str) -> i64 {
    match line {
        "A X" => SCORE_ROCK + SCORE_DRAW,
        "A Y" => SCORE_PAPER + SCORE_WIN,
        "A Z" => SCORE_SCISSOR + SCORE_LOSE,
        "B X" => SCORE_ROCK + SCORE_LOSE,
        "B Y" => SCORE_PAPER + SCORE_DRAW,
        "B Z" => SCORE_SCISSOR + SCORE_WIN,
        "C X" => SCORE_ROCK + SCORE_WIN,
        "C Y" => SCORE_PAPER + SCORE_LOSE,
        "C Z" => SCORE_SCISSOR + SCORE_DRAW,
        _ => panic!("Unknown line"),
    }
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(file.lines().map(score_b).sum())
}

//noinspection DuplicatedCode
fn score_b(line: &str) -> i64 {
    match line {
        "A X" => SCORE_SCISSOR + SCORE_LOSE,
        "A Y" => SCORE_ROCK + SCORE_DRAW,
        "A Z" => SCORE_PAPER + SCORE_WIN,
        "B X" => SCORE_ROCK + SCORE_LOSE,
        "B Y" => SCORE_PAPER + SCORE_DRAW,
        "B Z" => SCORE_SCISSOR + SCORE_WIN,
        "C X" => SCORE_PAPER + SCORE_LOSE,
        "C Y" => SCORE_SCISSOR + SCORE_DRAW,
        "C Z" => SCORE_ROCK + SCORE_WIN,
        _ => panic!("Unknown line"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!(8, score_a("A Y"));
        assert_eq!(1, score_a("B X"));
        assert_eq!(6, score_a("C Z"));

        assert_eq!(Solution::Integer(15), part_a("A Y\nB X\nC Z"));
    }

    #[test]
    fn test_b() {
        assert_eq!(4, score_b("A Y"));
        assert_eq!(1, score_b("B X"));
        assert_eq!(7, score_b("C Z"));

        assert_eq!(Solution::Integer(12), part_b("A Y\nB X\nC Z"));
    }
}
