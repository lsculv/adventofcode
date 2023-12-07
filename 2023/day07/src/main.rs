#![feature(slice_split_once)]
use atoi::atoi;
use std::cmp::Ordering;

fn main() {
    let input = include_bytes!("../day07.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let five = Vec::new();
    let four = Vec::new();
    let fullhouse = Vec::new();
    let three = Vec::new();
    let two_pair = Vec::new();
    let one_pair = Vec::new();
    let high_card = Vec::new();
    let mut hands = [high_card, one_pair, two_pair, three, fullhouse, four, five];

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

        add_hands(counts, hand_values, bid, 0, &mut hands);
    }

    rank_and_sum_hands(&mut hands)
}

fn part_2(input: &[u8]) -> usize {
    let five = Vec::new();
    let four = Vec::new();
    let fullhouse = Vec::new();
    let three = Vec::new();
    let two_pair = Vec::new();
    let one_pair = Vec::new();
    let high_card = Vec::new();
    let mut hands = [high_card, one_pair, two_pair, three, fullhouse, four, five];

    for line in input.split(|&b| b == b'\n') {
        let (hand, bid) = line.split_once(|&b| b == b' ').unwrap();
        let bid: usize = atoi(bid).unwrap();
        let mut counts = [0u8; 13];
        let mut jokers = 0u8;
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
                    jokers += 1;
                    hand_values[i] = 0
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
        add_hands(counts, hand_values, bid, jokers, &mut hands);
    }
    rank_and_sum_hands(&mut hands)
}

fn add_hands(
    mut counts: [u8; 13],
    hand_values: [u8; 5],
    bid: usize,
    jokers: u8,
    hands: &mut [Vec<([u8; 5], usize)>],
) {
    counts.sort_unstable();
    counts[12] += jokers;
    match counts[8..13] {
        [0, 0, 0, 0, 5] => hands[6].push((hand_values, bid)),
        [0, 0, 0, 1, 4] => hands[5].push((hand_values, bid)),
        [0, 0, 0, 2, 3] => hands[4].push((hand_values, bid)),
        [0, 0, 1, 1, 3] => hands[3].push((hand_values, bid)),
        [0, 0, 1, 2, 2] => hands[2].push((hand_values, bid)),
        [0, 1, 1, 1, 2] => hands[1].push((hand_values, bid)),
        [1, 1, 1, 1, 1] => hands[0].push((hand_values, bid)),
        _ => unreachable!(),
    }
}

fn rank_and_sum_hands(hands: &mut [Vec<([u8; 5], usize)>]) -> usize {
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
