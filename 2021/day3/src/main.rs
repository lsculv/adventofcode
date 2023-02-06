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
    fn find_line(mut lines: Vec<&str>, most_common: bool) -> u32 {
        let mut i = 0;

        while lines.len() > 1 {
            let (ones, zeros): (Vec<&str>, Vec<&str>) =
                lines.iter().partition(|s| s.chars().nth(i).unwrap() == '1');
            if (ones.len() >= zeros.len()) == most_common {
                lines = ones;
            } else {
                lines = zeros;
            }
            i += 1
        }
        u32::from_str_radix(lines.pop().unwrap(), 2).unwrap()
    }

    let lines: Vec<&str> = input.lines().collect();
    let oxygen_rating = find_line(lines.clone(), true);
    let co2_rating = find_line(lines.clone(), false);

    oxygen_rating * co2_rating
}

fn main() {
    let input = fs::read_to_string("../inputs/day3.txt").unwrap();
    println!("Part 1:\n{}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));
}
