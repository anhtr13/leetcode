package main

func zeroFilledSubarray(nums []int) int64 {
	res := int64(0)
	l, r := int64(0), int64(0)
	if nums[0] == 0 {
		l = -1
	}
	for i, x := range nums {
		if x == 0 {
			r = int64(i)
		} else {
			n := r - l
			res += (n + 1) * n / 2
			l = int64(i)
			r = int64(i)
		}
	}
	n := r - l
	res += (n + 1) * n / 2
	return res
}
