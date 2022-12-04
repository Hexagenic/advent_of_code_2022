use crate::solutions::Solution;

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(file.lines().map(score_backpack).sum())
}

pub fn part_b(file: &str) -> Solution {
    let mut lines = file.lines().peekable();

    let mut total: i64 = 0;

    loop {
        let first = lines.next().unwrap();
        let second = lines.next().unwrap();
        let third = lines.next().unwrap();

        total += score_item(in_three(first, second, third));

        if lines.peek().is_none() {
            break;
        }
    }

    Solution::Integer(total)
}

fn in_both(backpack: &str) -> char {
    let (first, second) = backpack.split_at(backpack.len() / 2);
    for c in first.chars() {
        if second.chars().any(|d| d == c) {
            return c;
        }
    }

    panic!("No common characters");
}

fn in_three(first: &str, second: &str, third: &str) -> char {
    for c in first.chars() {
        if second.chars().any(|d| d == c) && third.chars().any(|d| d == c) {
            return c;
        }
    }

    panic!("No common characters");
}

fn score_item(i: char) -> i64 {
    if i >= 'a' {
        i64::from(i as u32) - 96
    } else {
        i64::from(i as u32) + 26 - 64
    }
}

fn score_backpack(backpack: &str) -> i64 {
    score_item(in_both(backpack))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        assert_eq!('p', in_both("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!('L', in_both("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!('P', in_both("PmmdzqPrVvPwwTWBwg"));
        assert_eq!('v', in_both("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!('t', in_both("ttgJtRGJQctTZtZT"));
        assert_eq!('s', in_both("CrZsJsPPZsGzwwsLwLmpwMDw"));

        assert_eq!(16, score_item('p'));
        assert_eq!(38, score_item('L'));
        assert_eq!(42, score_item('P'));
        assert_eq!(22, score_item('v'));
        assert_eq!(20, score_item('t'));
        assert_eq!(19, score_item('s'));

        assert_eq!(Solution::Integer(157), part_a("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn test_b() {
        assert_eq!(
            'r',
            in_three(
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg",
            )
        );
        assert_eq!(
            'Z',
            in_three(
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw",
            )
        );
        assert_eq!(Solution::Integer(70), part_b("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }
}
