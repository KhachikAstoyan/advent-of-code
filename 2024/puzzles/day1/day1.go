package day1

import (
	_ "embed"
	"slices"
	"strings"

	"github.com/KhachikAstoyan/advent-of-code/2024/utils"
)

type Day1 struct{}

//go:embed input.txt
var input string

func (d Day1) Name() string {
	return "Day 1: Historian Hysteria"
}

func (d Day1) Part1() uint64 {
	left, right := parseLists(strings.Split(input, "\n"))
	slices.Sort(left)
	slices.Sort(right)

	answer := uint64(0)
	for i := 0; i < len(left); i++ {
		answer += utils.AbsDiff(left[i], right[i])
	}

	return answer
}

func (d Day1) Part2() uint64 {
	left, right := parseLists(strings.Split(input, "\n"))
	m := make(map[uint64]uint64)

	for _, num := range left {
		m[num] = 0
	}

	for _, num := range right {
		if _, ok := m[num]; ok {
			m[num]++
		}
	}

	similarity := uint64(0)
	for num, count := range m {
		similarity += num * count
	}

	return similarity
}

func parseLists(lines []string) ([]uint64, []uint64) {
	n := len(lines)
	left := make([]uint64, n)
	right := make([]uint64, n)

	for i, line := range lines {
		parts := strings.Fields(line)

		leftNum := utils.ParseInt(parts[0], 10, 64)
		rightNum := utils.ParseInt(parts[1], 10, 64)

		left[i] = uint64(leftNum)
		right[i] = uint64(rightNum)
	}

	return left, right
}
