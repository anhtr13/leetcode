package main

func searchMatrix(matrix [][]int, target int) bool {
	x := len(matrix)
	y := len(matrix[0])
	if target < matrix[0][0] || target > matrix[x-1][y-1] {
		return false
	}
	var row []int
	l, r := 0, x-1
	for l <= r {
		if l == r {
			row = matrix[l]
			break
		}
		m := (l + r) / 2
		if matrix[m][0] <= target && target <= matrix[m][y-1] {
			row = matrix[m]
			break
		}
		if matrix[m][y-1] < target {
			l = m + 1
		} else if matrix[m][0] > target {
			r = m - 1
		} else {
			return false
		}
	}
	if row == nil {
		return false
	}
	l, r = 0, y-1
	for l < r {
		m := (l + r) / 2
		if row[m] == target {
			return true
		}
		if row[m] > target {
			r = m - 1
		} else {
			l = m + 1
		}
	}
	return row[l] == target
}
