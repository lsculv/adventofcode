package main

import (
	"fmt"
	"io"
	"log"
	"os"
	"strings"
)

var e = log.New(os.Stderr, "Error: ", 0)

func part_1(input string) int {
	lines := strings.Split(input, "\n")

	col := 0
	trees_hit := 0
	for i := 0; i < len(lines); i++ {
		if lines[i][col%31] == '#' {
			trees_hit++
		}

		col += 3
	}

	return trees_hit
}

func part_2(input string) int {
	lines := strings.Split(input, "\n")

	var col [5]int
	var trees_hit [5]int
	for i := 0; i < len(lines); i++ {
		if lines[i][col[0]%31] == '#' {
			trees_hit[0]++
		}
		col[0] += 1

		if lines[i][col[1]%31] == '#' {
			trees_hit[1]++
		}
		col[1] += 3

		if lines[i][col[2]%31] == '#' {
			trees_hit[2]++
		}
		col[2] += 5

		if lines[i][col[3]%31] == '#' {
			trees_hit[3]++
		}
		col[3] += 7

	}

    for i := 0; i < len(lines); i += 2 {
		if lines[i][col[4]%31] == '#' {
			trees_hit[4]++
		}
		col[4] += 1
    }
    
	return trees_hit[0] * trees_hit[1] * trees_hit[2] * trees_hit[3] * trees_hit[4]
}

func main() {
	file, err := os.Open("../inputs/day3.txt")
	defer file.Close()
	if err != nil {
		e.Println(err)
	}

	bytes, err := io.ReadAll(file)
	if err != nil {
		e.Println(err)
	}

	input := string(bytes)
	input = strings.TrimSuffix(input, "\n")

    fmt.Printf("%v\n", 70 * 240 * 68 * 67 * 37)

	fmt.Printf("Part 1: %v\n", part_1(input))
	fmt.Printf("Part 2: %v\n", part_2(input))
}
