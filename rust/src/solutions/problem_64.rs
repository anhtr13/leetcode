pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut dp = vec![vec![i32::MAX; n + 1]; m + 1];
    dp[0][1] = 0;
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1]) + grid[i - 1][j - 1];
        }
    }
    dp[m][n]
}
