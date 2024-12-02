use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;

fn parse(input: &[u8]) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let input = input.trim_ascii_end();

    for line in input.split(|&b| b == b'\n') {
        let first_space = line.iter().position(|&b| b == b' ').unwrap();
        let left_num: u32 = atoi(&line[..first_space]).unwrap();
        let right_num: u32 = atoi(&line[(first_space + 3)..]).unwrap();
        left.push(left_num);
        right.push(right_num);
    }

    (left, right)
}

fn part1(input: &[u8]) -> u32 {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn part2(input: &[u8]) -> u32 {
    let (left, right) = parse(input);

    let mut counts = HashMap::new();
    for num in right {
        counts
            .entry(num)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    left.iter()
        .map(|num| {
            let count = *counts.get(num).unwrap_or(&0);
            num * count
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input_file = "day01.txt";
    let mut file = File::open(input_file)?;

    let file_length = fs::metadata(input_file)?.len() as usize;
    let mut input = Vec::with_capacity(file_length);
    let _ = file.read_to_end(&mut input)?;

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part1() {
        let input = b"3   4\n\
                      4   3\n\
                      2   5\n\
                      1   3\n\
                      3   9\n\
                      3   3";
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn test_part2() {
        let input = b"3   4\n\
                      4   3\n\
                      2   5\n\
                      1   3\n\
                      3   9\n\
                      3   3";
        assert_eq!(part2(input), 31);
    }
}
