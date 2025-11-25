package main

func numSubmat(mat [][]int) int {
	m, n := len(mat), len(mat[0])
	res := 0
	height := make([]int, n)
	for i := range m {
		for j := range n {
			if mat[i][j] == 0 {
				height[j] = 0
			} else {
				height[j] += 1
			}
		}
		sum := make([]int, n)
		stack := []int{}
		for r := range n {
			for len(stack) > 0 && height[stack[len(stack)-1]] >= height[r] {
				stack = stack[:len(stack)-1]
			}
			if len(stack) > 0 {
				l := stack[len(stack)-1]
				sum[r] = sum[l] + height[r]*(r-l)
			} else {
				sum[r] = height[r] * (r + 1)
			}
			stack = append(stack, r)
			res += sum[r]
		}
	}
	return res
}
