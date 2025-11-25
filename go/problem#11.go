package main

func maxArea(height []int) int {
	l, r := 0, len(height)-1
	res := 0
	for l < r {
		res = max(res, (r-l)*min(height[l], height[r]))
		if height[l] < height[r] {
			l++
		} else {
			r--
		}
	}
	return res
}
