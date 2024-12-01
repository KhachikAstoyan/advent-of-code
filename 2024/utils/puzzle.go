package utils

import (
	"time"
)

type Day interface {
	Name() string
	Part1() uint64
	Part2() uint64
}

type DayResult struct {
	Part1     uint64
	Part2     uint64
	Part1Time time.Duration
	Part2Time time.Duration
}

func RunDay(day Day) DayResult {
	result := DayResult{}

	p1Start := time.Now()
	result.Part1 = day.Part1()
	result.Part1Time = time.Since(p1Start)

	p2Start := time.Now()
	result.Part2 = day.Part2()
	result.Part2Time = time.Since(p2Start)

	return result
}

func PrintDayResult(day Day, result DayResult) {
	println("--------------------------------")
	println(day.Name())
	println("Part 1:", result.Part1, "Time:", FormatDuration(result.Part1Time))
	println("Part 2:", result.Part2, "Time:", FormatDuration(result.Part2Time))
	println("--------------------------------")
}
