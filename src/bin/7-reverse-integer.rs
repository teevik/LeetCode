struct Solution;

fn reverse_impl(x: i32) -> Option<i32> {
    let is_negative = x.is_negative();
    let mut x = x.checked_abs()?;
    let mut reversed: i32 = 0;

    let amount_of_digits = (x as f64).log10().floor() as u32 + 1;

    for i in 0..amount_of_digits {
        let digit = x % 10;
        reversed =
            reversed.checked_add(digit.checked_mul(10_i32.pow(amount_of_digits - i - 1))?)?;

        x /= 10;
    }

    let result = if is_negative { -reversed } else { reversed };

    Some(result)
}

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        reverse_impl(x).unwrap_or(0)
    }
}

fn main() {
    assert_eq!(Solution::reverse(123), 321);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-2147483648), 0);
    assert_eq!(Solution::reverse(1563847412), 0);
}
