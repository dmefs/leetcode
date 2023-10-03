package solution

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinMovesToSeat1(t *testing.T) {
	assert.Equal(t, 4, minMovesToSeat([]int{3, 1, 5}, []int{2, 7, 4}))
}

func TestMinMovesToSeat2(t *testing.T) {
	assert.Equal(t, 7, minMovesToSeat([]int{4, 1, 5, 9}, []int{1, 3, 2, 6}))
}

func TestMinMovesToSeat3(t *testing.T) {
	assert.Equal(t, 4, minMovesToSeat([]int{2, 2, 6, 6}, []int{1, 3, 2, 6}))
}
