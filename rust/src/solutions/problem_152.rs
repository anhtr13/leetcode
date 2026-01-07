pub fn max_product(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }
    let (mut pos, mut neg) = (0, 0);
    let mut res = nums[0];
    for n in nums.iter() {
        if *n > 0 {
            neg *= n;
            if pos == 0 {
                pos = *n;
            } else {
                pos *= n;
            }
            res = res.max(pos);
        } else if *n < 0 {
            let (old_pos, old_neg) = (pos, neg);
            if old_pos > 0 {
                neg = old_pos * n;
            } else {
                neg = *n;
            }
            if neg == 0 {
                pos = 0;
            } else {
                pos = old_neg * n;
            }
            res = res.max(pos);
        } else {
            pos = 0;
            neg = 0;
        }
    }
    if res < 0 {
        for x in nums.iter() {
            res = res.max(*x);
        }
    }
    res
}
