use itertools::Itertools;
use std::fs;

fn parse_input(input: &str) -> (&str, Vec<Vec<char>>) {
    // split initial input into the map of the cargo and the instruction set
    let (stack_map, moves) = input.split("\n\n").collect_tuple().unwrap();
    // get the number of columns making up all the stacks
    let columns = (stack_map.find('\n').unwrap() + 1) / 4;
    // initialize a vector containing all the stacks, each containing the
    // character from that box in the given stack
    let mut stacks: Vec<Vec<char>> = vec![vec![]; columns];

    for line in stack_map.lines() {
        for column in 0..columns {
            // if the character in the column is a letter push it
            // onto the proper stack in the vector of stacks
            let character = line.chars().nth(column * 4 + 1).unwrap();
            if character.is_alphabetic() {
                stacks[column].push(character)
            }
        }
    }
    // reverse each of the stacks so that when calling push() on the stack
    // a character is added to what would be the top of the stack instead of the back
    for stack in &mut stacks {
        stack.reverse();
    }

    (moves, stacks)
}

fn execute_moves_part_one(moves: &str, stacks: &[Vec<char>]) -> String {
    let mut stacks = stacks.to_owned();

    moves
        .lines()
        // convert each line into a tuple of the three values need to perform
        // a given operation: amount, origin, and destination
        .map(|line| {
            line.split_whitespace()
                .filter(|word| word.chars().any(|character| character.is_numeric()))
                .map(|word| word.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        // for each parsed instruction, get the value of the origin and push it onto
        // the destination the amount of times specified
        .for_each(|(amount, origin, destination)| {
            for _ in 0..amount {
                let temp = stacks[origin - 1].pop().unwrap();
                stacks[destination - 1].push(temp);
            }
        });
    // collect the first element of each stack into the return type of String
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn execute_moves_part_two(moves: &str, stacks: &[Vec<char>]) -> String {
    let mut stacks = stacks.to_owned();

    moves
        .lines()
        // convert each line into a tuple of the three values need to perform
        // a given operation: amount, origin, and destination
        .map(|line| {
            line.split_whitespace()
                .filter(|word| word.chars().any(|character| character.is_numeric()))
                .map(|word| word.parse::<usize>().unwrap())
                .collect_tuple::<(usize, usize, usize)>()
                .unwrap()
        })
        // for each parsed instruction, collect the elements to transfer into a 
        // temperatory vector that is then appended to the destination vector
        .for_each(|(amount, origin, destination)| {
            let length = stacks[origin - 1].len();
            let mut temp: Vec<char> =
                stacks[origin - 1].drain((length - amount)..).collect();
            stacks[destination - 1].append(&mut temp);
        });

    // collect the first element of each stack into the return type of String
    stacks
        .into_iter()
        .map(|mut stack| stack.pop().unwrap())
        .collect()
}

fn main() {
    let path = "../inputs/day5.txt";
    let input = fs::read_to_string(path).expect("File Read Error");
    let (moves, stacks) = parse_input(&input);
    println!("Part 1:\n{}", execute_moves_part_one(moves, &stacks));
    println!("Part 2:\n{}", execute_moves_part_two(moves, &stacks));
}
