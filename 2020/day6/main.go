package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

const maxUint32 uint32 = 4294967295

func part1(input string) int {
	groups := strings.Split(input, "\n\n")

	count := 0
	for _, group := range groups {
		var yesAnswers [26]bool
		for i := 0; i < len(group); i++ {
			if group[i] != '\n' {
				yesAnswers[group[i]-97] = true
			}
		}

		for _, yesAnswer := range yesAnswers {
			if yesAnswer {
				count++
			}
		}
	}

	return count
}

func part2(input string) int {
	groups := strings.Split(input, "\n\n")

	count := 0
	for _, group := range groups {
		yesAnswers := maxUint32
		answers := strings.Split(group, "\n")

		for _, answer := range answers {
			var localAnswers uint32 = 0
			for i := 0; i < len(answer); i++ {
				localAnswers |= 1 << (answer[i] - 97)
			}
			yesAnswers &= localAnswers
		}

		for yesAnswers > 0 {
			count += int(yesAnswers & 1)
			yesAnswers >>= 1
		}
	}

	return count
}

func main() {
	file, err := os.Open("../inputs/day6.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}

	bytes, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}

	input := strings.TrimSuffix(string(bytes), "\n")

	fmt.Printf("Part 1: %v\n", part1(input))
	fmt.Printf("Part 2: %v\n", part2(input))
}
