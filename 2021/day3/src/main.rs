use std::fs;

fn part_1(input: &str) -> u32 {
    const LINE_LENGTH: usize = 12;
    let mut columns: [(u32, u32); 12] = [(0, 0); 12];
    for (i, c) in input.chars().filter(|c| c != &'\n').enumerate() {
        match c {
            '0' => columns[i % LINE_LENGTH].0 += 1,
            '1' => columns[i % LINE_LENGTH].1 += 1,
            _ => (),
        }
    }
    let mut gamma = 0u32;
    let mut epsilon = 0u32;
    for (i, column) in columns.iter().enumerate() {
        if column.0 > column.1 {
            epsilon |= 1 << (LINE_LENGTH - (i + 1));
        } else {
            gamma |= 1 << (LINE_LENGTH - (i + 1));
        }
    }
    gamma * epsilon
}

fn part_2(input: &str) -> u32 {
    const LINE_LENGTH: usize = 12;
    let mut columns: [(u32, u32); 12] = [(0, 0); 12];
    for (i, c) in input.chars().filter(|c| c != &'\n').enumerate() {
        match c {
            '0' => columns[i % LINE_LENGTH].0 += 1,
            '1' => columns[i % LINE_LENGTH].1 += 1,
            _ => (),
        }
    }
    let mut lines: Vec<&str> = input.lines().collect();
    let mut column = 0;
    let mut max: char;
    while lines.len() > 1 {
        println!("{:?}", lines.len());
        for (i, line) in lines.clone().iter().enumerate() {
            if columns[column].0 > columns[column].1 {
                max = '0'
            } else {
                max = '1'
            }

            if line.chars().nth(column).unwrap() != max {
                lines.remove(i);
            }
        }
        column += 1;
    }
    let oxygen_rating: u32 = lines.pop().unwrap().parse().unwrap();
    let mut lines: Vec<&str> = input.lines().collect();
    column = 0;
    while lines.len() > 1 {
        println!("{:?}", lines.len());
        for (i, line) in lines.clone().iter().enumerate() {
            if columns[column].0 < columns[column].1 {
                max = '1'
            } else {
                max = '0'
            }

            if line.chars().nth(column).unwrap() == max {
                lines.remove(i);
            }
        }
        column += 1;
    }
    let co2_rating: u32 = lines.pop().unwrap().parse().unwrap();
    oxygen_rating * co2_rating
}

fn main() {
    let input = fs::read_to_string("../inputs/day3.txt").unwrap();
    println!("Part 1:\n{}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
}
