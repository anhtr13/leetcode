package main

func findDiagonalOrder(mat [][]int) []int {
	m, n := len(mat), len(mat[0])
	res := []int{}
	i, j := 0, 0
	x, y := -1, 1
	for i < m && j < n {
		res = append(res, mat[i][j])
		if i+x < 0 || j+y >= n {
			if j+y < n {
				j += y
			} else {
				i += 1
			}
			x = -x
			y = -y
		} else if i+x >= m || j+y < 0 {
			if i+x < m {
				i += x
			} else {
				j += 1
			}
			x = -x
			y = -y
		} else {
			i += x
			j += y
		}
	}
	return res
}
