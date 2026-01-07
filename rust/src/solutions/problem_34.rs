pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.is_empty() {
        return vec![-1, -1];
    }
    let n = nums.len();

    let (mut l, mut r) = (0, n - 1);
    while l < r {
        let m = (l + r) / 2;
        if nums[m] < target {
            l = m + 1;
        } else {
            r = m;
        }
    }
    if nums[l] != target {
        return vec![-1, -1];
    }

    let f = l as i32;

    let (mut l, mut r) = (0, n - 1);
    while l < r {
        let m = (l + r + 1) / 2;
        if nums[m] > target {
            r = m - 1;
        } else {
            l = m;
        }
    }
    if nums[r] != target {
        return vec![-1, -1];
    }

    let s = r as i32;

    return vec![f, s];
}
