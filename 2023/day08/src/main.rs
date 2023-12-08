#![feature(slice_split_once, byte_slice_trim_ascii)]
use std::collections::HashMap;

fn main() {
    let input = include_bytes!("../day08.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let (node_map, steps) = parse_input(input);

    let mut node: &[u8] = b"AAA";
    let mut counter = 0;
    for direction in steps.iter().cycle() {
        counter += 1;
        node = match direction {
            b'L' => node_map.get(node).unwrap().0,
            b'R' => node_map.get(node).unwrap().1,
            _ => unreachable!(),
        };
        if node == b"ZZZ" {
            break;
        }
    }
    counter
}

fn part_2(input: &[u8]) -> usize {
    let (node_map, steps) = parse_input(input);

    let mut nodes = Vec::new();
    for &node in node_map.keys() {
        if node[2] == b'A' {
            nodes.push(node);
        }
    }

    let mut passed = vec![0usize; nodes.len()];
    let mut cycle_lengths = vec![0usize; nodes.len()];
    let mut count = 0;
    'outer: for direction in steps.iter().cycle() {
        count += 1;
        for (i, node) in nodes.iter_mut().enumerate() {
            *node = match direction {
                b'L' => node_map.get(node).unwrap().0,
                b'R' => node_map.get(node).unwrap().1,
                _ => unreachable!(),
            };

            if node[2] == b'Z' && passed[i] == 0 {
                passed[i] = count;
            } else if node[2] == b'Z' && passed[i] != 0 && cycle_lengths[i] == 0 {
                cycle_lengths[i] = count - passed[i];
            }
        }

        for &len in cycle_lengths.iter() {
            if len == 0 {
                continue 'outer;
            }
        }

        break 'outer;
    }

    let mut lcm = num::integer::lcm(cycle_lengths[0], cycle_lengths[1]);
    for len in &cycle_lengths[2..] {
        lcm = num::integer::lcm(lcm, *len);
    }

    lcm
}

type ParsedInput<'a> = (HashMap<&'a [u8], (&'a [u8], &'a [u8])>, &'a [u8]);
fn parse_input(input: &[u8]) -> ParsedInput {
    let (steps, nodes) = input.split_once(|&b| b == b'\n').unwrap();
    let nodes = nodes.trim_ascii();
    let mut node_map: HashMap<&[u8], (&[u8], &[u8])> =
        HashMap::with_capacity(nodes.split(|&b| b == b'\n').count());

    for node in nodes.split(|&b| b == b'\n') {
        let start = &node[0..3];
        let left = &node[7..10];
        let right = &node[12..15];
        node_map.insert(start, (left, right));
    }

    (node_map, steps)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT_1: &[u8] = b"RL\n\
                           \n\
                           AAA = (BBB, CCC)\n\
                           BBB = (DDD, EEE)\n\
                           CCC = (ZZZ, GGG)\n\
                           DDD = (DDD, DDD)\n\
                           EEE = (EEE, EEE)\n\
                           GGG = (GGG, GGG)\n\
                           ZZZ = (ZZZ, ZZZ)";

    const INPUT_2: &[u8] = b"LLR\n\
                             \n\
                             AAA = (BBB, BBB)\n\
                             BBB = (AAA, ZZZ)\n\
                             ZZZ = (ZZZ, ZZZ)";

    const INPUT_3: &[u8] = b"LR\n\
                             \n\
                             11A = (11B, XXX)\n\
                             11B = (XXX, 11Z)\n\
                             11Z = (11B, XXX)\n\
                             22A = (22B, XXX)\n\
                             22B = (22C, 22C)\n\
                             22C = (22Z, 22Z)\n\
                             22Z = (22B, 22B)\n\
                             XXX = (XXX, XXX)";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT_1), 2);
        assert_eq!(part_1(INPUT_2), 6);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT_3), 6);
    }
}
