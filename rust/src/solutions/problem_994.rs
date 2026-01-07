pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (grid.len(), grid[0].len());
    let mut rotten = vec![vec![false; n]; m];
    let mut q = Vec::<[i8; 2]>::new();
    for (x, row) in grid.iter().enumerate() {
        for (y, cell) in row.iter().enumerate() {
            if *cell == 2 {
                rotten[x][y] = true;
                q.push([x as i8, y as i8]);
            }
        }
    }
    let mut res = 0;
    let adj = [[1, 0], [-1, 0], [0, 1], [0, -1]];
    while !q.is_empty() {
        let mut new_q = Vec::<[i8; 2]>::new();
        for p in q.iter() {
            let (x, y) = (p[0], p[1]);

            for a in adj.iter() {
                let (nx, ny) = (x + a[0], y + a[1]);
                if nx >= 0
                    && nx < m as i8
                    && ny >= 0
                    && ny < n as i8
                    && grid[nx as usize][ny as usize] == 1
                    && !rotten[nx as usize][ny as usize]
                {
                    rotten[nx as usize][ny as usize] = true;
                    new_q.push([nx, ny]);
                }
            }
        }
        if !new_q.is_empty() {
            res += 1;
            q = new_q;
        } else {
            break;
        }
    }
    for x in 0..m {
        for y in 0..n {
            if grid[x][y] == 1 && !rotten[x][y] {
                return -1;
            }
        }
    }
    res
}
