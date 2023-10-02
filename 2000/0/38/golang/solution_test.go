package solution

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestWinnerOfGame1(t *testing.T) {
	assert.Equal(t, true, winnerOfGame("AAABABB"))
}

func TestWinnerOfGame2(t *testing.T) {
	assert.Equal(t, false, winnerOfGame("AA"))
}

func TestWinnerOfGame3(t *testing.T) {
	assert.Equal(t, false, winnerOfGame("ABBBBBBBAAA"))
}
