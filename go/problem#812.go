package main

func largestTriangleArea(points [][]int) float64 {
	res := float64(0)
	for i := range points {
		x1, y1 := float64(points[i][0]), float64(points[i][1])
		for j := i + 1; j < len(points); j++ {
			x2, y2 := float64(points[j][0]), float64(points[j][1])
			for k := j + 1; k < len(points); k++ {
				x3, y3 := float64(points[k][0]), float64(points[k][1])
				s := (x1*(y2-y3) + x2*(y3-y1) + x3*(y1-y2)) / float64(2)
				if s < 0 {
					s = -s
				}
				if res < s {
					res = s
				}
			}
		}
	}
	return res
}
