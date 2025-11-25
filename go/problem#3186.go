package main

import "slices"

func maximumTotalDamage(power []int) int64 {
	count := map[int64]int64{}
	d_power := []int64{}
	for _, p := range power {
		d_p := int64(p)
		if count[d_p] == 0 {
			d_power = append(d_power, d_p)
		}
		count[int64(d_p)]++
	}
	slices.Sort(d_power)
	n := len(d_power)
	dp := make([]int64, n)
	dp[0] = d_power[0] * count[d_power[0]]
	res := dp[0]
	for i := 1; i < n; i++ {
		j := i - 1
		if d_power[i]-d_power[j] == 1 || d_power[i]-d_power[j] == 2 {
			j--
		}
		if j >= 0 && d_power[i]-d_power[j] == 2 {
			j--
		}
		x := int64(0)
		if j >= 0 {
			x = dp[j]
		}
		res = max(res, x+d_power[i]*count[d_power[i]])
		dp[i] = res
	}
	return res
}
