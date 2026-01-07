pub fn roman_to_int(s: String) -> i32 {
    let n = s.len();
    let c = s.as_bytes();
    let key_vals: std::collections::HashMap<u8, i32> = [
        ('I' as u8, 1),
        ('V' as u8, 5),
        ('X' as u8, 10),
        ('L' as u8, 50),
        ('C' as u8, 100),
        ('D' as u8, 500),
        ('M' as u8, 1000),
    ]
    .into_iter()
    .collect();
    let mut res = 0;
    let mut i: usize = 0;
    while i < n {
        if c[i] == 'I' as u8 && i + 1 < n {
            if c[i + 1] == 'V' as u8 {
                res += 4;
                i += 2;
                continue;
            } else if c[i + 1] == 'X' as u8 {
                res += 9;
                i += 2;
                continue;
            }
        }
        if c[i] == 'X' as u8 && i + 1 < n {
            if c[i + 1] == 'L' as u8 {
                res += 40;
                i += 2;
                continue;
            } else if c[i + 1] == 'C' as u8 {
                res += 90;
                i += 2;
                continue;
            }
        }
        if c[i] == 'C' as u8 && i + 1 < n {
            if c[i + 1] == 'D' as u8 {
                res += 400;
                i += 2;
                continue;
            } else if c[i + 1] == 'M' as u8 {
                res += 900;
                i += 2;
                continue;
            }
        }
        res += match key_vals.get(&c[i]) {
            Some(x) => *x,
            None => 0,
        };
        i += 1;
    }
    return res;
}
