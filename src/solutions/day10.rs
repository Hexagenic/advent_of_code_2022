use crate::solutions::Solution;

#[derive(PartialEq, Eq, Debug)]
enum Instruction {
    Noop,
    AddX(i64),
}

fn parse_instruction(file: &str) -> impl Iterator<Item = Instruction> + '_ {
    file.lines().map(|l| {
        if l.starts_with('n') {
            Instruction::Noop
        } else {
            Instruction::AddX(l.split_once(' ').unwrap().1.parse().unwrap())
        }
    })
}

fn to_state<I>(instructions: &mut I) -> impl Iterator<Item = (usize, i64)> + '_
where
    I: Iterator<Item = Instruction>,
{
    instructions
        .flat_map(|i| match i {
            Instruction::Noop => vec![0],
            Instruction::AddX(v) => vec![0, v],
        })
        .scan(1, |state, v| {
            *state += v;
            Some(*state)
        })
        .enumerate()
}

pub fn part_a(file: &str) -> Solution {
    let mut instructions = parse_instruction(file);
    let mut states = to_state(&mut instructions);

    let mut sum = 0;
    for i in [18, 39, 39, 39, 39, 39] {
        let (a, b) = states.nth(i).unwrap();
        sum += (a as i64 + 2) * b;
    }

    Solution::Integer(sum)
}

pub fn part_b(file: &str) -> Solution {
    let mut instructions = parse_instruction(file);
    let states = to_state(&mut instructions);

    let mut output = "\n".to_string();

    let mut prev: i64 = 1;

    for s in states {
        let pixel_pos = (s.0 % 40) as i64;

        let should_draw = (pixel_pos - prev).abs() <= 1;

        output += if should_draw { "#" } else { "." };

        if pixel_pos == 39 && s.0 != 0 {
            output += "\n";
        }

        prev = s.1;
    }
    Solution::String(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_data(file: u8) -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join(format!("day10_test{}.txt", file));
        std::fs::read_to_string(&path).unwrap()
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Instruction::Noop,
                Instruction::AddX(3),
                Instruction::AddX(-5)
            ],
            parse_instruction(&read_test_data(1)).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_acc() {
        assert_eq!(
            vec![(0, 1), (1, 1), (2, 4), (3, 4), (4, -1)],
            to_state(&mut parse_instruction(&read_test_data(1))).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_part_a() {
        assert_eq!(Solution::Integer(13140), part_a(&read_test_data(2)));
    }

    #[test]
    fn test_part_b() {
        let expected = "\n##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
        assert_eq!(
            Solution::String(expected.to_string()),
            part_b(&read_test_data(2))
        );
    }
}
