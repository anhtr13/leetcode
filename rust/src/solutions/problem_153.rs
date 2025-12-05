use crate::Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let m = (l + r) / 2;
            if nums[l] < nums[m] {
                if nums[l] > nums[r] {
                    l = m + 1
                } else {
                    return nums[l];
                }
            } else {
                if nums[m + 1] > nums[m] {
                    r = m
                } else {
                    return nums[m + 1];
                }
            }
        }
        nums[l]
    }
}
