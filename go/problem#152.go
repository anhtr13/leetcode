package main

func maxProduct(nums []int) int {
	n := len(nums)
	if n == 1 {
		return nums[0]
	}
	neg := 0
	pos := 0
	res := nums[0]
	for i := range n {
		if nums[i] > 0 {
			neg *= nums[i]
			if pos == 0 {
				pos = nums[i]
			} else {
				pos *= nums[i]
			}
			res = max(res, pos)
		} else if nums[i] < 0 {
			old_neg := neg
			old_pos := pos
			if old_pos > 0 {
				neg = old_pos * nums[i]
			} else {
				neg = nums[i]
			}
			if neg == 0 {
				pos = 0
			} else {
				pos = old_neg * nums[i]
			}
			res = max(res, pos)
		} else {
			neg = 0
			pos = 0
		}
	}
	if res < 0 {
		for _, x := range nums {
			res = max(res, x)
		}
	}
	return res
}
