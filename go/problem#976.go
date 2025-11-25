package main

import "slices"

func largestPerimeter(nums []int) int {
	slices.Sort(nums)
	res := 0
	for i := len(nums) - 1; i-2 >= 0; i-- {
		a, b, c := nums[i], nums[i-1], nums[i-2]
		if b+c > a {
			res = a + b + c
			break
		}
	}
	return res
}
