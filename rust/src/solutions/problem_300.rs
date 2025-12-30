use crate::Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut sub_seq = Vec::with_capacity(n);
        sub_seq.push(nums[0]);
        for i in 1..n {
            let (mut l, mut r) = (0, sub_seq.len() - 1);
            while l < r {
                let m = (l + r) / 2;
                if sub_seq[m] < nums[i] {
                    l = m + 1;
                } else {
                    r = m;
                }
            }
            if sub_seq[r] < nums[i] {
                sub_seq.push(nums[i]);
            } else {
                sub_seq[r] = nums[i];
            }
        }
        sub_seq.len() as i32
    }
}
