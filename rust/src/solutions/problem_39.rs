use crate::Solution;

impl Solution {
    fn backtrack_39(
        candidates: &Vec<i32>,
        target: i32,
        res: &mut Vec<Vec<i32>>,
        v: &mut Vec<i32>,
        cur_sum: i32,
        cur_idx: usize,
    ) {
        for i in cur_idx..candidates.len() {
            let sum = cur_sum + candidates[i];
            v.push(candidates[i]);
            if sum == target {
                res.push(v.clone());
            } else if sum < target {
                Self::backtrack_39(candidates, target, res, v, sum, i);
            }
            v.pop();
        }
    }
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut res = Vec::<Vec<i32>>::new();
        Self::backtrack_39(&candidates, target, &mut res, &mut Vec::<i32>::new(), 0, 0);
        res
    }
}
