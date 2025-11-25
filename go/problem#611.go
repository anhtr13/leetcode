package main

import "slices"

func triangleNumber(nums []int) int {
	slices.Sort(nums)
	res := 0
	for k := len(nums) - 1; k > 1; k-- {
		i, j := 0, k-1
		for i < j {
			if nums[i]+nums[j] > nums[k] {
				res += j - i
				j--
			} else {
				i++
			}
		}
	}
	return res
}
