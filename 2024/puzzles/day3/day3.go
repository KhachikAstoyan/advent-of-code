package day3

import (
	_ "embed"
	"errors"
	"strconv"
	"strings"
)

type Day3 struct{}

//go:embed input.txt
var input string

func (d Day3) Name() string {

	return "Day 2: Red-Nosed Reports"
}

func (d Day3) Part1() uint64 {
	i := 0
	sum := uint64(0)

outer:
	for i < len(input)-2 {
		if input[i:i+3] == "mul" {
			startIndex := i + 3
			endIndex := i + 4

			if input[startIndex] != '(' {
				i++
				continue
			}

			for input[endIndex] != ')' {
				if isParenthesis(input[endIndex]) {
					i++
					continue outer
				}

				endIndex++
			}
			prod, err := multiply(input[startIndex+1 : endIndex])
			if err != nil {
				i = endIndex
				continue
			}

			sum += prod
			i = endIndex
		}

		i++
	}

	return sum
}

func (d Day3) Part2() uint64 {
	i := 0
	sum := uint64(0)
	shouldDo := true

outer:
	for i < len(input)-2 {
		if isDoInstruction(input, i) {
			shouldDo = true
			i += 4
		} else if isDontInstruction(input, i) {
			shouldDo = false
			i += 6
		} else if input[i:i+3] == "mul" {
			startIndex := i + 3
			endIndex := i + 4

			if !shouldDo {
				i = endIndex
				continue
			}

			if input[startIndex] != '(' {
				i = endIndex
				continue
			}

			for input[endIndex] != ')' {
				if isParenthesis(input[endIndex]) {
					i++
					continue outer
				}

				endIndex++
			}
			prod, err := multiply(input[startIndex+1 : endIndex])
			if err != nil {
				i = endIndex
				continue
			}

			sum += prod
			i = endIndex
		} else {
			i++
		}

	}

	return sum
}

func multiply(match string) (uint64, error) {
	parts := strings.Split(match, ",")
	if len(parts) != 2 {
		return 0, errors.New("invalid match")
	}
	aStr := strings.TrimSpace(parts[0])
	bStr := strings.TrimSpace(parts[1])

	a, errA := strconv.ParseInt(aStr, 10, 64)
	b, errB := strconv.ParseInt(bStr, 10, 64)

	if errA != nil || errB != nil {
		return 0, errors.New("invalid match")
	}

	return uint64(a) * uint64(b), nil
}

func isDoInstruction(input string, i int) bool {
	return i+4 < len(input) && input[i:i+4] == "do()"
}

func isDontInstruction(input string, i int) bool {
	return i+7 < len(input) && input[i:i+7] == "don't()"
}

func isParenthesis(c byte) bool {
	return c == '(' || c == ')' || c == '[' || c == ']' || c == '{' || c == '}'
}
