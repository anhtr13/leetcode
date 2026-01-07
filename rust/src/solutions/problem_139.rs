pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
    let n = s.len();
    let mut dp = vec![false; n + 1];
    dp[0] = true;
    for i in 1..(n + 1) {
        for w in word_dict.iter() {
            if w.len() <= i {
                let j = i - w.len();
                if &s[j..i] == w {
                    dp[i] = dp[j];
                }
            }
            if dp[i] {
                break;
            }
        }
    }
    dp[n]
}
