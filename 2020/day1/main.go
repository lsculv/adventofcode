package main

import (
	"errors"
	"fmt"
	"io"
	"log"
	"os"
	"strconv"
	"strings"
)

func part_1(input string) (int, error) {
	e := log.New(os.Stderr, "Error: ", 0)

	var num1 int
	var num2 int
	var err error

	for _, i := range strings.Split(input, "\n") {
		num1, err = strconv.Atoi(i)
		if err != nil {
			e.Println(err)
		}

		for _, j := range strings.Split(input, "\n") {
			num2, err = strconv.Atoi(j)
			if err != nil {
				e.Println(err)
			}

			if num1+num2 == 2020 {
				return num1 * num2, nil
			}
		}

	}
	return -1, errors.New("Could not find two ints that sum to 2020")
}

func part_2(input string) (int, error) {
	e := log.New(os.Stderr, "Error: ", 0)

	var num1 int
	var num2 int
	var num3 int
	var err error

	for _, i := range strings.Split(input, "\n") {
		num1, err = strconv.Atoi(i)
		if err != nil {
			e.Println(err)
		}

		for _, j := range strings.Split(input, "\n") {
			num2, err = strconv.Atoi(j)
			if err != nil {
				e.Println(err)
			}

			for _, k := range strings.Split(input, "\n") {
				num3, err = strconv.Atoi(k)
				if err != nil {
					e.Println(err)
				}

				if num1+num2+num3 == 2020 {
					return num1 * num2 * num3, nil
				}
			}

		}

	}
	return -1, errors.New("Could not find ints that sum to 2020")
}

func main() {
	e := log.New(os.Stderr, "Error: ", 0)

	f, err := os.Open("../inputs/day1.txt")
	defer f.Close()
	if err != nil {
		e.Println(err)
	}

	bytes, err := io.ReadAll(f)
	if err != nil {
		e.Println(err)
	}

	input := string(bytes)
	input = strings.TrimSuffix(input, "\n")

	part_1_result, err := part_1(input)
	if err != nil {
		e.Println(err)
	}
	part_2_result, err := part_2(input)
	if err != nil {
		e.Println(err)
	}

	fmt.Printf("Part 1: %v\n", part_1_result)
	fmt.Printf("Part 2: %v\n", part_2_result)
	return
}
