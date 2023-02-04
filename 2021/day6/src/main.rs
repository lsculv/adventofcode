use std::fs;

fn parse_input(input: &str) -> [u64; 9] {
    let mut collection: [u64; 9] = [0; 9];
    input
        .split(',')
        .map(|c| c.parse::<u64>().unwrap())
        .for_each(|num| collection[num as usize] += 1);
    return collection;
}

fn part_1(fish: &mut [u64; 9]) -> u64 {
    for _ in 0..80 {
        let born_fish = fish[0];
        fish.rotate_left(1);
        fish[6] += born_fish;
    }
    fish.into_iter().map(|num| *num).sum()
}

fn part_2(fish: &mut [u64; 9]) -> u64 {
    for _ in 0..256 {
        let born_fish = fish[0];
        fish.rotate_left(1);
        fish[6] += born_fish;
    }
    fish.into_iter().map(|num| *num).sum()
}

fn main() {
    let path = "../inputs/day6.txt";
    let input = fs::read_to_string(path).unwrap();
    let fish = parse_input(&input);
    let fish_count_part_1 = part_1(&mut fish.clone());
    let fish_count_part_2 = part_2(&mut fish.clone());
    println!("Part 1:\n{}", fish_count_part_1);
    println!("Part 2:\n{}", fish_count_part_2);
}
