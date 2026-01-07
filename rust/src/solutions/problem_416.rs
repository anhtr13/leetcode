pub fn can_partition(nums: Vec<i32>) -> bool {
    let sum: i32 = nums.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let part = sum as usize / 2;
    let mut ok = vec![false; part + 1];
    ok[0] = true;
    for x in nums {
        let x = x as usize;
        if x <= part {
            for y in (x..=part).rev() {
                if ok[y - x] {
                    ok[y] = ok[y - x];
                }
            }
        }
    }
    ok[part]
}
