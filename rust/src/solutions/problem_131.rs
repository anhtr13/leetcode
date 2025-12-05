use crate::Solution;

impl Solution {
    fn check_palindrome_131(s: &[u8]) -> bool {
        let (mut l, mut r) = (0, s.len() - 1);
        while l < r {
            if s[l] != s[r] {
                return false;
            }
            l += 1;
            r -= 1;
        }
        true
    }
    fn backtrack_131(s: &str, res: &mut Vec<Vec<String>>, cur: &mut Vec<String>, i: usize) {
        if i >= s.len() {
            res.push(cur.clone());
            return;
        }
        for j in (i + 1)..(s.len() + 1) {
            let ss = &s[i..j];
            if Self::check_palindrome_131(ss.as_bytes()) {
                cur.push(ss.to_string());
                Self::backtrack_131(s, res, cur, j);
                cur.pop();
            }
        }
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut res = Vec::<Vec<String>>::new();
        let s = s.as_str();
        Self::backtrack_131(s, &mut res, &mut Vec::<String>::new(), 0);
        res
    }
}
