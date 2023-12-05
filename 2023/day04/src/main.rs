#![feature(slice_split_once)]
#![feature(byte_slice_trim_ascii)]

use std::collections::HashSet;

fn main() {
    let input = include_bytes!("../day04.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    count_matches(input)
        .iter()
        .map(|&m| {
            if m == 0 {
                0
            } else {
                let m: u32 = m.try_into().unwrap();
                2usize.pow(m - 1)
            }
        })
        .sum()
}

fn part_2(input: &[u8]) -> usize {
    let matches = count_matches(input);
    let mut copies = vec![1; matches.len()];
    for i in 0..copies.len() {
        for j in (i + 1)..=(i + matches[i]) {
            copies[j] += copies[i];
        }
    }

    copies.iter().sum()
}

fn count_matches(cards: &[u8]) -> Vec<usize> {
    let mut matches = Vec::with_capacity(cards.split(|&b| b == b'\n').count() + 1);
    for line in cards.split(|&b| b == b'\n') {
        let (_, numbers) = line.split_once(|&b| b == b':').unwrap();
        let (winners, numbers) = numbers.split_once(|&b| b == b'|').unwrap();
        let winners: HashSet<&[u8]> = winners
            .trim_ascii()
            .split(|&b| b == b' ')
            .filter(|s| s != b"") // Some numbers are separated by two spaces.
            .collect();

        let numbers: HashSet<&[u8]> = numbers
            .trim_ascii()
            .split(|&b| b == b' ')
            .filter(|s| s != b"")
            .collect();

        let overlap = winners.intersection(&numbers).count();
        matches.push(overlap);
    }

    matches
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = b"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_1(input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = b"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part_2(input), 30);
    }
}
