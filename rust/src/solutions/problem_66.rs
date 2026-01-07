pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut r = 1;
    let mut res = Vec::with_capacity(digits.len() + 1);
    for x in digits.iter().rev() {
        let y = x + r;
        if y == 10 {
            res.push(0);
            r = 1;
        } else {
            res.push(y);
            r = 0;
        }
    }
    if r == 1 {
        res.push(1);
    }
    res.reverse();
    return res;
}
