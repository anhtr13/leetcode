pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let n = nums.len();
    let mut bucket = vec![Vec::<i32>::new(); n + 1];
    let mut freq = std::collections::HashMap::<i32, i32>::new();
    for n in nums {
        let f = *freq.entry(n).or_default() + 1;
        freq.insert(n, f);
    }
    for (n, f) in freq {
        bucket[f as usize].push(n);
    }
    let mut ans = Vec::<i32>::new();
    let mut i = n;
    while ans.len() < k as usize {
        ans.extend(&bucket[i]);
        i -= 1;
    }
    while ans.len() > k as usize {
        ans.pop();
    }
    ans
}
