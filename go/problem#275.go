package main

func hIndex(citations []int) int {
	n := len(citations)
	l, r := 0, n-1
	for l < r {
		m := (l + r) / 2
		papers := n - m
		cities := citations[m]
		if cities < papers {
			l = m + 1
		} else {
			r = m
		}
	}
	papers := n - l
	cities := citations[l]
	if cities < papers {
		return 0
	}
	return papers
}
