package main

func isPowerOfThree(n int) bool {
	if n <= 0 {
		return false
	}
	big3X := 1162261467
	return big3X%n == 0
}
