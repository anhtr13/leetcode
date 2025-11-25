package main

func findKthPositive(arr []int, k int) int {
	if arr[0] > k {
		return k
	}
	l, r := 0, len(arr)-1
	for l < r {
		m := (l + r + 1) / 2
		missingBeforeM := arr[m] - (m + 1)
		if missingBeforeM >= k {
			r = m - 1
		} else {
			l = m
		}
	}
	missingBefore := arr[l] - (l + 1)
	return arr[l] + (k - missingBefore)
}
