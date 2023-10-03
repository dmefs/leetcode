package solution

func numIdenticalPairs(nums []int) int {
	m := make(map[int]int)
	ans := 0
	for _, num := range nums {
		if count, ok := m[num]; ok {
			ans += count
			m[num] = count + 1
		} else {
			m[num] = 1
		}
	}
	return ans
}
