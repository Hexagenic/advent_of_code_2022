use pathfinding::prelude::astar;

use crate::solutions::Solution;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Pos(i32, i32);

impl Pos {
    const fn distance(&self, other: &Self) -> u32 {
        (self.0.abs_diff(other.0) + self.1.abs_diff(other.1)) as u32
    }
}

#[derive(Eq, PartialEq, Debug)]
enum Square {
    Start,
    End,
    Height(i32),
}

struct Mountain {
    width: i32,
    height: i32,
    squares: Vec<Vec<Square>>,
}

impl Mountain {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    fn parse(file: &str) -> Self {
        let squares: Vec<Vec<Square>> = file
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        'S' => Square::Start,
                        'E' => Square::End,
                        'a'..='z' => Square::Height((c as i32) - ('a' as i32)),
                        _ => panic!("Unknown char {}", c),
                    })
                    .collect()
            })
            .collect();

        let width = squares[0].len() as i32;
        let height = squares.len() as i32;

        Self {
            width,
            height,
            squares,
        }
    }

    #[allow(clippy::cast_sign_loss)]
    fn height(&self, pos: &Pos) -> i32 {
        match self.squares[pos.1 as usize].get(pos.0 as usize) {
            Some(Square::End) => 25,
            Some(Square::Start) => 0,
            Some(Square::Height(h)) => *h,
            _ => panic!("Invalid index for height"),
        }
    }

    const fn can_go(from: i32, to: i32) -> bool {
        to <= from || from.abs_diff(to) <= 1
    }

    const fn can_go_back(from: i32, to: i32) -> bool {
        to > from || from.abs_diff(to) <= 1
    }

    fn path(&self) -> u32 {
        let start = self.find(&Square::Start);
        let end = self.find(&Square::End);
        astar(
            &start,
            |p| self.successors(p, Self::can_go),
            |p| p.distance(&end),
            |p| p == &end,
        )
        .expect("No path found")
        .1
    }

    fn distance_to_zero(&self, pos: &Pos) -> u32 {
        let mut min_dist = 0;

        for x in 0..self.width {
            for y in 0..self.height {
                let search_pos = Pos(x, y);
                if self.height(&search_pos) == 0 {
                    min_dist = min_dist.min(search_pos.distance(pos));
                }
            }
        }

        min_dist
    }

    fn path_down(&self) -> u32 {
        let start = self.find(&Square::End);
        astar(
            &start,
            |p| self.successors(p, Self::can_go_back),
            |p| self.distance_to_zero(p),
            |p| self.height(p) == 0,
        )
        .expect("No path found")
        .1
    }

    fn successors<P>(&self, pos: &Pos, can_go: P) -> Vec<(Pos, u32)>
    where
        P: Fn(i32, i32) -> bool,
    {
        let mut succ: Vec<(Pos, u32)> = vec![];
        let height = self.height(pos);

        if pos.0 > 0 {
            let new_pos = Pos(pos.0 - 1, pos.1);
            if can_go(height, self.height(&new_pos)) {
                succ.push((new_pos, 1));
            }
        }
        if pos.1 > 0 {
            let new_pos = Pos(pos.0, pos.1 - 1);
            if can_go(height, self.height(&new_pos)) {
                succ.push((new_pos, 1));
            }
        }
        if pos.0 < self.width - 1 {
            let new_pos = Pos(pos.0 + 1, pos.1);
            if can_go(height, self.height(&new_pos)) {
                succ.push((new_pos, 1));
            }
        }
        if pos.1 < self.height - 1 {
            let new_pos = Pos(pos.0, pos.1 + 1);
            if can_go(height, self.height(&new_pos)) {
                succ.push((new_pos, 1));
            }
        }

        succ
    }

    #[allow(clippy::cast_sign_loss)]
    fn find(&self, square: &Square) -> Pos {
        for x in 0..self.width {
            for y in 0..self.height {
                if &self.squares[y as usize][x as usize] == square {
                    return Pos(x, y);
                }
            }
        }

        panic!("Oh no");
    }
}

#[allow(clippy::cast_lossless)]
pub fn part_a(file: &str) -> Solution {
    Solution::Integer(Mountain::parse(file).path() as i64)
}

#[allow(clippy::cast_lossless)]
pub fn part_b(file: &str) -> Solution {
    Solution::Integer(Mountain::parse(file).path_down() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_data() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day12_test.txt");
        std::fs::read_to_string(&path).unwrap()
    }

    #[test]
    fn test_parse() {
        let mountain = Mountain::parse(&read_test_data());

        assert_eq!(40, mountain.width * mountain.height);

        assert_eq!(0, mountain.height(&Pos(0, 0)));
        assert_eq!(8, mountain.height(&Pos(7, 4)));
    }

    #[test]
    fn test_path() {
        let mountain = Mountain::parse(&read_test_data());

        assert_eq!(31, mountain.path());
    }

    #[test]
    fn test_path_down() {
        let mountain = Mountain::parse(&read_test_data());

        assert_eq!(29, mountain.path_down());
    }
}
