fn backtrack_46(
    nums: &Vec<i32>,
    set: &mut std::collections::HashSet<i32>,
    v: &mut Vec<i32>,
    res: &mut Vec<Vec<i32>>,
) {
    if set.len() == nums.len() {
        res.push(v.clone());
        return;
    }
    for x in nums {
        if !set.contains(x) {
            set.insert(*x);
            v.push(*x);
            backtrack_46(nums, set, v, res);
            v.pop();
            set.remove(x);
        }
    }
}

pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = Vec::<Vec<i32>>::new();
    backtrack_46(
        &nums,
        &mut std::collections::HashSet::<i32>::new(),
        &mut Vec::<i32>::new(),
        &mut res,
    );
    res
}
