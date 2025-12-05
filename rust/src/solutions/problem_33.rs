use crate::Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            let m = (l + r) / 2;
            if nums[m] > nums[r] {
                l = m + 1;
            } else {
                r = m;
            }
        }
        let rot_pos = l;
        let (mut l, mut r) = (0, n - 1);
        while l <= r {
            let m = (l + r) / 2;
            let m_real = (m + rot_pos) % n;
            if nums[m_real] == target {
                return m_real as i32;
            }
            if nums[m_real] < target {
                l = m + 1;
            } else {
                if m == 0 {
                    break;
                }
                r = m - 1;
            }
        }
        -1
    }
}
