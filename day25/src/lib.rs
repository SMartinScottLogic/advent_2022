use std::io::{BufRead, BufReader};

pub type ResultType = String;

#[derive(Debug, Default)]
pub struct Solution {
    input: Vec<String>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut answer = 0;
        for i in &self.input {
            answer += Self::from_snafu(i);
        }
        Ok(Self::to_snafu(answer))
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        Ok("".to_string())
    }
}

impl Solution {
    fn to_snafu(i: i64) -> String {
        let mut i = i;
        let mut result = String::new();

        let mut carry = 0;
        while i != 0 {
            let r = i % 5;
            let (t_carry, r) = match r + carry {
                0 => (0, '0'),
                1 => (0, '1'),
                2 => (0, '2'),
                3 => (1, '='),
                4 => (1, '-'),
                5 => (1, '0'),
                v => unreachable!("Unexpected {v}"),
            };
            carry = t_carry;
            i /= 5;
            result.push(r);
        }
        match carry {
            1 => result.push('1'),
            0 => {}
            _ => unreachable!(),
        }
        result.chars().rev().collect()
    }

    fn from_snafu(s: &str) -> i64 {
        let mut a = 0;
        for c in s.chars() {
            a *= 5;
            match c {
                '0' => {}
                '1' => a += 1,
                '2' => a += 2,
                '-' => a += -1,
                '=' => a += -2,
                _ => unreachable!(),
            }
        }
        a
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            solution.input.push(line);
        }
        Ok(solution)
    }
}

#[cfg(test)]
mod test {
    use crate::Solution;

    #[test]
    fn from_snafu_test() {
        assert_eq!(1, Solution::from_snafu("1"));
        assert_eq!(2, Solution::from_snafu("2"));
        assert_eq!(3, Solution::from_snafu("1="));
        assert_eq!(4, Solution::from_snafu("1-"));
        assert_eq!(5, Solution::from_snafu("10"));
        assert_eq!(6, Solution::from_snafu("11"));
        assert_eq!(7, Solution::from_snafu("12"));
        assert_eq!(8, Solution::from_snafu("2="));
        assert_eq!(9, Solution::from_snafu("2-"));
        assert_eq!(10, Solution::from_snafu("20"));
        assert_eq!(15, Solution::from_snafu("1=0"));
        assert_eq!(20, Solution::from_snafu("1-0"));
        assert_eq!(2022, Solution::from_snafu("1=11-2"));
        assert_eq!(12345, Solution::from_snafu("1-0---0"));
        assert_eq!(314159265, Solution::from_snafu("1121-1110-1=0"));
    }

    #[test]
    fn to_snafu_test() {
        assert_eq!(Solution::to_snafu(1), "1");
        assert_eq!(Solution::to_snafu(2), "2");
        assert_eq!(Solution::to_snafu(3), "1=");
        assert_eq!(Solution::to_snafu(4), "1-");
        assert_eq!(Solution::to_snafu(5), "10");
        assert_eq!(Solution::to_snafu(6), "11");
        assert_eq!(Solution::to_snafu(7), "12");
        assert_eq!(Solution::to_snafu(8), "2=");
        assert_eq!(Solution::to_snafu(9), "2-");
        assert_eq!(Solution::to_snafu(10), "20");
        assert_eq!(Solution::to_snafu(15), "1=0");
        assert_eq!(Solution::to_snafu(20), "1-0");
        assert_eq!(Solution::to_snafu(2022), "1=11-2");
        assert_eq!(Solution::to_snafu(12345), "1-0---0");
        assert_eq!(Solution::to_snafu(314159265), "1121-1110-1=0");
    }
}
