use crate::Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut l, mut r) = (0, m - 1);
        while l < r {
            let mid = (l + r) / 2;
            if matrix[mid][n - 1] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        let row = &matrix[l];
        if row[0] > target || row[n - 1] < target {
            return false;
        }
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            let mid = (l + r) / 2;
            if row[mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if row[l] != target {
            return false;
        }
        true
    }
}
