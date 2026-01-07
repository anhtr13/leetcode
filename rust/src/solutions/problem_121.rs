pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut buy = i32::MAX;
    let mut ans = 0;
    for sell in prices {
        if sell > buy {
            ans = ans.max(sell - buy);
        }
        buy = buy.min(sell);
    }
    ans
}
