use std::collections::HashMap;

use crate::solutions::Solution;

#[derive(Eq, PartialEq, Debug)]
enum Command {
    CdRoot,
    CdUp,
    CdDir { directory: String },
    Ls { size: i64 },
}

fn parse_commands(file: &str) -> impl Iterator<Item = Command> + '_ {
    file.split('$').skip(1).map(|c| {
        let cmd = c.trim();

        return if cmd.starts_with('c') {
            let p = cmd.split_once(' ').unwrap().1;

            if p.starts_with('/') {
                Command::CdRoot
            } else if p.starts_with('.') {
                Command::CdUp
            } else {
                Command::CdDir {
                    directory: p.to_string(),
                }
            }
        } else {
            let size = cmd
                .lines()
                .skip(1)
                .filter(|l| !l.starts_with('d'))
                .map(|l| l.split_once(' ').unwrap().0.parse::<i64>().unwrap())
                .sum();

            Command::Ls { size }
        };
    })
}

fn execute_commands(commands: impl Iterator<Item = Command>) -> HashMap<String, i64> {
    let mut path = "/".to_string();

    let mut tree: HashMap<String, i64> = HashMap::new();

    for command in commands {
        match command {
            Command::CdRoot => {
                path.drain(..1);
            }
            Command::CdDir { directory } => {
                path.push_str(&directory);
                path.push('/');
            }
            Command::CdUp => {
                path.drain(..path.rfind('/').unwrap());
            }
            Command::Ls { size } => {
                if tree.contains_key(&path) {
                    continue;
                }
                tree.insert(path.clone(), size);
            }
        }
    }

    tree
}

fn tree_collapse(tree: &HashMap<String, i64>) -> (i64, impl Iterator<Item = i64> + '_) {
    let root = tree.values().sum();

    let iter = tree.iter().map(|(k1, v1)| {
        tree.iter()
            .map(|(k2, v2)| {
                if k2 == k1 {
                    *v1
                } else if k2.starts_with(k1) {
                    *v2
                } else {
                    0
                }
            })
            .sum()
    });

    (root, iter)
}

pub fn part_a(file: &str) -> Solution {
    let tree = execute_commands(parse_commands(file));

    let (_, iter) = tree_collapse(&tree);

    Solution::Integer(iter.filter(|v| *v <= 100_000).sum())
}

pub fn part_b(file: &str) -> Solution {
    let tree = execute_commands(parse_commands(file));
    let (used, iter) = tree_collapse(&tree);

    let total: i64 = 70_000_000;
    let required: i64 = 30_000_000;
    let to_free = required - (total - used);

    Solution::Integer(iter.filter(|v| v >= &to_free).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_test_data() -> String {
        let path = std::env::current_dir()
            .unwrap()
            .join("input")
            .join("day07_test.txt");
        std::fs::read_to_string(&path).unwrap()
    }

    #[test]
    fn test_parse_commands() {
        let test_data = read_test_data();

        assert_eq!(
            parse_commands(test_data.as_str()).collect::<Vec<_>>(),
            vec![
                Command::CdRoot,
                Command::Ls { size: 23_352_670 },
                Command::CdDir {
                    directory: "a".to_string()
                },
                Command::Ls { size: 94269 },
                Command::CdDir {
                    directory: "e".to_string()
                },
                Command::Ls { size: 584 },
                Command::CdUp,
                Command::CdUp,
                Command::CdDir {
                    directory: "d".to_string()
                },
                Command::Ls { size: 24_933_642 },
            ]
        );
    }

    #[test]
    fn test_part_a() {
        let test_data = read_test_data();
        assert_eq!(Solution::Integer(95437), part_a(&test_data));
    }

    #[test]
    fn test_part_b() {
        let test_data = read_test_data();
        assert_eq!(Solution::Integer(24_933_642), part_b(&test_data));
    }
}
