#![feature(array_chunks)]

use std::{
    cmp::Ordering,
    collections::{HashMap, LinkedList},
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;

type PageMap = HashMap<u16, Vec<u16>>;

fn part1(input: &[u8]) -> u32 {
    let updates_pos = input
        .windows(2)
        .position(|slice| matches!(slice, b"\n\n"))
        .map(|i| i + 2)
        .expect("Found section break");

    let mut map: PageMap = HashMap::new();

    for &[n1, n2, _, n3, n4, _] in input[..(updates_pos - 1)].array_chunks::<6>() {
        let value = u16::from_le_bytes([n1, n2]);
        let key = u16::from_le_bytes([n3, n4]);
        map.entry(key)
            .and_modify(|vec| vec.push(value))
            .or_insert(vec![value]);
    }

    input[updates_pos..]
        .split(|&b| b == b'\n')
        .map(|line| {
            let pages = line
                .split(|&b| b == b',')
                .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
                .collect::<Vec<u16>>();
            let mut no_errors = true;
            for (i, page) in pages.iter().enumerate() {
                let Some(rules) = map.get(page) else {
                    continue;
                };
                if pages.iter().skip(i).any(|p| rules.contains(p)) {
                    no_errors = false;
                    break;
                }
            }
            let middle = pages[pages.len() / 2];
            no_errors as u32 * atoi::<u32>(&middle.to_le_bytes()).expect("Bytes are valid")
        })
        .sum()
}

fn part2(input: &[u8]) -> u32 {
    let updates_pos = input
        .windows(2)
        .position(|slice| matches!(slice, b"\n\n"))
        .map(|i| i + 2)
        .expect("Found section break");

    let mut map: PageMap = HashMap::new();

    for &[n1, n2, _, n3, n4, _] in input[..(updates_pos - 1)].array_chunks::<6>() {
        let value = u16::from_le_bytes([n1, n2]);
        let key = u16::from_le_bytes([n3, n4]);
        map.entry(key)
            .and_modify(|vec| vec.push(value))
            .or_insert(vec![value]);
    }

    input[updates_pos..]
        .split(|&b| b == b'\n')
        .map(|line| {
            let mut pages = line
                .split(|&b| b == b',')
                .map(|bytes| u16::from_le_bytes([bytes[0], bytes[1]]))
                .enumerate()
                .collect::<Vec<(usize, u16)>>();

            let mut was_incorrect = false;
            pages.sort_unstable_by(|(i, a), (j, b)| {
                if let Some(rules) = map.get(b) {
                    if rules.contains(a) && j < i {
                        was_incorrect = true;
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                } else {
                    Ordering::Equal
                }
            });

            let (_, middle) = pages[pages.len() / 2];
            was_incorrect as u32 * atoi::<u32>(&middle.to_le_bytes()).expect("Bytes are valid")
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input_file = "day05.txt";
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
        let input = b"47|53\n\
                      97|13\n\
                      97|61\n\
                      97|47\n\
                      75|29\n\
                      61|13\n\
                      75|53\n\
                      29|13\n\
                      97|29\n\
                      53|29\n\
                      61|53\n\
                      97|53\n\
                      61|29\n\
                      47|13\n\
                      75|47\n\
                      97|75\n\
                      47|61\n\
                      75|61\n\
                      47|29\n\
                      75|13\n\
                      53|13\n\n\
                      75,47,61,53,29\n\
                      97,61,53,29,13\n\
                      75,29,13\n\
                      75,97,47,61,53\n\
                      61,13,29\n\
                      97,13,75,29,47";
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn test_part2() {
        let input = b"47|53\n\
                      97|13\n\
                      97|61\n\
                      97|47\n\
                      75|29\n\
                      61|13\n\
                      75|53\n\
                      29|13\n\
                      97|29\n\
                      53|29\n\
                      61|53\n\
                      97|53\n\
                      61|29\n\
                      47|13\n\
                      75|47\n\
                      97|75\n\
                      47|61\n\
                      75|61\n\
                      47|29\n\
                      75|13\n\
                      53|13\n\n\
                      75,47,61,53,29\n\
                      97,61,53,29,13\n\
                      75,29,13\n\
                      75,97,47,61,53\n\
                      61,13,29\n\
                      97,13,75,29,47";
        assert_eq!(part2(input), 123);
    }
}
