package day4

import (
	_ "embed"
	"strings"
)

type Day4 struct{}

//go:embed input.txt
var input string

func (d Day4) Name() string {
	return "Day 4: Ceres Search"
}

func (d Day4) Part1() uint64 {
	grid := parseInput(input)
	rows := len(grid)
	cols := len(grid[0])
	total := 0

	for i := 0; i < rows; i++ {
		for j := 0; j < cols; j++ {
			if grid[i][j] == 'X' {
				if checkBackwards(grid, i, j) ||
					checkForwards(grid, i, j) ||
					checkDownwards(grid, i, j) {
					total++
				}
			}
		}
	}

	return uint64(total)
}

func (d Day4) Part2() uint64 {
	return 0
}

func checkBackwards(grid [][]rune, i, j int) bool {
	for k := j - 1; k >= j-3; k-- {
		if !isInBounds(i, k, grid) || !isMASChar(grid[i][k]) {
			return false
		}
	}
	return true
}

func checkForwards(grid [][]rune, i, j int) bool {
	for k := j + 1; j <= j+3; j++ {
		if !isInBounds(i, k, grid) || !isMASChar(grid[i][j]) {
			return false
		}
	}
	return true
}

func checkDownwards(grid [][]rune, i, j int) bool {
	for k := i + 1; k <= i+3; k++ {
		if !isInBounds(i, k, grid) || !isMASChar(grid[i][j]) {
			return false
		}
	}

	return true
}

func isMASChar(r rune) bool {
	return r == 'M' || r == 'A' || r == 'S'
}

func isInBounds(i, j int, grid [][]rune) bool {
	rows := len(grid)
	cols := len(grid[0])
	return i >= 0 && i < rows && j >= 0 && j < cols
}

func parseInput(input string) [][]rune {
	lines := strings.Split(input, "\n")
	grid := make([][]rune, len(lines))

	for i, line := range lines {
		grid[i] = []rune(line)
	}

	return grid
}
