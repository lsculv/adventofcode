use itertools::Itertools;
use std::str::from_utf8;

fn part_1(input: &[u8]) -> u64 {
    let mut last = u64::MAX;
    input
        .split(|b| *b == b'\n')
        .filter_map(|s| from_utf8(s).ok().unwrap().parse().ok())
        .map(|num| match num > last {
            true => {
                last = num;
                1u64
            }
            false => {
                last = num;
                0u64
            }
        })
        .sum()
}

fn part_2(input: &[u8]) -> u64 {
    let mut last = u64::MAX;
    input
        .split(|b| *b == b'\n')
        .filter_map(|s| from_utf8(s).ok().unwrap().parse().ok())
        .tuple_windows::<(u64, u64, u64)>()
        .map(|(n1, n2, n3)| match (n1 + n2 + n3) > last {
            true => {
                last = n1 + n2 + n3;
                1u64
            }
            false => {
                last = n1 + n2 + n3;
                0u64
            }
        })
        .sum()
}

fn main() {
    let input = include_bytes!("../../inputs/day1.txt");
    println!("Part 1:\n{}", part_1(input));
    println!("Part 2:\n{}", part_2(input))
}
