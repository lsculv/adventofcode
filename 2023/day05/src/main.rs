#![feature(iter_array_chunks)]

fn main() {
    let input = include_str!("../day05.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut seeds: Vec<usize> = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|n| n.parse().unwrap())
        .collect();

    find_min_seed_location(&mut seeds, maps)
}

fn part_2(input: &str) -> usize {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seed_ranges: Vec<[usize; 2]> = seeds
        .split_once(": ")
        .unwrap()
        .1
        .split(' ')
        .map(|n| n.parse().unwrap())
        .array_chunks()
        .collect();

    let mut seeds = Vec::with_capacity(370_114_248);
    let mut mins = Vec::with_capacity(10);
    for [start, count] in seed_ranges {
        for i in start..(start + count) {
            seeds.push(i);
        }
        mins.push(find_min_seed_location(&mut seeds, maps));
        seeds.clear();
    }

    *mins.iter().min().unwrap()
}

fn find_min_seed_location(seeds: &mut [usize], maps: &str) -> usize {
    for map in maps.split("\n\n") {
        let ranges: Vec<[usize; 3]> = map
            .split_whitespace()
            .skip(2)
            .map(|n| n.parse::<usize>().unwrap())
            .array_chunks()
            .collect();

        'seeds: for seed in seeds.iter_mut() {
            for [dest, source, count] in ranges.iter() {
                if *seed >= *source && *seed < source + count {
                    *seed = dest + (*seed - source);
                    continue 'seeds;
                }
            }
        }
    }

    *seeds.iter().min().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 35);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 46);
    }
}
