use std::fs;

fn part_1(input: &str) -> i64 {
    let mut horizontal_postion = 0i64;
    let mut depth = 0i64;
    input.lines().for_each(|line| {
        let (direction, value) = line.split_once(' ').unwrap();
        match direction {
            "up" => depth -= value.parse::<i64>().unwrap(),
            "down" => depth += value.parse::<i64>().unwrap(),
            "forward" => horizontal_postion += value.parse::<i64>().unwrap(),
            _ => (),
        }
    });
    depth * horizontal_postion
}

fn part_2(input: &str) -> i64 {
    let mut horizontal_postion = 0i64;
    let mut depth = 0i64;
    let mut aim = 0i64;
    input.lines().for_each(|line| {
        let (command, value) = line.split_once(' ').unwrap();
        match command {
            "up" => aim -= value.parse::<i64>().unwrap(),
            "down" => aim += value.parse::<i64>().unwrap(),
            "forward" => {
                horizontal_postion += value.parse::<i64>().unwrap();
                depth += aim * value.parse::<i64>().unwrap();
            }
            _ => (),
        }
    });
    depth * horizontal_postion
}

fn main() {
    let input = fs::read_to_string("../inputs/day2.txt").unwrap();
    println!("Part 1:\n{}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
}
