package main

import (
	"os"

	"github.com/KhachikAstoyan/advent-of-code/2024/puzzles/day1"
	"github.com/KhachikAstoyan/advent-of-code/2024/puzzles/day2"
	"github.com/KhachikAstoyan/advent-of-code/2024/puzzles/day3"
	"github.com/KhachikAstoyan/advent-of-code/2024/utils"
)

func main() {
	args := os.Args[1:]
	days := []utils.Day{
		day1.Day1{},
		day2.Day2{},
		day3.Day3{},
	}

	if len(args) == 0 {
		for _, day := range days {
			result := utils.RunDay(day)
			utils.PrintDayResult(day, result)
		}
	} else {
		dayNum := int(utils.ParseInt(args[0], 10, 32))
		if dayNum < 1 || dayNum > len(days) {
			panic("Invalid day number")
		}

		day := days[dayNum-1]
		result := utils.RunDay(day)
		utils.PrintDayResult(day, result)
	}
}
