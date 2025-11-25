package main

func lenOfVDiagonal(grid [][]int) int {
	/*
		Direction:
		| 2 |   | 3 |
		|		| x |		|
		|	1 | 	|	0 |
	*/
	ds := [4][2]int{{1, 1}, {1, -1}, {-1, -1}, {-1, 1}}
	nx := [3]int{2, 2, 0}

	m, n := len(grid), len(grid[0])
	cache := [][][3][4][2]int{}
	for range m {
		row1 := [][3][4][2]int{}
		for range n {
			row1 = append(row1, [3][4][2]int{})
		}
		cache = append(cache, row1)
	}

	var dfs func(i, j, x, d, k int) int
	dfs = func(i, j, x, d, k int) int {
		if i >= m || i < 0 || j >= n || j < 0 {
			return 0
		}
		if grid[i][j] != x {
			return 0
		}
		if cache[i][j][x][d][k] > 0 {
			return cache[i][j][x][d][k]
		}
		res := dfs(i+ds[d][0], j+ds[d][1], nx[x], d, k) + 1
		if k > 0 {
			d2 := (d + 1) % 4
			res2 := dfs(i+ds[d2][0], j+ds[d2][1], nx[x], d2, 0) + 1
			res = max(res, res2)
		}
		cache[i][j][x][d][k] = res
		return res
	}

	res := 0
	for i := range m {
		for j := range n {
			if grid[i][j] == 1 {
				cur := 0
				for d := range 4 {
					cur = max(cur, dfs(i, j, 1, d, 1))
				}
				res = max(res, cur)
			}
		}
	}
	return res
}
