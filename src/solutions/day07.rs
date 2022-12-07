use std::collections::HashMap;

use crate::solutions::Solution;

#[derive(Eq, PartialEq, Debug)]
enum Command {
    CdRoot,
    CdUp,
    CdDir { directory: String },
    Ls { size: i64 },
}

fn parse_commands(file: &str) -> Vec<Command> {
    file.split('$')
        .skip(1)
        .map(|c| {
            let cmd = c.trim();

            return if cmd.starts_with('c') {
                let p = cmd.split(' ').nth(1).unwrap();

                match p {
                    "/" => Command::CdRoot,
                    ".." => Command::CdUp,
                    s => Command::CdDir {
                        directory: s.to_string(),
                    },
                }
            } else {
                let mut dirs: Vec<String> = vec![];
                let mut size = 0;

                for l in cmd.lines().skip(1) {
                    if l.starts_with('d') {
                        dirs.push(l.split_at(4).1.to_string());
                    } else {
                        size += l.split(' ').next().unwrap().parse::<i64>().unwrap();
                    }
                }

                Command::Ls { size }
            };
        })
        .collect()
}

fn execute_commands(commands: &[Command]) -> HashMap<String, i64> {
    let mut path = "/".to_string();

    let mut tree: HashMap<String, i64> = HashMap::new();

    for command in commands {
        match command {
            Command::CdRoot => {
                path = "/".to_string();
            }
            Command::CdDir { directory } => {
                if path != "/" {
                    path.push('/');
                }

                path.push_str(directory);
            }
            Command::CdUp => {
                path = path.rsplit_once('/').unwrap().0.to_string();
            }
            Command::Ls { size } => {
                if tree.contains_key(&path) {
                    continue;
                }
                tree.insert(path.clone(), *size);
            }
        }
    }

    let mut cumulative_tree = tree.clone();

    for (k1, v1) in &tree {
        for (k, v) in &mut cumulative_tree {
            if k != k1 && k1.starts_with(k) {
                *v += v1;
            }
        }
    }

    cumulative_tree
}

pub fn part_a(file: &str) -> Solution {
    let tree = execute_commands(&parse_commands(file));
    Solution::Integer(tree.iter().map(|(_, v)| v).filter(|v| **v <= 100_000).sum())
}

pub fn part_b(file: &str) -> Solution {
    let tree = execute_commands(&parse_commands(file));

    let total: i64 = 70_000_000;
    let required: i64 = 30_000_000;
    let used: i64 = *tree.get("/").unwrap();
    let to_free = required - (total - used);

    Solution::Integer(*tree.values().filter(|v| v >= &&to_free).min().unwrap())
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
            parse_commands(test_data.as_str()),
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
