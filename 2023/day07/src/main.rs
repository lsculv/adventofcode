#![feature(slice_split_once)]
use atoi::atoi;
use std::cmp::Ordering;

fn main() {
    let input = include_bytes!("../day07.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let mut five = Vec::new();
    let mut four = Vec::new();
    let mut fullhouse = Vec::new();
    let mut three = Vec::new();
    let mut two_pair = Vec::new();
    let mut one_pair = Vec::new();
    let mut high_card = Vec::new();

    for line in input.split(|&b| b == b'\n') {
        let (hand, bid) = line.split_once(|&b| b == b' ').unwrap();
        let bid: usize = atoi(bid).unwrap();
        let mut counts = [0u8; 13];
        let mut hand_values = [0u8; 5];
        for (i, byte) in hand.iter().enumerate() {
            match byte {
                num @ b'2'..=b'9' => {
                    counts[(num - b'2') as usize] += 1;
                    hand_values[i] = num - b'0'
                }
                b'T' => {
                    counts[8] += 1;
                    hand_values[i] = 10;
                }
                b'J' => {
                    counts[9] += 1;
                    hand_values[i] = 11
                }
                b'Q' => {
                    counts[10] += 1;
                    hand_values[i] = 12;
                }
                b'K' => {
                    counts[11] += 1;
                    hand_values[i] = 13;
                }
                b'A' => {
                    counts[12] += 1;
                    hand_values[i] = 14
                }
                _ => unreachable!(),
            }
        }

        counts.sort_unstable();
        match counts[8..13] {
            [0, 0, 0, 0, 5] => five.push((hand_values, bid)),
            [0, 0, 0, 1, 4] => four.push((hand_values, bid)),
            [0, 0, 0, 2, 3] => fullhouse.push((hand_values, bid)),
            [0, 0, 1, 1, 3] => three.push((hand_values, bid)),
            [0, 0, 1, 2, 2] => two_pair.push((hand_values, bid)),
            [0, 1, 1, 1, 2] => one_pair.push((hand_values, bid)),
            [1, 1, 1, 1, 1] => high_card.push((hand_values, bid)),
            _ => unreachable!(),
        }
    }

    let mut hands = [high_card, one_pair, two_pair, three, fullhouse, four, five];

    for hand in hands.iter_mut() {
        hand.sort_unstable_by(|(a, _), (b, _)| {
            for (a, b) in a.iter().zip(b.iter()) {
                let cmp = a.cmp(b);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });
    }

    hands
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}

fn part_2(input: &[u8]) -> usize {
    let mut five = Vec::new();
    let mut four = Vec::new();
    let mut fullhouse = Vec::new();
    let mut three = Vec::new();
    let mut two_pair = Vec::new();
    let mut one_pair = Vec::new();
    let mut high_card = Vec::new();

    for line in input.split(|&b| b == b'\n') {
        let (hand, bid) = line.split_once(|&b| b == b' ').unwrap();
        let bid: usize = atoi(bid).unwrap();
        let mut counts: [(u8, u8); 13] = [
            (0, b'2'),
            (0, b'3'),
            (0, b'4'),
            (0, b'5'),
            (0, b'6'),
            (0, b'7'),
            (0, b'8'),
            (0, b'9'),
            (0, b'T'),
            (0, b'J'),
            (0, b'Q'),
            (0, b'K'),
            (0, b'A'),
        ];
        let mut hand_values = [0u8; 5];
        for (i, byte) in hand.iter().enumerate() {
            match byte {
                num @ b'2'..=b'9' => {
                    counts[(num - b'2') as usize].0 += 1;
                    hand_values[i] = num - b'0'
                }
                b'T' => {
                    counts[8].0 += 1;
                    hand_values[i] = 10;
                }
                b'J' => {
                    counts[9].0 += 1;
                    hand_values[i] = 0
                }
                b'Q' => {
                    counts[10].0 += 1;
                    hand_values[i] = 12;
                }
                b'K' => {
                    counts[11].0 += 1;
                    hand_values[i] = 13;
                }
                b'A' => {
                    counts[12].0 += 1;
                    hand_values[i] = 14
                }
                _ => unreachable!(),
            }
        }

        counts.sort_unstable_by(|(a, _), (b, _)| a.cmp(b));
        match counts[8..13] {
            [_, _, _, _, (5, _)] => five.push((hand_values, bid)),
            [_, _, _, (1, _), (4, b'J')] => five.push((hand_values, bid)),
            [_, _, _, (1, b'J'), (4, _)] => five.push((hand_values, bid)),
            [_, _, _, (1, _), (4, _)] => four.push((hand_values, bid)),
            [_, _, _, (2, _), (3, b'J')] => five.push((hand_values, bid)),
            [_, _, _, (2, b'J'), (3, _)] => five.push((hand_values, bid)),
            [_, _, _, (2, _), (3, _)] => fullhouse.push((hand_values, bid)),
            [_, _, (1, _), (1, _), (3, b'J')] => four.push((hand_values, bid)),
            [_, _, (1, _), (1, b'J'), (3, _)] => four.push((hand_values, bid)),
            [_, _, (1, b'J'), (1, _), (3, _)] => four.push((hand_values, bid)),
            [_, _, (1, _), (1, _), (3, _)] => three.push((hand_values, bid)),
            [_, _, (1, _), (2, _), (2, b'J')] => four.push((hand_values, bid)),
            [_, _, (1, _), (2, b'J'), (2, _)] => four.push((hand_values, bid)),
            [_, _, (1, b'J'), (2, _), (2, _)] => fullhouse.push((hand_values, bid)),
            [_, _, (1, _), (2, _), (2, _)] => two_pair.push((hand_values, bid)),
            [_, (1, _), (1, _), (1, _), (2, b'J')] => three.push((hand_values, bid)),
            [_, (1, _), (1, _), (1, b'J'), (2, _)] => three.push((hand_values, bid)),
            [_, (1, _), (1, b'J'), (1, _), (2, _)] => three.push((hand_values, bid)),
            [_, (1, b'J'), (1, _), (1, _), (2, _)] => three.push((hand_values, bid)),
            [_, (1, _), (1, _), (1, _), (2, _)] => one_pair.push((hand_values, bid)),
            [(1, _), (1, _), (1, _), (1, _), (1, b'J')] => one_pair.push((hand_values, bid)),
            [(1, _), (1, _), (1, _), (1, b'J'), (1, _)] => one_pair.push((hand_values, bid)),
            [(1, _), (1, _), (1, b'J'), (1, _), (1, _)] => one_pair.push((hand_values, bid)),
            [(1, _), (1, b'J'), (1, _), (1, _), (1, _)] => one_pair.push((hand_values, bid)),
            [(1, b'J'), (1, _), (1, _), (1, _), (1, _)] => one_pair.push((hand_values, bid)),
            [(1, _), (1, _), (1, _), (1, _), (1, _)] => high_card.push((hand_values, bid)),
            _ => unreachable!(),
        }
    }

    let mut hands = [high_card, one_pair, two_pair, three, fullhouse, four, five];

    for hand in hands.iter_mut() {
        hand.sort_unstable_by(|(a, _), (b, _)| {
            for (a, b) in a.iter().zip(b.iter()) {
                let cmp = a.cmp(b);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
            Ordering::Equal
        });
    }

    hands
        .iter()
        .flatten()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i + 1))
        .sum()
}
#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = b"32T3K 765\n\
                           T55J5 684\n\
                           KK677 28\n\
                           KTJJT 220\n\
                           QQQJA 483";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 5905);
    }
}
