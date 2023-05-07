package main

import (
	"fmt"
	"io"
	"os"
	"strings"
)

const passLength = 10

func parse(input string) []int {
	lines := strings.Split(input, "\n")

	var seatIDs []int

	for _, line := range lines {
		front := 127
		back := 0
		left := 0
		right := 7
		for i := 0; i < passLength-3; i++ {
			switch line[i] {
			case 'B':
				back = (front+back)/2 + 1
			case 'F':
				front = (front + back) / 2
			}
		}

		for i := 7; i < passLength; i++ {
			switch line[i] {
			case 'R':
				left = (right+left)/2 + 1
			case 'L':
				right = (right + left) / 2
			}
		}

		seatIDs = append(seatIDs, front*8+right)
	}

	return seatIDs
}

func part1(seatIDs []int) int {
	maxID := 0
	for _, n := range seatIDs {
		if n > maxID {
			maxID = n
		}
	}

	return maxID
}

func part2(seatIDs []int) int {
	var containedIDs [919 + 3]bool

	for _, id := range seatIDs {
		containedIDs[id] = true
	}

	for id := range seatIDs {
		if containedIDs[id] && containedIDs[id+2] && !containedIDs[id+1] {
			return id + 1
		}
	}

	return -1
}

func main() {
	file, err := os.Open("../inputs/day5.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}

	bytes, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}

	input := strings.TrimSuffix(string(bytes), "\n")
	seatIDs := parse(input)

	fmt.Printf("Part 1: %v\n", part1(seatIDs))
	fmt.Printf("Part 2: %v\n", part2(seatIDs))
}
