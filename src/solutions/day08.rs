use std::collections::HashSet;

use crate::solutions::Solution;

#[allow(clippy::cast_possible_truncation)]
fn parse_square(file: &str) -> Vec<i8> {
    file.chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i8))
        .collect()
}

fn count_left(forest: &[i8], width: usize, seen: &mut HashSet<(usize, usize)>) {
    for y in 0..width {
        let mut talest: i8 = -1;
        for x in 0..width {
            let tree = forest[x + y * width];

            if tree > talest {
                talest = tree;
                seen.insert((x, y));
            }
        }
    }
}

fn count_right(forest: &[i8], width: usize, seen: &mut HashSet<(usize, usize)>) {
    for y in 0..width {
        let mut talest: i8 = -1;
        for x in (0..width).rev() {
            let tree = forest[x + y * width];

            if tree > talest {
                talest = tree;
                seen.insert((x, y));
            }
        }
    }
}

fn count_top(forest: &[i8], width: usize, seen: &mut HashSet<(usize, usize)>) {
    for x in 0..width {
        let mut talest: i8 = -1;
        for y in 0..width {
            let tree = forest[x + y * width];

            if tree > talest {
                talest = tree;
                seen.insert((x, y));
            }
        }
    }
}

fn count_bottom(forest: &[i8], width: usize, seen: &mut HashSet<(usize, usize)>) {
    for x in 0..width {
        let mut talest: i8 = -1;
        for y in (0..width).rev() {
            let tree = forest[x + y * width];

            if tree > talest {
                talest = tree;
                seen.insert((x, y));
            }
        }
    }
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn part_a(file: &str) -> Solution {
    let forest = parse_square(file);
    let width = f64::sqrt(forest.len() as f64) as usize;
    let mut seen = HashSet::new();
    count_left(&forest, width, &mut seen);
    count_right(&forest, width, &mut seen);
    count_top(&forest, width, &mut seen);
    count_bottom(&forest, width, &mut seen);

    Solution::Integer(seen.len() as i64)
}

fn scenic_right(forest: &[i8], width: usize, start: &(usize, usize)) -> usize {
    let start_height = forest[start.0 + start.1 * width];
    let y = start.1;

    for x in (start.0 + 1)..width {
        let tree = forest[x + y * width];

        if tree >= start_height {
            return x - start.0;
        }
    }
    width - start.0 - 1
}

fn scenic_left(forest: &[i8], width: usize, start: &(usize, usize)) -> usize {
    let start_height = forest[start.0 + start.1 * width];
    let y = start.1;

    for x in (0..start.0).rev() {
        let tree = forest[x + y * width];

        if tree >= start_height {
            return start.0 - x;
        }
    }
    start.0
}

fn scenic_down(forest: &[i8], width: usize, start: &(usize, usize)) -> usize {
    let start_height = forest[start.0 + start.1 * width];
    let x = start.0;

    for y in (start.1 + 1)..width {
        let tree = forest[x + y * width];

        if tree >= start_height {
            return y - start.1;
        }
    }
    width - start.1 - 1
}

fn scenic_up(forest: &[i8], width: usize, start: &(usize, usize)) -> usize {
    let start_height = forest[start.0 + start.1 * width];
    let x = start.0;

    for y in (0..start.1).rev() {
        let tree = forest[x + y * width];

        if tree >= start_height {
            return start.1 - y;
        }
    }
    start.1
}

fn calc_scene(forest: &[i8], width: usize, tree: &(usize, usize)) -> usize {
    if tree.0 == 0 || tree.1 == 0 || tree.0 == width - 1 || tree.1 == width - 1 {
        return 0;
    }

    scenic_left(forest, width, tree)
        * scenic_right(forest, width, tree)
        * scenic_down(forest, width, tree)
        * scenic_up(forest, width, tree)
}

#[allow(clippy::cast_sign_loss)]
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_precision_loss)]
pub fn part_b(file: &str) -> Solution {
    let forest = parse_square(file);
    let width = f64::sqrt(forest.len() as f64) as usize;

    let mut max = 0;

    for x in 1..(width - 1) {
        for y in 1..(width - 1) {
            let scene = calc_scene(&forest, width, &(x, y));

            if scene > max {
                max = scene;
            }
        }
    }

    Solution::Integer(max as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_data() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day08_test.txt");
        std::fs::read_to_string(&path).unwrap()
    }

    #[test]
    fn test_parse() {
        assert_eq!(25, parse_square(&read_test_data()).len())
    }

    #[test]
    fn test_left() {
        let forest = parse_square(&read_test_data());
        let mut seen = HashSet::new();
        count_left(&forest, 5, &mut seen);
        assert_eq!(11, seen.len());
    }

    #[test]
    fn test_right() {
        let forest = parse_square(&read_test_data());
        let mut seen = HashSet::new();
        count_right(&forest, 5, &mut seen);
        assert_eq!(11, seen.len());
    }

    #[test]
    fn test_top() {
        let forest = parse_square(&read_test_data());
        let mut seen = HashSet::new();
        count_top(&forest, 5, &mut seen);
        assert_eq!(10, seen.len());
    }

    #[test]
    fn test_bottom() {
        let forest = parse_square(&read_test_data());
        let mut seen = HashSet::new();
        count_bottom(&forest, 5, &mut seen);
        assert_eq!(8, seen.len());
    }

    #[test]
    fn test_part_a() {
        assert_eq!(Solution::Integer(21), part_a(&read_test_data()));
    }

    #[test]
    fn test_scenic() {
        let forest = parse_square(&read_test_data());

        assert_eq!(1, scenic_up(&forest, 5, &(2, 1)));
        assert_eq!(1, scenic_left(&forest, 5, &(2, 1)));
        assert_eq!(2, scenic_right(&forest, 5, &(2, 1)));
        assert_eq!(2, scenic_down(&forest, 5, &(2, 1)));
        assert_eq!(4, calc_scene(&forest, 5, &(2, 1)));

        assert_eq!(2, scenic_up(&forest, 5, &(2, 3)));
        assert_eq!(2, scenic_left(&forest, 5, &(2, 3)));
        assert_eq!(2, scenic_right(&forest, 5, &(2, 3)));
        assert_eq!(1, scenic_down(&forest, 5, &(2, 3)));
        assert_eq!(8, calc_scene(&forest, 5, &(2, 3)));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(Solution::Integer(8), part_b(&read_test_data()));
    }
}
