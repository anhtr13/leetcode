package main

func peakIndexInMountainArray(arr []int) int {
	n := len(arr)
	l, r := 0, n-1
	for l < r {
		m := (l + r + 1) / 2
		if arr[m] > arr[m-1] {
			l = m
		} else if arr[m] < arr[m-1] {
			r = m - 1
		} else {
			return m
		}
	}
	return r
}

