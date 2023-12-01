struct Solution;

fn parse_next_roman_number(roman: &[u8]) -> (&[u8], Option<i32>) {
    if roman.is_empty() {
        return (roman, None);
    }

    // Last character
    let current_character = *roman.last().expect("Never empty");

    // Second last character
    let prefix_character = roman.len().checked_sub(2).map(|index| roman[index]);

    let single = |number| (&roman[0..(roman.len() - 1)], Some(number));
    let double = |number| (&roman[0..(roman.len() - 2)], Some(number));

    match (current_character, prefix_character) {
        (b'V', Some(b'I')) => double(4),
        (b'X', Some(b'I')) => double(9),
        (b'L', Some(b'X')) => double(40),
        (b'C', Some(b'X')) => double(90),
        (b'D', Some(b'C')) => double(400),
        (b'M', Some(b'C')) => double(900),
        (b'I', _) => single(1),
        (b'V', _) => single(5),
        (b'X', _) => single(10),
        (b'L', _) => single(50),
        (b'C', _) => single(100),
        (b'D', _) => single(500),
        (b'M', _) => single(1000),

        _ => unreachable!(),
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut s = s.as_bytes();

        let mut sum = 0;

        while let (new_s, Some(next_roman_number)) = parse_next_roman_number(s) {
            sum += next_roman_number;
            s = new_s;
        }

        sum
    }
}

fn main() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
    assert_eq!(Solution::roman_to_int("IV".to_string()), 4);
    assert_eq!(Solution::roman_to_int("IX".to_string()), 9);
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
    assert_eq!(Solution::roman_to_int("MMXXI".to_string()), 2021);
    assert_eq!(Solution::roman_to_int("MMMCMXCIX".to_string()), 3999);

    println!("Pass test cases!");
}
