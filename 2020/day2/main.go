package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

var e = log.New(os.Stderr, "Error: ", 0)

func part_1(input string) int {
	var bottom int
	var top int
	var char string
	var count int
	var err error

	valid_passwords := 0
	for _, line := range strings.Split(input, "\n") {
		count = 0
		split_line := strings.Split(line, " ")
		count_range := strings.Split(split_line[0], "-")
		bottom, err = strconv.Atoi(count_range[0])
		if err != nil {
			e.Println(err)
		}
		top, err = strconv.Atoi(count_range[1])
		if err != nil {
			e.Println(err)
		}

		char = string(split_line[1][0])

		for _, c := range strings.Split(split_line[2], "") {
			if c == char {
				count++
			}
		}

		if count >= bottom && count <= top {
			valid_passwords++
		}
	}
	return valid_passwords
}

func part_2(input string) int {
	var pos1 int
	var pos2 int
	var char byte
	var err error

	valid_passwords := 0
	for _, line := range strings.Split(input, "\n") {
		split_line := strings.Split(line, " ")
		positions := strings.Split(split_line[0], "-")
		pos1, err = strconv.Atoi(positions[0])
		if err != nil {
			e.Println(err)
		}
		pos2, err = strconv.Atoi(positions[1])
		if err != nil {
			e.Println(err)
		}

		char = split_line[1][0]

		if ((split_line[2][pos1-1] == char) && (split_line[2][pos2-1] != char)) ||
			((split_line[2][pos2-1] == char) && (split_line[2][pos1-1] != char)) {
			valid_passwords++
		}
	}

	return valid_passwords
}

func main() {

	file, err := os.Open("../inputs/day2.txt")
	defer file.Close()
	if err != nil {
		e.Println(err)
	}

	bytes, err := io.ReadAll(file)
	if err != nil {
		e.Println(err)
	}

	input := strings.TrimSuffix(string(bytes), "\n")

	fmt.Printf("Part 1: %v\n", part_1(input))
	fmt.Printf("Part 2: %v\n", part_2(input))
}
