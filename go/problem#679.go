package main

import (
	"math"
)

func judgePoint24(cards []int) bool {
	var backtrack func(nums []float64) bool
	backtrack = func(nums []float64) bool {
		n := len(nums)
		if n == 1 {
			return math.Round(nums[0]*100)/100 == 24
		}
		for i := range n {
			for j := i + 1; j < n; j++ {
				vals := []float64{}
				for k := range n {
					if k != i && k != j {
						vals = append(vals, nums[k])
					}
				}
				newVals := []float64{
					nums[i] + nums[j],
					nums[i] * nums[j],
					nums[i] - nums[j],
					nums[j] - nums[i],
				}
				if nums[j] > 0 {
					newVals = append(newVals, nums[i]/nums[j])
				}
				if nums[i] > 0 {
					newVals = append(newVals, nums[j]/nums[i])
				}
				for _, v := range newVals {
					vals = append(vals, v)
					if backtrack(vals) {
						return true
					}
					vals = vals[:len(vals)-1]
				}
			}
		}
		return false
	}
	fcards := []float64{}
	for i := range 4 {
		fcards = append(fcards, float64(cards[i]))
	}
	return backtrack(fcards)
}
