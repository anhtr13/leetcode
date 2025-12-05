use crate::Solution;

impl Solution {
    fn backtrack_22(res: &mut Vec<String>, cur: &mut String, n1: i32, n2: i32) {
        if n1 == 0 && n2 == 0 {
            res.push(cur.to_string());
            return;
        }
        if n2 > n1 {
            cur.push(')');
            Self::backtrack_22(res, cur, n1, n2 - 1);
            cur.pop();
        }
        if n1 > 0 {
            cur.push('(');
            Self::backtrack_22(res, cur, n1 - 1, n2);
            cur.pop();
        }
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res = Vec::<String>::new();
        Self::backtrack_22(&mut res, &mut String::new(), n, n);
        res
    }
}
