use core::ops::RangeInclusive;
use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

fn priority_hashmap() -> HashMap<char, u8> {
    // Create ranges over all lower and upper case letters
    let lower_case_letters: RangeInclusive<char> = 'a'..='z';
    let upper_case_letters: RangeInclusive<char> = 'A'..='Z';

    // Generate hashmap of characters and priorities
    lower_case_letters
        // convert ascii value of lower case characers to priority value
        .map(|character| (character, character as u8 - 96))
        // convert upper case characters from ascii
        .chain(upper_case_letters.map(|character| (character, character as u8 - 38)))
        .collect::<HashMap<char, u8>>()
}

fn parse_rucksack_part_one(rucksack: &str) -> i64 {
    let priorities = priority_hashmap();

    let mut priority_sum = 0;
    // iterate through each line of supplied data
    for line in rucksack.lines() {
        // split string into two equal halves
        let compartments = line.split_at(line.len() / 2);
        // iterate through the characters of each string
        // adding to the total based on the first match found
        'outer: for first_char in compartments.0.chars() {
            for second_char in compartments.1.chars() {
                if first_char == second_char {
                    priority_sum += *priorities.get(&first_char).unwrap() as i64;
                    break 'outer;
                }
            }
        }
    }
    return priority_sum;
}

fn parse_rucksack_part_two(rucksack: &str) -> i64 {
    let priorities = priority_hashmap();

    let mut priority_sum = 0;
    // iterate over characters of each string and find common character among them
    // and add total based on match found
    for (member_one, member_two, member_three) in rucksack.lines().tuples() {
        'outer: for one_char in member_one.chars() {
            for two_char in member_two.chars() {
                for three_char in member_three.chars() {
                    if one_char == two_char && one_char == three_char {
                        priority_sum += *priorities.get(&one_char).unwrap() as i64;
                        break 'outer;
                    }
                }
            }
        }
    }
    return priority_sum;
}

fn main() {
    let path = "../inputs/day3.txt";
    let rucksack = fs::read_to_string(path).expect("File Read Error");

    let priority_sum_part_one = parse_rucksack_part_one(&rucksack);
    println!("Part 1: {}", priority_sum_part_one);

    let priority_sum_part_two = parse_rucksack_part_two(&rucksack);
    println!("Part 2: {}", priority_sum_part_two);
}
