use crate::Solution;

impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let w1 = word1.as_bytes();
        let w2 = word2.as_bytes();
        let (m, n) = (w1.len(), w2.len());
        let mut cost = vec![vec![0; n + 1]; m + 1];
        for i in 1..=m {
            cost[i][0] = i as i32;
        }
        for j in 1..=n {
            cost[0][j] = j as i32;
        }
        for i in 1..=m {
            for j in 1..=n {
                let mut c = 1;
                if w1[i - 1] == w2[j - 1] {
                    c = 0;
                }
                cost[i][j] = (cost[i - 1][j - 1] + c)
                    .min(cost[i - 1][j] + 1)
                    .min(cost[i][j - 1] + 1);
            }
        }
        cost[m][n]
    }
}
