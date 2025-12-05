use crate::Solution;

impl Solution {
    fn backtrack_17(
        letters: &std::collections::HashMap<u8, Vec<char>>,
        digits: &[u8],
        res: &mut Vec<String>,
        cur: &mut String,
        i: usize,
    ) {
        if i == digits.len() {
            res.push(cur.to_string());
            return;
        }
        for c in &letters[&digits[i]] {
            cur.push(*c);
            Self::backtrack_17(letters, digits, res, cur, i + 1);
            cur.pop();
        }
    }
    pub fn letter_combinations(digits: String) -> Vec<String> {
        let letters: std::collections::HashMap<u8, Vec<char>> = [
            ('2' as u8, vec!['a', 'b', 'c']),
            ('3' as u8, vec!['d', 'e', 'f']),
            ('4' as u8, vec!['g', 'h', 'i']),
            ('5' as u8, vec!['j', 'k', 'l']),
            ('6' as u8, vec!['m', 'n', 'o']),
            ('7' as u8, vec!['p', 'q', 'r', 's']),
            ('8' as u8, vec!['t', 'u', 'v']),
            ('9' as u8, vec!['w', 'x', 'y', 'z']),
        ]
        .into_iter()
        .collect();
        let digits = digits.as_bytes();
        let mut res = Vec::<String>::new();
        Self::backtrack_17(&letters, digits, &mut res, &mut String::new(), 0);
        res
    }
}
