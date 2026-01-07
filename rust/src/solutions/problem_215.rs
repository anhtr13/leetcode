pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    let k = k as usize;
    let mut res = 10000;
    let mut heap = std::collections::BinaryHeap::<i32>::with_capacity(n - k);
    for num in nums {
        heap.push(num);
        if heap.len() > n - k {
            if let Some(x) = heap.pop() {
                res = res.min(x);
            }
        }
    }
    res
}
