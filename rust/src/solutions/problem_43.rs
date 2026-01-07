fn num_multi_digit(num: &[u8], digit: u8, mut pow: usize) -> Vec<u8> {
    let d = digit - '0' as u8;
    let mut r: u8 = 0;
    let mut res = Vec::<u8>::with_capacity(num.len() + pow + 1);
    while pow > 0 {
        res.push(0);
        pow -= 1;
    }
    for b in num {
        let n = b - '0' as u8;
        let x = n * d + r;
        res.push(x % 10);
        r = x / 10;
    }
    if r > 0 {
        res.push(r);
    }
    res
}

fn add(base: &mut Vec<u8>, num: &Vec<u8>) {
    let mut r = 0;
    for i in 0..(base.len().min(num.len())) {
        let mut x = r + base[i] + num[i];
        r = x / 10;
        x = x % 10;
        base[i] = x;
    }
    if base.len() < num.len() {
        for i in base.len()..num.len() {
            let mut x = r + num[i];
            r = x / 10;
            x = x % 10;
            base.push(x);
        }
    }
    if r > 0 {
        base.push(r);
    }
}

pub fn multiply(num1: String, num2: String) -> String {
    if num1 == "0" || num2 == "0" {
        return "0".to_string();
    }
    let binding_1 = num1.chars().rev().collect::<String>();
    let binding_2 = num2.chars().rev().collect::<String>();
    let mut a = binding_1.as_bytes();
    let mut b = binding_2.as_bytes();
    if a.len() < b.len() {
        (a, b) = (b, a);
    }
    let mut p = 0;
    let mut ans = Vec::<u8>::with_capacity(num1.len() * num2.len() + 1);
    for i in b {
        let num = num_multi_digit(a, *i, p);
        add(&mut ans, &num);
        p += 1;
    }
    ans.reverse();
    let mut res = String::with_capacity(ans.len());
    for i in ans {
        if !(i == 0 && res.is_empty()) {
            res.push((i + '0' as u8) as char);
        }
    }
    if res.is_empty() {
        res.push('0');
    }
    res
}
