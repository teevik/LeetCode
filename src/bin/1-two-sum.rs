use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Map from number to it's index
        let mut map = HashMap::new();

        nums.into_iter()
            .enumerate()
            .find_map(|(index, num)| {
                let diff = target - num;

                if let Some(&other_index) = map.get(&diff) {
                    Some(vec![index as i32, other_index as i32])
                } else {
                    map.insert(num, index);

                    None
                }
            })
            .expect("Unsolvable")
    }
}

fn main() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
}
