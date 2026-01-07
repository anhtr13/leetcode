fn bfs_200(grid: &mut Vec<Vec<char>>, nei: &[[i16; 2]; 4], x: i16, y: i16) {
    grid[x as usize][y as usize] = '0';
    for n in nei.iter() {
        let nx = n[0] + x;
        let ny = n[1] + y;
        if nx >= 0 && ny >= 0 && nx < grid.len() as i16 && ny < grid[0].len() as i16 {
            if grid[nx as usize][ny as usize] == '1' {
                bfs_200(grid, nei, nx, ny);
            }
        }
    }
}

pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid.clone();
    let mut res = 0;
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] == '1' {
                res += 1;
                bfs_200(
                    &mut grid,
                    &[[-1, 0], [1, 0], [0, -1], [0, 1]],
                    x as i16,
                    y as i16,
                );
            }
        }
    }
    res
}
