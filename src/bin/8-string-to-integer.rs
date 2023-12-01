struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars().peekable();

        while chars.peek() == Some(&' ') {
            chars.next();
        }

        let is_positive = match chars.peek() {
            Some(&'+') => {
                chars.next();
                true
            }

            Some(&'-') => {
                chars.next();
                false
            }

            _ => true,
        };

        let digits = chars
            .take_while(|char| char.is_ascii_digit())
            .map(|digit| digit.to_digit(10).expect("Always ascii digit"))
            .collect::<Vec<_>>();

        let mut integer: i64 = 0;

        for (i, digit) in digits.into_iter().rev().enumerate() {
            integer = integer
                .saturating_add((digit as i64).saturating_mul(10_i64.saturating_pow(i as u32)))
        }

        let integer = if is_positive {
            integer
        } else {
            integer.saturating_neg()
        };

        if integer > i32::MAX as i64 {
            i32::MAX
        } else if integer < i32::MIN as i64 {
            i32::MIN
        } else {
            integer as i32
        }
    }
}

fn main() {
    assert_eq!(Solution::my_atoi("42".to_string()), 42);
    assert_eq!(Solution::my_atoi("   -42".to_string()), -42);
    assert_eq!(Solution::my_atoi("4193 with words".to_string()), 4193);
    assert_eq!(Solution::my_atoi("words and 987".to_string()), 0);
    assert_eq!(Solution::my_atoi("-91283472332".to_string()), -2147483648);
}
