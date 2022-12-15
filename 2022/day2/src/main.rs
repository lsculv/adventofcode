use std::fs;

fn parse_guide_part_one(data: &str) -> u32 {
    let mut score = 0;
    // according to each rock, paper, scissors combination, add the associated score
    data.split("\n").for_each(|line| match line {
        "A X" => score += 4,
        "A Y" => score += 8,
        "A Z" => score += 3,
        "B X" => score += 1,
        "B Y" => score += 5,
        "B Z" => score += 9,
        "C X" => score += 7,
        "C Y" => score += 2,
        "C Z" => score += 6,
        // fail quietly
        &_ => score += 0,
    });
    return score;
}

fn parse_guide_part_two(data: &str) -> u32 {
    let mut score = 0;
    // according to each opponent move and desired outcome, add the associated score
    data.split("\n").for_each(|line| match line {
        "A X" => score += 3,
        "A Y" => score += 4,
        "A Z" => score += 8,
        "B X" => score += 1,
        "B Y" => score += 5,
        "B Z" => score += 9,
        "C X" => score += 2,
        "C Y" => score += 6,
        "C Z" => score += 7,
        // fail quietly
        &_ => score += 0,
    });
    return score;
}

fn main() {
    // read data from input file
    let path = "../inputs/day2.txt";
    let guide_input = fs::read_to_string(path).expect("File Read Error");

    let score_part_one = parse_guide_part_one(&guide_input);
    let score_part_two = parse_guide_part_two(&guide_input);
    println!("Part 1:\n{}", score_part_one);
    println!("Part 2:\n{}", score_part_two);
}
