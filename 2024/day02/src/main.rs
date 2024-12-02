use std::{
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;

fn check_levels(mut levels: impl Iterator<Item = i32>) -> bool {
    let first = levels.next().unwrap();
    let second = levels.next().unwrap();

    let diff = first - second;
    let increasing = diff.is_negative();
    let diff = diff.abs();
    if !(1..=3).contains(&diff) {
        return false;
    }

    let mut prev = second;
    for level in levels {
        let diff = prev - level;
        if diff.is_negative() != increasing {
            return false;
        }
        let diff = diff.abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
        prev = level
    }

    true
}

fn part1(input: &[u8]) -> u32 {
    let mut safe_count: u32 = 0;
    for line in input.split(|&b| b == b'\n') {
        let levels = line.split(|&b| b == b' ').map(|n| atoi::<i32>(n).unwrap());
        let safe = check_levels(levels);
        safe_count += safe as u32;
    }

    safe_count
}

fn part2(input: &[u8]) -> u32 {
    let mut safe_count: u32 = 0;
    'outer: for line in input.split(|&b| b == b'\n') {
        let levels = line.split(|&b| b == b' ').map(|n| atoi::<i32>(n).unwrap());

        let safe = check_levels(levels.clone());

        if safe {
            safe_count += 1;
            continue 'outer;
        }

        for i in 0..levels.clone().count() {
            let iter = levels.clone().enumerate().filter_map(
                |(idx, x)| {
                    if idx == i {
                        None
                    } else {
                        Some(x)
                    }
                },
            );
            let safe = check_levels(iter);
            if safe {
                safe_count += 1;
                continue 'outer;
            }
        }
    }

    safe_count
}

fn main() -> io::Result<()> {
    let input_file = "day02.txt";
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
        let input = b"7 6 4 2 1\n\
                      1 2 7 8 9\n\
                      9 7 6 2 1\n\
                      1 3 2 4 5\n\
                      8 6 4 4 1\n\
                      1 3 6 7 9";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part2() {
        let input = b"7 6 4 2 1\n\
                      1 2 7 8 9\n\
                      9 7 6 2 1\n\
                      1 3 2 4 5\n\
                      8 6 4 4 1\n\
                      1 3 6 7 9";
        assert_eq!(part2(input), 4);
    }
}
