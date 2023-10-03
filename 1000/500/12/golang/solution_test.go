package solution

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestNumIdenticalPairs1(t *testing.T) {
	assert.Equal(t, 4, numIdenticalPairs([]int{1, 2, 3, 1, 1, 3}))
}

func TestNumIdenticalPairs2(t *testing.T) {
	assert.Equal(t, 6, numIdenticalPairs([]int{1, 1, 1, 1}))
}

func TestNumIdenticalPairs3(t *testing.T) {
	assert.Equal(t, 0, numIdenticalPairs([]int{1, 2, 3}))
}
