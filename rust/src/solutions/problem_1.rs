pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut pos = std::collections::HashMap::<i32, i32>::with_capacity(nums.len());
    for (i, n) in nums.iter().enumerate() {
        let x = target - n;
        if pos.contains_key(&x) {
            let j = pos.get(&x).unwrap();
            return vec![*j, i as i32];
        }
        pos.insert(*n, i as i32);
    }
    vec![]
}
