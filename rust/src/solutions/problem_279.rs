use crate::Solution;

impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let n = n as usize;
        let mut squares = Vec::<usize>::new();
        let mut i = 1;
        while i * i <= n {
            squares.push(i * i);
            i += 1;
        }
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;
        for i in 1..=n {
            for s in squares.iter() {
                if *s > i {
                    break;
                }
                let j = i - *s;
                dp[i] = dp[i].min(dp[j] + 1);
            }
        }
        dp[n]
    }
}
