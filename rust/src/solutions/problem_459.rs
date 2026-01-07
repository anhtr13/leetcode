pub fn repeated_substring_pattern(s: String) -> bool {
    let mut long_s = s.repeat(2);
    long_s.push_str(&format!(" {s}"));

    let arr = long_s[1..long_s.len() - 1].as_bytes();
    let n = arr.len();

    let mut pi = vec![0; n];
    let mut sub_len: usize = 0;
    let mut i = 1;

    while i < n {
        if arr[i] == arr[sub_len] {
            sub_len += 1;
            pi[i] = sub_len;
            i += 1;
        } else {
            if sub_len == 0 {
                pi[i] = 0;
                i += 1;
            } else {
                sub_len = pi[sub_len - 1];
            }
        }
    }

    for i in (s.len())..(arr.len()) {
        if pi[i] == s.len() {
            return true;
        }
    }
    return false;
}
