fn main() {
    let input: &[u8] = include_bytes!("../day01.txt");
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

fn part_1(input: &[u8]) -> u32 {
    let mut sum = 0;
    for line in input.split(|&b| b == b'\n') {
        let tens = line
            .iter()
            .find(|b| b.is_ascii_digit())
            .expect("line should always contain at least one digit")
            - b'0';
        let ones = line
            .iter()
            .rev()
            .find(|b| b.is_ascii_digit())
            .expect("line should always contain at least one digit")
            - b'0';
        sum += (tens * 10 + ones) as u32;
    }

    sum
}

fn part_2(input: &[u8]) -> u32 {
    let mut sum = 0;
    for line in input.split(|&n| n == b'\n') {
        let tens = find_digit_left(line);
        let ones = find_digit_right(line);
        sum += tens * 10 + ones;
    }

    sum
}

fn find_digit_right(line: &[u8]) -> u32 {
    for i in (0..line.len()).rev() {
        if line[i].is_ascii_digit() {
            return (line[i] - b'0') as u32;
        }
        match line[i] {
            b'e' => match line[i - 1] {
                b'n' => match line[i - 2] {
                    b'o' => return 1,
                    b'i' if line[i - 3] == b'n' => return 9,
                    _ => continue,
                },
                b'v' if checkr(line, i - 2, b"fi") => return 5,
                b'e' if checkr(line, i - 2, b"thr") => return 3,
                _ => continue,
            },
            b'o' if checkr(line, i - 1, b"tw") => return 2,
            b'r' if checkr(line, i - 1, b"fou") => return 4,
            b'x' if checkr(line, i - 1, b"si") => return 6,
            b'n' if checkr(line, i - 1, b"seve") => return 7,
            b't' if checkr(line, i - 1, b"eigh") => return 8,
            _ => continue,
        }
    }
    unreachable!("Must reach a digit.");
}

fn find_digit_left(line: &[u8]) -> u32 {
    for i in 0..line.len() {
        if line[i].is_ascii_digit() {
            return (line[i] - b'0') as u32;
        }
        match line[i] {
            b'o' if checkl(line, i + 1, b"ne") => return 1,
            b't' => match line[i + 1] {
                b'w' if line[i + 2] == b'o' => return 2,
                b'h' if checkl(line, i + 2, b"ree") => return 3,
                _ => continue,
            },
            b'f' => match line[i + 1] {
                b'o' if checkl(line, i + 2, b"ur") => return 4,
                b'i' if checkl(line, i + 2, b"ve") => return 5,
                _ => continue,
            },
            b's' => match line[i + 1] {
                b'i' if line[i + 2] == b'x' => return 6,
                b'e' if checkl(line, i + 2, b"ven") => return 7,
                _ => continue,
            },
            b'e' if checkl(line, i + 1, b"ight") => return 8,
            b'n' if checkl(line, i + 1, b"ine") => return 9,
            _ => continue,
        }
    }
    unreachable!("Must reach a digit.");
}

fn checkl(slice: &[u8], start: usize, cmp: &[u8]) -> bool {
    for i in 0..cmp.len() {
        if slice[start + i] != cmp[i] {
            return false;
        }
    }
    true
}

fn checkr(slice: &[u8], end: usize, cmp: &[u8]) -> bool {
    for i in 0..cmp.len() {
        if slice[end - i] != cmp[cmp.len() - (i + 1)] {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_part_1() {
        let input: &[u8] = b"1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        assert_eq!(part_1(input), 142);
    }
    #[test]
    fn test_part_2() {
        let input: &[u8] = b"two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        assert_eq!(part_2(input), 281);
    }
}
