use crate::solutions::Solution;
use std::collections::HashSet;

type Move = (char, i32);
type Pos = (i32, i32);

fn parse_lines(file: &str) -> impl Iterator<Item = Move> + '_ {
    file.lines().map(|line| {
        let dir = line.chars().next().unwrap();
        let distance = line[2..].parse().unwrap();

        (dir, distance)
    })
}

fn move_rope<I>(moves: I) -> usize
where
    I: Iterator<Item = Move>,
{
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut positions: HashSet<Pos> = HashSet::new();
    positions.insert(tail);

    for (d, l) in moves {
        for _ in 0..l {
            match d {
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                'U' => head.1 -= 1,
                'D' => head.1 += 1,
                _ => panic!("Unknown move"),
            }

            tail = follow_move(tail, head);
            positions.insert(tail);
        }
    }

    positions.len()
}

fn move_long_rope<I, const N: usize>(moves: I) -> usize
where
    I: Iterator<Item = Move>,
{
    let mut snake: [Pos; N] = [(0, 0); N];

    let mut positions: HashSet<Pos> = HashSet::new();
    positions.insert((0, 0));

    for (d, l) in moves {
        for _ in 0..l {
            match d {
                'L' => snake[0].0 -= 1,
                'R' => snake[0].0 += 1,
                'U' => snake[0].1 -= 1,
                'D' => snake[0].1 += 1,
                _ => panic!("Unknown move"),
            }

            for i in 1..N {
                snake[i] = follow_move(snake[i], snake[i - 1]);
            }

            positions.insert(snake[N - 1]);
        }
    }

    positions.len()
}

const fn follow_move(current: Pos, ahead: Pos) -> Pos {
    let x_diff = current.0 - ahead.0;
    let y_diff = current.1 - ahead.1;

    if x_diff.abs() <= 1 && y_diff.abs() <= 1 {
        return current;
    }

    (current.0 - x_diff.signum(), current.1 - y_diff.signum())
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(move_rope(parse_lines(file)) as i64)
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(move_long_rope::<_, 10>(parse_lines(file)) as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![('D', 10), ('U', 5), ('L', 3)],
            parse_lines("D 10\nU 5\nL 3").collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part_a() {
        let moves = parse_lines("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
        assert_eq!(13, move_rope(moves));
    }

    #[test]
    fn test_part_b() {
        let moves_short = parse_lines("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2");
        assert_eq!(1, move_long_rope::<_, 10>(moves_short));

        let moves_long = parse_lines("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20");
        assert_eq!(36, move_long_rope::<_, 10>(moves_long));
    }
}
