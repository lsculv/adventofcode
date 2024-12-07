#![feature(slice_split_once)]

use std::{
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;
use rayon::prelude::*;

fn to_base3(mut n: usize) -> Vec<u8> {
    if n == 0 {
        return vec![b'0'; 64];
    }

    let mut base3 = Vec::new();

    while n > 0 {
        base3.push(b'0' + (n % 3) as u8);
        n /= 3;
    }

    let pad_len = 64 - base3.len();
    base3
        .into_iter()
        .chain((0..pad_len).map(|_| b'0'))
        .collect()
}

trait Concat {
    fn concat(self, rhs: Self) -> Self;
}

impl Concat for u64 {
    fn concat(self, rhs: Self) -> Self {
        let mut multiplier = 1;
        let mut temp = rhs;

        while temp > 0 {
            multiplier *= 10;
            temp /= 10;
        }

        self * multiplier + rhs
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
            let is_possible = (0..possible_combinations).any(|comb| {
                values
                    .iter()
                    .copied()
                    .enumerate()
                    .reduce(|(_, acc), (i, n)| {
                        if ((comb >> (i - 1)) & 1) == 0 {
                            (0, acc + n)
                        } else {
                            (0, acc * n)
                        }
                    })
                    .map(|(_, e)| e)
                    .unwrap()
                    == target
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
            let is_possible = (0..possible_combinations)
                .map(|comb| to_base3(comb))
                .any(|comb| {
                    values
                        .iter()
                        .copied()
                        .enumerate()
                        .reduce(|(_, acc), (i, n)| match comb.get(i - 1) {
                            Some(b'0') => (0, acc + n),
                            Some(b'1') => (0, acc * n),
                            Some(b'2') => (0, acc.concat(n)),
                            e => unreachable!("{e:?}"),
                        })
                        .map(|(_, e)| e)
                        .unwrap()
                        == target
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
