package main

import (
	"fmt"
	"io"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func part_1(input string) int {
	passports := strings.Split(input, "\n\n")

	split := func(r rune) bool {
		return r == ' ' || r == '\n'
	}

	var fields uint8
	valid_passports := 0
	for _, passport := range passports {
		fields = 0
		codes := strings.FieldsFunc(passport, split)
		for _, code := range codes {
			switch code[:3] {
			case "byr":
				fields |= 1 << 0
			case "iyr":
				fields |= 1 << 1
			case "eyr":
				fields |= 1 << 2
			case "hgt":
				fields |= 1 << 3
			case "hcl":
				fields |= 1 << 4
			case "ecl":
				fields |= 1 << 5
			case "pid":
				fields |= 1 << 6
			}
		}
		if fields == 0b01111111 {
			valid_passports += 1
		}
	}
	return valid_passports
}

func part_2(input string) int {
	passports := strings.Split(input, "\n\n")

	split := func(r rune) bool {
		return r == ' ' || r == '\n'
	}

	var fields uint8
	valid_passports := 0
	for _, passport := range passports {
		fields = 0
		codes := strings.FieldsFunc(passport, split)
		for _, code := range codes {
			switch code[:3] {
			case "byr":
				year, err := strconv.Atoi(code[4:])
				if err == nil && year >= 1920 && year <= 2002 {
					fields |= 1 << 0
				}
			case "iyr":
				year, err := strconv.Atoi(code[4:])
				if err == nil && year >= 2010 && year <= 2020 {
					fields |= 1 << 1
				}
			case "eyr":
				year, err := strconv.Atoi(code[4:])
				if err == nil && year >= 2020 && year <= 2030 {
					fields |= 1 << 2
				}
			case "hgt":
				unit := code[len(code)-2:]
				height, err := strconv.Atoi(code[4 : len(code)-2])
				if err != nil {
					break
				}
				if (unit == "cm" && height >= 150 && height <= 193) || (unit == "in" && height >= 59 && height <= 76) {
					fields |= 1 << 3
				}
			case "hcl":
				match, err := regexp.MatchString("^#[0-9a-f]{6}", code[4:])
				if err != nil {
					panic(err)
				}
				if match {
					fields |= 1 << 4
				}
			case "ecl":
				if code[4:] == "amb" ||
					code[4:] == "blu" ||
					code[4:] == "brn" ||
					code[4:] == "gry" ||
					code[4:] == "grn" ||
					code[4:] == "hzl" ||
					code[4:] == "oth" {
					fields |= 1 << 5
				}
			case "pid":
				match, err := regexp.MatchString("^[0-9]{9}$", code[4:])
				if err != nil {
					panic(err)
				}
				if match {
					fields |= 1 << 6
				}
			}
		}
		if fields == 0b01111111 {
			valid_passports += 1
		}
	}

	return valid_passports
}

func main() {
	file, err := os.Open("../inputs/day4.txt")
	defer file.Close()
	if err != nil {
		panic(err)
	}

	bytes, err := io.ReadAll(file)
	if err != nil {
		panic(err)
	}

	input := strings.TrimSuffix(string(bytes), "\n")

	fmt.Printf("Part 1: %v\n", part_1(input))
	fmt.Printf("Part 2: %v\n", part_2(input))
}
