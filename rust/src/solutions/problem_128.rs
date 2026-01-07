pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: std::collections::HashSet<i32> = nums.into_iter().collect();
    let mut ans = 0;
    for x in set.iter() {
        if !set.contains(&(x - 1)) {
            let mut y = x + 1;
            while set.contains(&y) {
                y += 1;
            }
            ans = ans.max(y - x);
        }
    }
    ans
}
