use std::{
    fs::{self, File},
    io::{self, Read},
};

use atoi::atoi;

fn part1(input: &[u8]) -> u32 {
    let mut sum = 0;
    let mut iter = input.iter();
    while let Some(byte) = iter.next() {
        match byte {
            b'm' if matches_rest(&mut iter, b"ul(") => {
                if let Some(val) = parse_operands(&mut iter) {
                    sum += val;
                }
            }
            _ => continue,
        }
    }

    sum
}

fn part2(input: &[u8]) -> u32 {
    let mut sum = 0;
    let mut iter = input.iter();
    let mut enable = true;
    while let Some(byte) = iter.next() {
        match byte {
            b'm' if enable && matches_rest(&mut iter, b"ul(") => {
                if let Some(val) = parse_operands(&mut iter) {
                    sum += val;
                }
            }
            b'd' => match iter.next() {
                Some(b'o') => match iter.next() {
                    Some(b'(') if matches_rest(&mut iter, b")") => enable = true,
                    Some(b'n') if matches_rest(&mut iter, b"'t()") => enable = false,
                    _ => continue,
                },
                _ => continue,
            },
            _ => continue,
        }
    }

    sum
}

fn parse_operands<'a>(iter: &mut impl Iterator<Item = &'a u8>) -> Option<u32> {
    let mut l_arg = Vec::with_capacity(16);
    while let Some(&x) = iter.next() {
        if x.is_ascii_digit() {
            l_arg.push(x);
        } else if x == b',' {
            let mut r_arg = Vec::with_capacity(16);
            while let Some(&y) = iter.next() {
                if y.is_ascii_digit() {
                    r_arg.push(y);
                } else if y == b')' && !r_arg.is_empty() {
                    return Some(atoi::<u32>(&l_arg).unwrap() * atoi::<u32>(&r_arg).unwrap());
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }
    }

    None
}

fn matches_rest<'a>(iter: &mut impl Iterator<Item = &'a u8>, rest: &[u8]) -> bool {
    for byte in rest {
        let Some(next) = iter.next() else {
            return false;
        };
        if next != byte {
            return false;
        }
    }

    true
}

fn main() -> io::Result<()> {
    let input_file = "day03.txt";
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
        let input = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn test_part2() {
        let input = b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 48);
    }
}
