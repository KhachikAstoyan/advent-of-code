package day3

import (
	_ "embed"
	"regexp"

	"github.com/KhachikAstoyan/advent-of-code/2024/utils"
)

type Day3 struct{}

//go:embed input.txt
var input string

func (d Day3) Name() string {
	return "Day 2: Red-Nosed Reports"
}

var multiplyRegex *regexp.Regexp = regexp.MustCompile(`mul\((\d+),(\d+)\)`)

func (d Day3) Part1() uint64 {
	matches := multiplyRegex.FindAllStringSubmatch(input, -1)
	sum := uint64(0)

	for _, match := range matches {
		left := utils.ParseInt(match[1], 10, 64)
		right := utils.ParseInt(match[2], 10, 64)

		sum += uint64(left) * uint64(right)
	}

	return sum
}

func (d Day3) Part2() uint64 {

	return 0
}
