package main

import "slices"

func numberOfPairs(points [][]int) int {
	slices.SortFunc(points, func(p1, p2 []int) int {
		if p1[1] == p2[1] {
			if p1[0] < p2[0] {
				return -1
			}
			return 1
		}
		if p1[1] > p2[1] {
			return -1
		}
		return 1
	})
	res := 0
	for i, p1 := range points {
		xStack := []int{}
		for j := i + 1; j < len(points); j++ {
			p2 := points[j]
			if p1[0] > p2[0] {
				continue
			}
			for len(xStack) > 0 && xStack[len(xStack)-1] > p2[0] {
				xStack = xStack[:(len(xStack) - 1)]
			}
			if len(xStack) == 0 {
				res++
				xStack = append(xStack, p2[0])
			}
		}
	}
	return res
}
