package main

func peopleAwareOfSecret(n int, delay int, forget int) int {
	mod := int(1e9 + 7)
	share := 0
	dp := make([]int, n+1)
	dp[1] = 1
	for i := 2; i <= n; i++ {
		share += dp[max(0, i-delay)] - dp[max(0, i-forget)] + mod
		share %= mod
		dp[i] = share
	}
	res := 0
	for i := n - forget + 1; i <= n; i++ {
		res += dp[i]
		res %= mod
	}
	return res
}
