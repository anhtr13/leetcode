package main

func minimumSum(grid [][]int) int {
	// minimumArea func(start_row, end_row, start_column, end_column)
	minimumArea := func(sr, er, sc, ec int) int {
		u, d := er, sr
		l, r := ec, sc
		for i := sr; i <= er; i++ {
			for j := sc; j <= ec; j++ {
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

	m, n := len(grid), len(grid[0])
	res := m * n

	/*
	   -------------
	   |    (1)    |
	   |           |
	   -------------
	   | (2) | (3) |
	   |     |     |
	   -------------
	*/
	for i := 0; i < m-1; i++ {
		one := minimumArea(0, i, 0, n-1)
		for j := 0; j < n-1; j++ {
			two := minimumArea(i+1, m-1, 0, j)
			three := minimumArea(i+1, m-1, j+1, n-1)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	/*
	   -------------
	   | (2) | (3) |
	   |     |     |
	   ------------
	   |           |
	   |    (1)    |
	   -------------
	*/
	for i := 0; i < m-1; i++ {
		one := minimumArea(i+1, m-1, 0, n-1)
		for j := 0; j < n-1; j++ {
			two := minimumArea(0, i, 0, j)
			three := minimumArea(0, i, j+1, n-1)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	/*
	   -------------
	   |     | (2) |
	   |     |     |
	   | (1) -------
	   |     |     |
	   |     | (3) |
	   -------------
	*/
	for j := 0; j < n-1; j++ {
		one := minimumArea(0, m-1, 0, j)
		for i := 0; i < m-1; i++ {
			two := minimumArea(0, i, j+1, n-1)
			three := minimumArea(i+1, m-1, j+1, n-1)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	/*
	   -------------
	   |     |     |
	   | (2) |     |
	   ------- (1) |
	   |     |     |
	   | (3) |     |
	   -------------
	*/
	for j := 0; j < n-1; j++ {
		one := minimumArea(0, m-1, j+1, n-1)
		for i := 0; i < m-1; i++ {
			two := minimumArea(0, i, 0, j)
			three := minimumArea(i+1, m-1, 0, j)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	/*
	   -------------
	   |    (1)    |
	   -------------
	   |    (2)    |
	   -------------
	   |    (3)    |
	   -------------
	*/
	for i := 0; i < m-2; i++ {
		one := minimumArea(0, i, 0, n-1)
		for j := i + 1; j < m-1; j++ {
			two := minimumArea(i+1, j, 0, n-1)
			three := minimumArea(j+1, m-1, 0, n-1)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	/*
	   -------------
	   |   |   |   |
	   |   |   |   |
	   |(1)|(2)|(3)|
	   |   |   |   |
	   |   |   |   |
	   -------------
	*/
	for i := 0; i < n-2; i++ {
		one := minimumArea(0, m-1, 0, i)
		for j := i + 1; j < n-1; j++ {
			two := minimumArea(0, m-1, i+1, j)
			three := minimumArea(0, m-1, j+1, n-1)
			if one > 0 && two > 0 && three > 0 {
				res = min(res, one+two+three)
			}
		}
	}

	return res
}
