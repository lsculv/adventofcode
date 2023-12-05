use atoi::atoi;

fn main() {
    let input = include_bytes!("../day03.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let mut sum = 0;
    let mut i = 0;
    loop {
        while i < input.len() && !input[i].is_ascii_digit() {
            i += 1;
        }

        if i >= input.len() {
            break;
        }

        let width = peek_digits(&input[i..]);
        if check_adjacent(input, i, width) {
            sum += atoi::<usize>(&input[i..(i + width)]).unwrap();
        }

        i += width;
    }

    sum
}

#[inline(always)]
fn is_symbol(byte: u8) -> bool {
    matches!(
        byte,
        b'#' | b'%' | b'&' | b'*' | b'+' | b'-' | b'/' | b'=' | b'@' | b'$'
    )
}

fn peek_digits(slice: &[u8]) -> usize {
    let mut width = 1;
    while width < slice.len() && slice[width].is_ascii_digit() {
        width += 1;
    }

    width
}

fn check_adjacent(input: &[u8], pos: usize, width: usize) -> bool {
    let line_len = input.iter().position(|&b| b == b'\n').unwrap();
    // All the dots are considered adjacent to the number 123
    //.....
    //.123.
    //.....

    // Right
    let at_end = pos + width >= input.len();
    if !at_end && is_symbol(input[pos + width]) {
        return true;
    }

    // Left
    let at_start = pos == 0;
    if !at_start && is_symbol(input[pos - 1]) {
        return true;
    }

    // Top
    if pos > line_len {
        for &byte in input
            .iter()
            .take(pos - line_len + width)
            .skip(pos - line_len - 2)
        {
            if is_symbol(byte) {
                return true;
            }
        }
    }

    // Bottom
    if pos < (input.len() - line_len) {
        for &byte in input
            .iter()
            .take(pos + line_len + 2 + width)
            .skip(pos + line_len)
        {
            if is_symbol(byte) {
                return true;
            }
        }
    }

    false
}

fn part_2(input: &[u8]) -> usize {
    let mut sum = 0;
    let mut i = 0;
    loop {
        while i < input.len() && input[i] != b'*' {
            i += 1;
        }

        if i >= input.len() {
            break;
        }

        if let Some(nums) = find_nums(input, i) {
            sum += nums.0 * nums.1;
        }

        i += 1;
    }

    sum
}

fn find_nums(input: &[u8], pos: usize) -> Option<(usize, usize)> {
    let line_len = input.iter().position(|&b| b == b'\n').unwrap();
    let mut adjacent_nums = 0;
    let mut num_indexes = [0; 2];

    // Right
    if input[pos + 1].is_ascii_digit() {
        num_indexes[adjacent_nums] = pos + 1;
        adjacent_nums += 1;
    }

    // Left
    if input[pos - 1].is_ascii_digit() {
        num_indexes[adjacent_nums] = pos - 1;
        adjacent_nums += 1;
    }

    // Top
    if pos > line_len {
        for i in (pos - line_len - 2)..(pos - line_len + 1) {
            if input[i].is_ascii_digit() {
                if adjacent_nums == 2 {
                    return None;
                }
                num_indexes[adjacent_nums] = i;
                adjacent_nums += 1;
                if i == pos - line_len - 2
                    && !input[i + 1].is_ascii_digit()
                    && input[i + 2].is_ascii_digit()
                {
                    if adjacent_nums == 2 {
                        return None;
                    }
                    num_indexes[adjacent_nums] = i + 2;
                    adjacent_nums += 1;
                }
                break;
            }
        }
    }

    // Bottom
    if pos < (input.len() - line_len) {
        for i in (pos + line_len)..(pos + line_len + 3) {
            if input[i].is_ascii_digit() {
                if adjacent_nums == 2 {
                    return None;
                }
                num_indexes[adjacent_nums] = i;
                adjacent_nums += 1;
                if i == pos + line_len
                    && !input[i + 1].is_ascii_digit()
                    && input[i + 2].is_ascii_digit()
                {
                    if adjacent_nums == 2 {
                        return None;
                    }
                    num_indexes[adjacent_nums] = i + 2;
                    adjacent_nums += 1;
                }
                break;
            }
        }
    }

    if adjacent_nums != 2 {
        return None;
    }

    let mut nums = [0, 0];
    for (i, idx) in num_indexes.iter().enumerate() {
        let mut backtrack = 1;
        while idx.checked_sub(backtrack).is_some() && input[idx - backtrack].is_ascii_digit() {
            backtrack += 1;
        }

        let num_base = idx + 1 - backtrack;
        let width = peek_digits(&input[num_base..]);
        nums[i] = atoi(&input[num_base..(num_base + width)]).unwrap();
    }

    Some((nums[0], nums[1]))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_1(input), 4361);
    }

    #[test]
    fn test_part_2() {
        let input = b"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        assert_eq!(part_2(input), 467835);
    }
}
