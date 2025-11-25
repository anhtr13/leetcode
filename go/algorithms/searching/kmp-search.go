package searching

func FindLPS(s string) []int {
	n := len(s)
	pi := make([]int, n)
	sub_len := 0
	i := 1
	for i < n {
		if s[i] == s[sub_len] {
			sub_len++
			pi[i] = sub_len
			i++
		} else {
			if sub_len == 0 {
				pi[i] = 0
				i++
			} else {
				sub_len = pi[sub_len-1]
			}
		}
	}
	return pi
}

func KmpSearch(str, subStr string) int {
	if len(subStr) > len(str) {
		return -1
	}
	s := subStr + " " + str
	pi := FindLPS(s)
	for i := len(subStr) + 1; i < len(s); i++ {
		if pi[i] == len(subStr) {
			return i - 2*len(subStr)
		}
	}
	return -1
}
