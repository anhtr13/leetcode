package main

import "slices"

func successfulPairs(spells []int, potions []int, success int64) []int {
	slices.Sort(potions)
	n := len(potions)
	count := func(s int64) int {
		l, r := 0, n-1
		if s*int64(potions[r]) < success {
			return 0
		}
		for l < r {
			m := (l + r) / 2
			if s*int64(potions[m]) >= success {
				r = m
			} else {
				l = m + 1
			}
		}
		return n - r
	}
	res := []int{}
	for _, s := range spells {
		res = append(res, count(int64(s)))
	}
	return res
}
