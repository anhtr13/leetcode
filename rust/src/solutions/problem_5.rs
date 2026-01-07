pub fn longest_palindrome(s: String) -> String {
    let str = s.as_bytes();
    let n = str.len();
    let (mut start, mut end) = (0, 0);
    for i in 1..n {
        let (mut l, mut r) = (i, i);
        loop {
            if str[l] != str[r] {
                break;
            }
            if (r - l) > (end - start) {
                start = l;
                end = r;
            }
            if l >= 1 && r + 1 < n {
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }
    }
    for i in 1..n {
        let (mut l, mut r) = (i - 1, i);
        loop {
            if str[l] != str[r] {
                break;
            }
            if (r - l) > (end - start) {
                start = l;
                end = r;
            }
            if l >= 1 && r + 1 < n {
                l -= 1;
                r += 1;
            } else {
                break;
            }
        }
    }
    std::str::from_utf8(&str[start..(end + 1)])
        .unwrap_or_else(|_| "")
        .to_string()
}
