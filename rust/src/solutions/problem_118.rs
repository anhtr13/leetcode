use crate::Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = Vec::<Vec<i32>>::with_capacity(num_rows as usize);
        for n in 0..(num_rows as usize) {
            let mut row = vec![1; n as usize + 1];
            for i in 1..n {
                row[i] = res[n-1][i - 1] + res[n-1][i];
            }
            res.push(row);
        }
        res
    }
}
