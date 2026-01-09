pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len();
    for i in 0..(n / 2) {
        for j in 0..(n + 1) / 2 {
            let x = matrix[j][n - 1 - i];
            let y = matrix[n - 1 - i][n - 1 - j];
            let z = matrix[n - 1 - j][i];
            matrix[j][n - 1 - i] = matrix[i][j];
            matrix[n - 1 - i][n - 1 - j] = x;
            matrix[n - 1 - j][i] = y;
            matrix[i][j] = z;
        }
    }
}
