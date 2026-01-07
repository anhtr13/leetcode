pub fn jump(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    if n <= 1 {
        return 0;
    }
    let mut max_reach = 0;
    let mut reach = 0;
    let mut ans = 0;
    for i in 0..n {
        max_reach = max_reach.max(i + nums[i] as usize).min(n - 1);
        if max_reach == n - 1 {
            return ans + 1;
        }
        if reach == i {
            ans += 1;
            reach = max_reach;
        }
    }
    ans
}
