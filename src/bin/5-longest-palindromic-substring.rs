struct Solution;

fn is_palindrome(s: &str) -> bool {
    let s = s.as_bytes();

    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() - 1 - i] {
            return false;
        }
    }

    true
}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        struct LongestPalindrome {
            length: usize,
            string: String,
        }

        let mut longest = LongestPalindrome {
            length: 1,
            string: (&s[0..1]).to_string(),
        };

        for start in 0..s.len() {
            for end in start..s.len() {
                let length = end - start + 1;
                if is_palindrome(&s[start..=end]) {
                    if length > longest.length {
                        longest = LongestPalindrome {
                            length,
                            string: (&s[start..=end]).to_string(),
                        };
                    }
                }
            }
        }

        longest.string
    }
}

fn main() {
    {
        let s = String::from("babad");
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bab");
    }
    {
        let s = String::from("cbbd");
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "bb");
    }
    {
        let s = String::from("a");
        let result = Solution::longest_palindrome(s);
        assert_eq!(result, "a");
    }
}
