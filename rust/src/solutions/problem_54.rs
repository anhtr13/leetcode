use crate::Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut u, mut d, mut l, mut r) = (0, m - 1, 0, n - 1);
        let mut dir = 1;
        let mut res = Vec::<i32>::with_capacity(m * n);
        while res.len() < m * n {
            if dir == 1 {
                for i in l..(r + 1) {
                    res.push(matrix[u][i]);
                }
                u += 1;
                dir = 2;
            } else if dir == 2 {
                for i in u..(d + 1) {
                    res.push(matrix[i][r]);
                }
                r -= 1;
                dir = 3;
            } else if dir == 3 {
                for i in (l..(r + 1)).rev() {
                    res.push(matrix[d][i]);
                }
                d -= 1;
                dir = 4;
            } else if dir == 4 {
                for i in (u..(d + 1)).rev() {
                    res.push(matrix[i][l]);
                }
                l += 1;
                dir = 1;
            }
        }
        return res;
    }
}
