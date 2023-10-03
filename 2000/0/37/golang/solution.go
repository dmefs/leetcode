package solution

import "sort"

func minMovesToSeat(seats []int, students []int) int {
	var moves int
	sort.Ints(seats)
	sort.Ints(students)
	for i, val := range seats {
		moves += diff(val, students[i])
	}
	return moves
}

func diff(a, b int) int {
	if a > b {
		return a - b
	}
	return b - a
}
