struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut i = 0;

        loop {
            let mut char_at_index = strs.iter().map(|str| str.as_bytes().get(i).copied());

            let first_char = char_at_index.next().expect("strs is never empty");

            let is_done = first_char.is_none();
            let is_done = is_done || char_at_index.any(|char| char != first_char);

            if is_done {
                break strs[0][0..i].to_owned();
            }

            i += 1;
        }
    }
}

fn main() {
    assert_eq!(
        Solution::longest_common_prefix(
            ["flower", "flow", "flight"]
                .map(|str| str.to_string())
                .to_vec()
        ),
        "fl"
    );
    assert_eq!(
        Solution::longest_common_prefix(
            ["dog", "racecar", "car"]
                .map(|str| str.to_string())
                .to_vec()
        ),
        ""
    );
}
