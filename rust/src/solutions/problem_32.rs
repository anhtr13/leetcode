use crate::Solution;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let mut stack = Vec::<usize>::new();
        let mut ans = 0;
        let str = s.as_bytes();
        for (i, c) in str.iter().enumerate() {
            if stack.len() == 0 {
                ans = ans.max(i);
                stack.push(i);
            } else {
                let j = stack[stack.len() - 1];
                if *c == b')' {
                    if str[j] == b'(' {
                        stack.pop();
                    } else {
                        stack.push(i);
                    }
                } else {
                    stack.push(i);
                }
                ans = ans.max(i - j - 1);
            }
        }
        if stack.len() == 0 {
            ans = str.len();
        } else {
            let j = stack[stack.len() - 1];
            ans = ans.max(str.len() - j - 1);
        }
        ans as i32
    }
}
