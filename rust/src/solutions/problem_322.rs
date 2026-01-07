pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    let amount = amount as usize;
    let mut coins: Vec<usize> = coins.iter().map(|x| *x as usize).collect();
    coins.sort_unstable();
    let mut dp = vec![10001; amount + 1];
    dp[0] = 0;
    for a in 1..=amount {
        for c in coins.iter() {
            if *c > a {
                break;
            }
            let i = a - c;
            dp[a] = dp[a].min(dp[i] + 1);
        }
    }
    if dp[amount] == 10001 {
        return -1;
    }
    dp[amount]
}
