pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let (m, n) = (matrix.len(), matrix[0].len());
    let mut row = std::collections::HashSet::<usize>::with_capacity(m);
    let mut col = std::collections::HashSet::<usize>::with_capacity(n);
    for (i, r) in matrix.iter().enumerate() {
        for (j, c) in r.iter().enumerate() {
            if *c == 0 {
                row.insert(i);
                col.insert(j);
            }
        }
    }
    for i in row {
        for j in 0..n {
            matrix[i][j] = 0;
        }
    }
    for j in col {
        for i in 0..m {
            matrix[i][j] = 0;
        }
    }
}
