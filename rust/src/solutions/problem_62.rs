use crate::Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];
        dp[0][1] = 1;
        for i in 1..=m {
            for j in 1..=n {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m][n]
    }
}
