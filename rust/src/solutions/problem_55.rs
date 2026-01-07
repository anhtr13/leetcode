pub fn can_jump(nums: Vec<i32>) -> bool {
    let n = nums.len();
    if n <= 1 {
        return true;
    }
    let mut max_reach = 0;
    for i in 0..n {
        if max_reach < i {
            return false;
        }
        max_reach = max_reach.max(i + nums[i] as usize);
        if max_reach >= n - 1 {
            return true;
        }
    }
    return max_reach >= n - 1;
}
