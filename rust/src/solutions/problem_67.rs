use crate::Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let binding_a = a.chars().rev().collect::<String>();
        let binding_b = b.chars().rev().collect::<String>();
        let mut a = binding_a.as_bytes();
        let mut b = binding_b.as_bytes();
        if binding_a.len() > binding_b.len() {
            (a, b) = (b, a);
        }
        let mut res = String::new();
        let mut i: usize = 0;
        let mut r: u8 = 0;
        while i < a.len() {
            let x = a[i] - '0' as u8;
            let y = b[i] - '0' as u8;
            let c = x + y + r;
            if c == 0 {
                res.push('0');
            } else if c == 1 {
                res.push('1');
                r = 0;
            } else if c == 2 {
                res.push('0');
                r = 1;
            } else {
                res.push('1');
                r = 1;
            }
            i += 1;
        }
        while i < b.len() {
            let x = b[i] - '0' as u8;
            let c = x + r;
            if c == 0 {
                res.push('0');
            } else if c == 1 {
                res.push('1');
                r = 0;
            } else if c == 2 {
                res.push('0');
                r = 1;
            } else {
                res.push('1');
                r = 1;
            }
            i += 1;
        }
        if r == 1 {
            res.push('1');
        }
        res.chars().rev().collect()
    }
}
