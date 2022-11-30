use crate::solutions::Solution;

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(file.len() as i64)
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(file.len() as i64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp_test() {
        assert_eq!(Solution::Integer(3), part_a("aaa"));
    }
}