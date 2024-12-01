package utils

import (
	"strconv"

	"golang.org/x/exp/constraints"
)

// Uses strconv.ParseInt to parse a string into an int64.
// Panics if the string cannot be parsed.
func ParseInt(s string, base int, bitSize int) int64 {
	num, err := strconv.ParseInt(s, base, bitSize)
	CheckErr(err)

	return num
}

func AbsDiff[T constraints.Unsigned](a, b T) T {
	if a > b {
		return a - b
	}

	return b - a
}
