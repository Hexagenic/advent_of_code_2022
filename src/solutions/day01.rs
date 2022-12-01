use crate::solutions::Solution;

pub fn part_a(file: &str) -> Solution {
    let nums = string_to_numbers(file);

    let mut highest: i64 = 0;
    let mut current: i64 = 0;

    for num in nums {
        if let Some(n) = num {
            current += n;
        } else {
            if current > highest {
                highest = current;
            }
            current = 0;
        }
    }

    if current > highest {
        highest = current;
    }

    Solution::Integer(highest)
}

pub fn part_b(file: &str) -> Solution {
    let nums = string_to_numbers(file);

    let mut elfs = vec![];

    let mut current: i64 = 0;

    for num in nums {
        if let Some(n) = num {
            current += n;
        } else {
            elfs.push(current);
            current = 0;
        }
    }

    elfs.push(current);

    elfs.sort_unstable();

    Solution::Integer(elfs.iter().rev().take(3).sum())
}

fn string_to_numbers(file: &str) -> impl Iterator<Item = Option<i64>> + '_ {
    file.lines().map(|l| l.parse().ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_string() {
        assert_eq!(
            vec![Some(1), Some(2), Some(3), None, Some(1)],
            string_to_numbers("1\n2\n3\n\n1").collect::<Vec<Option<i64>>>()
        );
    }

    #[test]
    fn test_a() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(Solution::Integer(24000), part_a(input));
    }

    #[test]
    fn test_b() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        assert_eq!(Solution::Integer(45000), part_b(input));
    }
}
