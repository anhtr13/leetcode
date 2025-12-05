use crate::Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let (n1, n2) = if nums1.len() <= nums2.len() {
            (&nums1, &nums2)
        } else {
            (&nums2, &nums1)
        };

        let (len1, len2) = (n1.len(), n2.len());
        let (mut l, mut r) = (0, len1);

        while l <= r {
            let p1 = (l + r) / 2;
            let p2 = (len1 + len2 + 1) / 2 - p1;

            let max_left1 = if p1 == 0 { std::i32::MIN } else { n1[p1 - 1] };
            let min_right1 = if p1 == len1 { std::i32::MAX } else { n1[p1] };
            let max_left2 = if p2 == 0 { std::i32::MIN } else { n2[p2 - 1] };
            let min_right2 = if p2 == len2 { std::i32::MAX } else { n2[p2] };

            if max_left1 <= min_right2 && max_left2 <= min_right1 {
                if (len1 + len2) % 2 == 0 {
                    return (max_left1.max(max_left2) as f64 + min_right1.min(min_right2) as f64)
                        / 2.0;
                } else {
                    return max_left1.max(max_left2) as f64;
                }
            } else if max_left1 > min_right2 {
                r = p1 - 1;
            } else {
                l = p1 + 1;
            }
        }

        0.0
    }
}
