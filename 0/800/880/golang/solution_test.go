package solution

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSolution1(t *testing.T) {
	assert.Equal(t, "o", decodeAtIndex("leet2code3", 10))
}
func TestSolution2(t *testing.T) {
	assert.Equal(t, "h", decodeAtIndex("ha22", 5))
}
func TestSolution3(t *testing.T) {
	assert.Equal(t, "a", decodeAtIndex("a2345678999999999999999", 1))
}