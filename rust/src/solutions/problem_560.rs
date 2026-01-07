pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum = 0;
    let mut ans = 0;
    let mut count = std::collections::HashMap::<i32, i32>::new();
    count.insert(0, 1);
    for n in nums {
        sum += n;
        let s = sum - k;
        if let Some(c) = count.get(&s) {
            ans += c;
        }
        if let Some(c) = count.get(&sum) {
            count.insert(sum, c + 1);
        } else {
            count.insert(sum, 1);
        }
    }
    ans
}
