pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);
    while l < r {
        let m = (l + r) / 2;
        if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    if nums[l] < target {
        return l as i32 + 1;
    }
    l as i32
}
