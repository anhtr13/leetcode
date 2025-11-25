package main

import "strconv"

func fractionToDecimal(numerator int, denominator int) string {
	if numerator == 0 {
		return "0"
	}

	res := []byte{}
	if (numerator > 0 && denominator < 0) || (numerator < 0 && denominator > 0) {
		res = append(res, '-')
	}

	if numerator < 0 {
		numerator = -numerator
	}
	if denominator < 0 {
		denominator = -denominator
	}
	num := int64(numerator)
	den := int64(denominator)

	integer := num / den
	num %= den
	interger_str := strconv.Itoa(int(integer))
	res = append(res, []byte(interger_str)...)
	if num == 0 {
		return string(res)
	}

	res = append(res, '.')
	start := map[int64]int{}
	start[num] = len(res)
	for num != 0 {
		num *= 10
		x := num / den
		c := rune('0' + x)
		res = append(res, byte(c))
		num %= den

		idx, ok := start[num]
		if ok {
			new_res := []byte{}
			new_res = append(new_res, res[:idx]...)
			new_res = append(new_res, '(')
			new_res = append(new_res, res[idx:]...)
			new_res = append(new_res, ')')
			return string(new_res)
		} else {
			start[num] = len(res)
		}
	}
	return string(res)
}
