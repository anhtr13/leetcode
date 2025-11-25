package main

func pacificAtlantic(heights [][]int) [][]int {
	m, n := len(heights), len(heights[0])
	atl, pac := make([][]bool, m), make([][]bool, m)
	for i := range m {
		atl[i] = make([]bool, n)
		pac[i] = make([]bool, n)
	}

	p_queue := [][]int{}
	a_queue := [][]int{}

	for i := range m {
		pac[i][0] = true
		p_queue = append(p_queue, []int{i, 0})
		atl[i][n-1] = true
		a_queue = append(a_queue, []int{i, n - 1})
	}
	for j := range n {
		pac[0][j] = true
		p_queue = append(p_queue, []int{0, j})
		atl[m-1][j] = true
		a_queue = append(a_queue, []int{m - 1, j})
	}

	dir := [][]int{{1, 0}, {-1, 0}, {0, 1}, {0, -1}}
	for len(p_queue) > 0 {
		a, b := p_queue[0][0], p_queue[0][1]
		p_queue = p_queue[1:]
		for _, d := range dir {
			x, y := a+d[0], b+d[1]
			if x >= 0 && x < m && y >= 0 && y < n &&
				!pac[x][y] &&
				heights[x][y] >= heights[a][b] {
				pac[x][y] = true
				p_queue = append(p_queue, []int{x, y})
			}
		}
	}
	for len(a_queue) > 0 {
		a, b := a_queue[0][0], a_queue[0][1]
		a_queue = a_queue[1:]
		for _, d := range dir {
			x, y := a+d[0], b+d[1]
			if x >= 0 && x < m && y >= 0 && y < n &&
				!atl[x][y] &&
				heights[x][y] >= heights[a][b] {
				atl[x][y] = true
				a_queue = append(a_queue, []int{x, y})
			}
		}
	}

	res := [][]int{}
	for i := range m {
		for j := range n {
			if atl[i][j] && pac[i][j] {
				res = append(res, []int{i, j})
			}
		}
	}
	return res
}
