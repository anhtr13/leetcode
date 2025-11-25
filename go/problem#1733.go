package main

func minimumTeachings(n int, languages [][]int, friendships [][]int) int {
	langMap := [501][501]bool{}
	for i, ls := range languages {
		for _, l := range ls {
			langMap[i+1][l] = true
		}
	}
	res := 0
	cannotCommunicate := map[int]bool{}
	for _, frs := range friendships {
		x, y := frs[0], frs[1]
		is_ok := false
		for _, l := range languages[x-1] {
			if langMap[y][l] {
				is_ok = true
				break
			}
		}
		for _, l := range languages[y-1] {
			if langMap[x][l] {
				is_ok = true
				break
			}
		}
		if !is_ok {
			cannotCommunicate[x] = true
			cannotCommunicate[y] = true
		}

	}
	if len(cannotCommunicate) > 0 {
		count := [501]int{}
		maxCount := 0
		for x := range cannotCommunicate {
			for _, l := range languages[x-1] {
				count[l]++
				if count[l] > maxCount {
					maxCount = count[l]
				}
			}
		}
		res += len(cannotCommunicate) - maxCount
	}
	return res
}
