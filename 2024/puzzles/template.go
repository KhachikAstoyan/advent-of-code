package main

type DayTemplate struct{}

func (d DayTemplate) Name() string {
	return "Template"
}

func (d DayTemplate) Part1() uint64 {
	return 0
}

func (d DayTemplate) Part2() uint64 {
	return 0
}

func (d DayTemplate) Part1Correct(val uint64) bool {
	return val == 0
}

func (d DayTemplate) Part2Correct(val uint64) bool {
	return val == 0
}
