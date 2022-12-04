use std::ops::RangeInclusive;

use crate::solutions::Solution;

fn contains(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    (ranges.0.contains(ranges.1.start()) && ranges.0.contains(ranges.1.end()))
        || (ranges.1.contains(ranges.0.start()) && ranges.1.contains(ranges.0.end()))
}

fn overlaps(ranges: &(RangeInclusive<u32>, RangeInclusive<u32>)) -> bool {
    ranges.0.contains(ranges.1.start()) || ranges.1.contains(ranges.0.start())
}

fn parse_line(line: &str) -> (RangeInclusive<u32>, RangeInclusive<u32>) {
    let mut parts = line.split(',');

    let first = parse_range(parts.next().unwrap());
    let second = parse_range(parts.next().unwrap());

    (first, second)
}

fn parse_range(line: &str) -> RangeInclusive<u32> {
    let mut parts = line.split('-');

    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();

    start..=end
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(file.lines().map(parse_line).filter(contains).count() as i64)
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(file.lines().map(parse_line).filter(overlaps).count() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        assert!(!contains(&(2..=4, 6..=8)));
        assert!(!contains(&(2..=3, 4..=5)));
        assert!(!contains(&(5..=7, 7..=9)));
        assert!(contains(&(2..=8, 3..=7)));
        assert!(contains(&(6..=6, 4..=6)));
        assert!(!contains(&(2..=6, 4..=8)));
    }

    #[test]
    fn test_overlaps() {
        assert!(!overlaps(&(2..=4, 6..=8)));
        assert!(!overlaps(&(2..=3, 4..=5)));
        assert!(overlaps(&(5..=7, 7..=9)));
        assert!(overlaps(&(2..=8, 3..=7)));
        assert!(overlaps(&(6..=6, 4..=6)));
        assert!(overlaps(&(2..=6, 4..=8)));
    }

    #[test]
    fn test_parse_line() {
        assert_eq!((2..=4, 6..=8), parse_line("2-4,6-8"));
        assert_eq!((54..=54, 55..=68), parse_line("54-54,55-68"));
    }

    #[test]
    fn test_a() {
        assert_eq!(
            Solution::Integer(2),
            part_a("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n")
        );
    }

    #[test]
    fn test_b() {
        assert_eq!(
            Solution::Integer(4),
            part_b("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n")
        );
    }
}
