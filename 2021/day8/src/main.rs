use std::fs;

fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            line.split_at(60)
                .1
                .split(' ')
                .map(|s| match s.len() {
                    2 | 4 | 3 | 7 => 1,
                    _ => 0,
                })
                .sum::<u32>()
        })
        .sum()
}

fn part_2(input: &str) -> u32 {
    // A byte containing the letters that are lit. They are layed out as follows
    // 0 g f e d c b a
    let number: u8;
    let segments: [char; 7] = [' '; 7];
    let mut code_buf = [0u8; 10];
    for line in input.lines() {
        let (clues, nums) = line.split_once('|').unwrap();
        clues.split(' ').enumerate().for_each(|(i, code)| {
            let mut code_num = 0u8;
            // Change the ascii code of our char into the byte offset to set the
            // desired bit in the byte code representation
            for c in code.chars() {
                code_num |= 1 << (c as u8 - 97);
                code_buf[i] = code_num;
            }
        });
        
    }
    32
}

fn count_set_bits(mut n: u8) -> u8 {
    let mut count = 0u8;
    for _ in 0..8 {
        count += n & 1;
        n >>= 1;
    }
    
    count
}

fn main() {
    let input = fs::read_to_string("../inputs/day8.txt").expect("Could not open input file");
    println!("Part 1:\t{}", part_1(&input));
}
