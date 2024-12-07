#![feature(slice_split_once)]
#![feature(iterator_try_reduce)]

use std::{
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;
use rayon::prelude::*;

trait Concat {
    fn concat(self, rhs: Self) -> Self;
}

impl Concat for u64 {
    fn concat(self, rhs: Self) -> Self {
        self * 10u64.pow(rhs.ilog10() + 1) + rhs
    }
}

fn part1(input: &[u8]) -> u64 {
    input
        .par_split(|&b| b == b'\n')
        .filter_map(|line| {
            let (target, values) = line.split_once(|&b| b == b':').unwrap();
            let target: u64 = atoi(target).unwrap();
            let values: Vec<u64> = values[1..]
                .split(|&b| b == b' ')
                .map(|n| atoi(n).unwrap())
                .collect();
            let possible_combinations: usize = 2usize.pow(values.len() as u32 - 1);
            let is_possible = (0..possible_combinations).any(|mut comb| {
                let result = values.iter().copied().try_reduce(|acc, n| {
                    let ret = if comb % 2 == 0 { acc + n } else { acc * n };
                    if ret > target {
                        None
                    } else {
                        comb /= 2;
                        Some(ret)
                    }
                });
                match result {
                    None => false,
                    Some(Some(x)) => x == target,
                    _ => unreachable!(),
                }
            });
            if is_possible {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &[u8]) -> u64 {
    input
        .par_split(|&b| b == b'\n')
        .filter_map(|line| {
            let (target, values) = line.split_once(|&b| b == b':').unwrap();
            let target: u64 = atoi(target).unwrap();
            let values: Vec<u64> = values[1..]
                .split(|&b| b == b' ')
                .map(|n| atoi(n).unwrap())
                .collect();
            let possible_combinations: usize = 3usize.pow(values.len() as u32 - 1);
            let is_possible = (0..possible_combinations).any(|mut comb| {
                let result = values.iter().copied().try_reduce(|acc, n| {
                    let ret = if comb % 3 == 0 {
                        acc + n
                    } else if comb % 3 == 1 {
                        acc * n
                    } else {
                        acc.concat(n)
                    };
                    if ret > target {
                        None
                    } else {
                        comb /= 3;
                        Some(ret)
                    }
                });
                match result {
                    None => false,
                    Some(Some(x)) => x == target,
                    _ => unreachable!(),
                }
            });
            if is_possible {
                Some(target)
            } else {
                None
            }
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input_file = "day07.txt";
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
        let input = b"190: 10 19\n\
                      3267: 81 40 27\n\
                      83: 17 5\n\
                      156: 15 6\n\
                      7290: 6 8 6 15\n\
                      161011: 16 10 13\n\
                      192: 17 8 14\n\
                      21037: 9 7 18 13\n\
                      292: 11 6 16 20\n\
                      100000: 10 10 10 10 10";
        assert_eq!(part1(input), 3749 + 100_000);
    }

    #[test]
    fn test_part2() {
        let input = b"190: 10 19\n\
                      3267: 81 40 27\n\
                      83: 17 5\n\
                      156: 15 6\n\
                      7290: 6 8 6 15\n\
                      161011: 16 10 13\n\
                      192: 17 8 14\n\
                      21037: 9 7 18 13\n\
                      292: 11 6 16 20";
        assert_eq!(part2(input), 11387);
    }
}
