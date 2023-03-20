use std::fs;

fn part_1(input: &str) -> i32 {
    let nums: Vec<i32> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut tmp: i32;
    let mut sol: i32 = std::i32::MAX;
    for i in 0..5000 {
        tmp = nums.clone().into_iter().map(|n| (n - i).abs()).sum();
        if tmp < sol {
            sol = tmp
        }
    }
    sol
}

fn part_2(input: &str) -> i64 {
    let nums: Vec<i64> = input
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut tmp: i64;
    let mut sol: i64 = std::i64::MAX;
    for i in 0..5000 {
        tmp = nums
            .clone()
            .into_iter()
            .map(|n| ((n - i).abs() * ((n - i).abs() + 1)) / 2)
            .sum();
        if tmp < sol {
            sol = tmp
        }
    }
    sol
}

fn main() {
    let input = fs::read_to_string("../inputs/day7.txt").expect("Could not open input file");
    println!("Part 1:\t{}", part_1(&input));
    println!("Part 2:\t{}", part_2(&input));
}
