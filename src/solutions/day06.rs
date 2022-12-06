use crate::solutions::Solution;

fn parse_start_of_packet<const N: usize>(string: &str) -> i64 {
    let mut buf: [char; N] = ['\0'; N];
    let mut buf_index = 0;

    for (i, c) in string.chars().enumerate() {
        buf[buf_index] = c;
        buf_index = (buf_index + 1) % N;

        if all_distinct(&buf) {
            return i as i64 + 1;
        }
    }

    0
}

/// Use bit masking to detect if all chars are distinct. Only works for \[a-z\], as that fits inside `u32`
fn all_distinct(chars: &[char]) -> bool {
    let mut acc = 0;

    for c in chars {
        if c == &'\0' {
            return false;
        }

        let mask: u32 = 1 << (*c as i32 - 97);

        if acc & mask == mask {
            return false;
        }

        acc |= mask;
    }

    true
}

pub fn part_a(file: &str) -> Solution {
    Solution::Integer(parse_start_of_packet::<4>(file))
}

pub fn part_b(file: &str) -> Solution {
    Solution::Integer(parse_start_of_packet::<14>(file))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start_of_packet() {
        assert_eq!(
            5,
            parse_start_of_packet::<4>("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            6,
            parse_start_of_packet::<4>("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            10,
            parse_start_of_packet::<4>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            11,
            parse_start_of_packet::<4>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );

        assert_eq!(
            19,
            parse_start_of_packet::<14>("mjqjpqmgbljsphdztnvjfqwrcgsmlb")
        );
        assert_eq!(
            23,
            parse_start_of_packet::<14>("bvwbjplbgvbhsrlpgdmjqwftvncz")
        );
        assert_eq!(
            23,
            parse_start_of_packet::<14>("nppdvjthqldpwncqszvftbrmjlhg")
        );
        assert_eq!(
            29,
            parse_start_of_packet::<14>("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
        );
        assert_eq!(
            26,
            parse_start_of_packet::<14>("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
        );
    }
}
