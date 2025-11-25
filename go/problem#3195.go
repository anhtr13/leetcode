package main

func minimumArea(grid [][]int) int {
	m, n := len(grid), len(grid[0])
	u, d := m-1, 0
	l, r := n-1, 0
	for i := range m {
		for j := range n {
			if grid[i][j] == 1 {
				u = min(u, i)
				d = max(d, i)
				l = min(l, j)
				r = max(r, j)
			}
		}
	}
	return (r - l + 1) * (d - u + 1)
}
