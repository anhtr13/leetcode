pub fn climb_stairs(n: i32) -> i32 {
    let mut dp = vec![0; n.max(2) as usize];
    dp[0] = 1;
    dp[1] = 2;
    for i in 2..(n as usize) {
        dp[i] = dp[i - 1] + dp[i - 2];
    }
    dp[n as usize - 1]
}
