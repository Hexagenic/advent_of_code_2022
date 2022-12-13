use std::cmp::Ordering;
use std::iter::Peekable;

use crate::solutions::Solution;

#[derive(Eq, PartialEq)]
enum Value {
    Constant(i32),
    List(Vec<Self>),
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        check_pair((self, other))
    }
}

fn parse_file(file: &str) -> impl Iterator<Item = (Value, Value)> + '_ {
    file.split("\n\n").map(|p| {
        let (a, b) = p.split_once('\n').unwrap();

        (parse_line(a), parse_line(b))
    })
}

fn parse_line(line: &str) -> Value {
    let mut it = line.chars().peekable();

    parse_list(&mut it)
}

fn parse_list<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Value {
    let mut list: Vec<Value> = vec![];

    it.next(); // skip '['

    while let Some(&c) = it.peek() {
        match c {
            '[' => list.push(parse_list(it)),
            ']' => {
                it.next(); // skip ']';
                break;
            }
            '0'..='9' => list.push(parse_number(it)),
            ',' => {
                it.next(); // skip ','
            }
            c => panic!("Unknown! {}", c),
        }
    }

    Value::List(list)
}

fn parse_number<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> Value {
    let mut str = String::new();

    while let Some(&c) = it.peek() {
        if !c.is_ascii_digit() {
            return Value::Constant(str.parse().unwrap());
        }

        str.push(c);
        it.next();
    }

    Value::Constant(str.parse().unwrap())
}

fn check_lists(a: &Vec<Value>, b: &Vec<Value>) -> Ordering {
    for (a, b) in a.iter().zip(b.iter()) {
        let ord = check_pair((a, b));
        if ord != Ordering::Equal {
            return ord;
        }
    }

    if a.len() != b.len() {
        return a.len().cmp(&b.len());
    }

    Ordering::Equal
}

fn check_pair(pair: (&Value, &Value)) -> Ordering {
    match pair {
        (Value::Constant(a), Value::Constant(b)) => a.cmp(b),
        (Value::Constant(a), b @ Value::List(_)) => {
            check_pair((&Value::List(vec![Value::Constant(*a)]), b))
        }
        (a @ Value::List(_), Value::Constant(b)) => {
            check_pair((a, &Value::List(vec![Value::Constant(*b)])))
        }
        (Value::List(a), Value::List(b)) => check_lists(a, b),
    }
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(
        parse_file(file)
            .map(|(a, b)| check_pair((&a, &b)))
            .enumerate()
            .filter(|(_, v)| *v == Ordering::Less)
            .map(|(i, _)| i as i64 + 1)
            .sum(),
    )
}

pub fn part_b(file: &str) -> Solution {
    let mut lines = file
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(parse_line(l))
            }
        })
        .chain(vec![
            Value::List(vec![Value::Constant(2)]),
            Value::List(vec![Value::Constant(6)]),
        ])
        .collect::<Vec<Value>>();

    lines.sort_unstable();

    let first_index = lines
        .iter()
        .position(|p| *p == Value::List(vec![Value::Constant(2)]))
        .unwrap()
        + 1;
    let second_index = lines
        .iter()
        .position(|p| *p == Value::List(vec![Value::Constant(6)]))
        .unwrap()
        + 1;

    Solution::Integer((first_index * second_index) as i64)
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use super::*;

    fn read_test_data() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day13_test.txt");
        std::fs::read_to_string(&path).unwrap()
    }

    impl fmt::Debug for Value {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Constant(v) => write!(f, "{}", v),
                Self::List(v) => write!(f, "{:?}", v),
            }
        }
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(
            Value::Constant(1),
            parse_number(&mut "1".chars().peekable())
        );

        assert_eq!(
            Value::Constant(100),
            parse_number(&mut "100".chars().peekable())
        );

        assert_eq!(
            Value::Constant(50),
            parse_number(&mut "50abc".chars().peekable())
        );
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(Value::List(vec![]), parse_line("[]"));

        assert_eq!(
            Value::List(vec![Value::Constant(1), Value::Constant(2)]),
            parse_line("[1,2]")
        );

        assert_eq!(
            Value::List(vec![
                Value::Constant(1),
                Value::List(vec![Value::Constant(2), Value::Constant(3)]),
            ]),
            parse_line("[1,[2,3]]")
        );
    }

    #[test]
    fn test_part_a() {
        assert_eq!(Solution::Integer(13), part_a(&read_test_data()));
    }

    #[test]
    fn test_part_b() {
        assert_eq!(Solution::Integer(140), part_b(&read_test_data()));
    }
}
