struct Solution;

fn find_next_small<'a, 'b>(nums1: &'a [i32], nums2: &'b [i32]) -> (i32, &'a [i32], &'b [i32]) {
    if nums1.is_empty() {
        return (nums2[0], nums1, &nums2[1..]);
    } else if nums2.is_empty() {
        return (nums1[0], &nums1[1..], nums2);
    }

    if nums1[0] < nums2[0] {
        (nums1[0], &nums1[1..], nums2)
    } else {
        (nums2[0], nums1, &nums2[1..])
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_length = nums1.len() + nums2.len();

        let mut nums1 = nums1.as_slice();
        let mut nums2 = nums2.as_slice();

        for _ in 0..((total_length - 1) / 2) {
            let (_, new_nums1, new_nums2) = find_next_small(nums1, nums2);
            nums1 = new_nums1;
            nums2 = new_nums2;
        }

        if total_length % 2 == 1 {
            let (value, _, _) = find_next_small(nums1, nums2);

            value as f64
        } else {
            let (value1, nums1, nums2) = find_next_small(nums1, nums2);
            let (value2, _, _) = find_next_small(nums1, nums2);

            ((value1 as f64) + (value2 as f64)) / 2.
        }
    }
}

fn main() {
    {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];

        assert_eq!(Solution::find_median_sorted_arrays(nums1, nums2), 2.5);
    }
}
