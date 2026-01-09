pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (m, n) = (matrix.len(), matrix[0].len());
    let (mut u, mut d) = (0, m - 1);
    while u < d {
        let mid = (u + d + 1) / 2;
        if matrix[mid][0] > target {
            d = mid - 1;
        } else {
            u = mid;
        }
    }
    if matrix[d][0] > target {
        return false;
    }
    for i in 0..=d {
        let (mut l, mut r) = (0, n - 1);
        while l < r {
            let mid = (l + r) / 2;
            if matrix[i][mid] < target {
                l = mid + 1;
            } else {
                r = mid;
            }
        }
        if matrix[i][r] == target {
            return true;
        }
    }
    false
}
