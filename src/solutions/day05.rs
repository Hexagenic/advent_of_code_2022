use regex::Regex;

use crate::solutions::Solution;

#[derive(Eq, PartialEq, Debug)]
struct Move {
    count: u8,
    from: u8,
    to: u8,
}

type Stacks = Vec<Vec<char>>;

fn parse_stacks(input: &str) -> Stacks {
    let mut result: Vec<Vec<char>> = Vec::new();

    for line in input.lines().rev().skip(1).map(parse_stack_line) {
        for (i, c) in line.enumerate() {
            if let Some(c) = c {
                if result.len() <= i {
                    result.resize(i + 1, vec![]);
                }

                result[i].push(c);
            }
        }
    }

    result
}

fn parse_stack_line(line: &str) -> impl Iterator<Item = Option<char>> + '_ {
    line.chars().skip(1).step_by(4).map(|c| match c {
        ' ' => None,
        a => Some(a),
    })
}

fn parse_move(line: &str) -> Move {
    let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
    let m = re.captures(line).unwrap();

    let count = m[1].parse().unwrap();
    let from = m[2].parse().unwrap();
    let to = m[3].parse().unwrap();

    Move { count, from, to }
}

fn parse_moves(lines: &str) -> impl Iterator<Item = Move> + '_ {
    lines.lines().skip(2).map(parse_move)
}

fn parse_input(file: &str) -> (Stacks, impl Iterator<Item = Move> + '_) {
    let parts = file.split_at(file.find("\n\n").unwrap());

    (parse_stacks(parts.0), parse_moves(parts.1))
}

pub fn part_a(file: &str) -> Solution {
    let (mut stack, moves) = parse_input(file);

    for m in moves {
        for _ in 0..m.count {
            let val = stack[m.from as usize - 1].pop().unwrap();
            stack[m.to as usize - 1].push(val);
        }
    }

    let result = stack.iter().filter_map(|k| k.last()).collect();

    Solution::String(result)
}

pub fn part_b(file: &str) -> Solution {
    let (mut stack, moves) = parse_input(file);

    for m in moves {
        let original_stack = &stack[m.from as usize - 1];
        let index = original_stack.len() - (m.count as usize);

        let a = original_stack[0..index].to_vec();
        let mut b = original_stack[index..original_stack.len()].to_vec();

        stack[m.from as usize - 1] = a;
        stack[m.to as usize - 1].append(&mut b);
    }

    let result = stack.iter().filter_map(|k| k.last()).collect();

    Solution::String(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2";

    #[test]
    fn test_parse_stacks() {
        assert_eq!(
            vec![Some('W'), Some('L'), None, Some('F')],
            parse_stack_line("[W] [L]     [F]").collect::<Vec<_>>()
        );

        assert_eq!(
            vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
            parse_stacks("    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ")
        );
    }

    #[test]
    fn test_parse_move() {
        assert_eq!(
            Move {
                count: 1,
                from: 2,
                to: 1,
            },
            parse_move("move 1 from 2 to 1")
        );
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            vec![
                Move {
                    count: 1,
                    from: 2,
                    to: 1,
                },
                Move {
                    count: 3,
                    from: 1,
                    to: 3,
                },
                Move {
                    count: 2,
                    from: 2,
                    to: 1,
                },
                Move {
                    count: 1,
                    from: 1,
                    to: 2,
                },
            ],
            parse_input(TEST_INPUT).1.collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part_a() {
        assert_eq!(Solution::String("CMZ".to_string()), part_a(TEST_INPUT));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(Solution::String("MCD".to_string()), part_b(TEST_INPUT));
    }
}
