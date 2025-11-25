package main

func isMatch(s string, p string) bool {
	m, n := len(s), len(p)
	dp := [][]bool{}
	for range m + 1 {
		dp = append(dp, make([]bool, n+1))
	}
	dp[0][0] = true
	for j := 2; j <= n; j++ {
		dp[0][j] = p[j-1] == '*' && dp[0][j-2]
	}
	for j := 1; j <= n; j++ {
		for i := 1; i <= m; i++ {
			if s[i-1] == p[j-1] || p[j-1] == '.' {
				dp[i][j] = dp[i-1][j-1]
			} else if p[j-1] == '*' {
				dp[i][j] = dp[i][j-2] || (dp[i-1][j] && ((s[i-1] == p[j-2]) || p[j-2] == '.'))
			}
		}
	}
	return dp[m][n]
}
