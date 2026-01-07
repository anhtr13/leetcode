fn backtrack_51(
    n: usize,
    col: &mut Vec<bool>,
    dia1: &mut std::collections::HashSet<i16>,
    dia2: &mut std::collections::HashSet<i16>,
    i: i16,
    count: usize,
    board: &mut Vec<Vec<char>>,
    res: &mut Vec<Vec<String>>,
) {
    if count == n {
        let mut ans = Vec::<String>::with_capacity(n);
        for v in board {
            let s = String::from_iter(v.clone());
            ans.push(s);
        }
        res.push(ans);
        return;
    }
    for j in 0..n {
        let d1 = i - j as i16;
        let d2 = i + j as i16;
        if !col[j] && !dia1.contains(&d1) && !dia2.contains(&d2) {
            col[j] = true;
            dia1.insert(d1);
            dia2.insert(d2);
            board[i as usize][j] = 'Q';
            backtrack_51(n, col, dia1, dia2, i + 1, count + 1, board, res);
            col[j] = false;
            dia1.remove(&d1);
            dia2.remove(&d2);
            board[i as usize][j] = '.';
        }
    }
}

pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut col = vec![false; n];
    let mut dia1 = std::collections::HashSet::<i16>::new(); // row - col
    let mut dia2 = std::collections::HashSet::<i16>::new(); // row + col
    let mut board = Vec::<Vec<char>>::with_capacity(n);
    let mut res = Vec::<Vec<String>>::new();
    for _ in 0..n {
        let v = vec!['.'; n];
        board.push(v);
    }
    backtrack_51(
        n, &mut col, &mut dia1, &mut dia2, 0, 0, &mut board, &mut res,
    );
    res
}
