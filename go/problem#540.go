package main

// The single element that need to be found will change the parity of the other pairs.
// [even-odd]...s...[odd-even]
func singleNonDuplicate(nums []int) int {
	n := len(nums)
	l, r := 0, n-1
	for l < r {
		m := (l + r) / 2
		i1, i2 := m, m
		if m-1 >= 0 && nums[m-1] == nums[m] {
			i1 = m - 1
		} else if m+1 < n && nums[m+1] == nums[m] {
			i2 = m + 1
		}
		if i1 == i2 {
			return nums[m]
		}
		if i1%2 == 0 {
			l = i2 + 1
		} else {
			r = i1 - 1
		}
	}
	return nums[l]
}
