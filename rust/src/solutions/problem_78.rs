use crate::Solution;

impl Solution {
    fn backtrack_78(nums: &Vec<i32>, idx: usize, v: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        res.push(v.clone());
        for i in idx..nums.len() {
            v.push(nums[i]);
            Self::backtrack_78(nums, i + 1, v, res);
            v.pop();
        }
    }
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::new();
        Self::backtrack_78(&nums, 0, &mut Vec::<i32>::new(), &mut res);
        res
    }
}
