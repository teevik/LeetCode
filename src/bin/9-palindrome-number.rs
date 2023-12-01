struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x.is_negative() {
            return false;
        }

        let x = x as u32;
        // dbg!(x);

        let amount_of_digits = (x as f64).log10() as u32 + 1;
        // dbg!(amount_of_digits);

        for i in 0..(amount_of_digits / 2) {
            let first_digit = (x / 10_u32.pow(amount_of_digits - i - 1)) % 10;
            let last_digit = (x / 10_u32.pow(i)) % 10;

            // dbg!(first_digit);
            // dbg!(last_digit);

            if first_digit != last_digit {
                return false;
            }
        }

        true
    }
}

fn main() {
    assert_eq!(Solution::is_palindrome(121), true);
    assert_eq!(Solution::is_palindrome(10), false);
    assert_eq!(Solution::is_palindrome(1000021), false);
    assert_eq!(Solution::is_palindrome(1001), true);
}
