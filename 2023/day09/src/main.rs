#![feature(iter_collect_into)]

use atoi::atoi;

fn main() {
    let input = include_bytes!("../day09.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> i64 {
    let mut sum = 0;
    let mut numbers: Vec<i64> = Vec::with_capacity(32);
    let mut aux: Vec<i64> = Vec::with_capacity(32);
    let mut history: Vec<Vec<i64>> = Vec::new();
    for line in input.split(|&b| b == b'\n') {
        write_history(line, &mut numbers, &mut aux, &mut history);

        for i in (1..history.len()).rev() {
            let len = history[i].len();
            let next_len = history[i - 1].len();
            let diff = history[i][len - 1];
            let prev = history[i - 1][next_len - 1];
            history[i - 1].push(prev + diff);
        }

        sum += history.first().unwrap().last().unwrap();

        history.clear();
        numbers.clear();
        aux.clear();
    }

    sum
}

fn part_2(input: &[u8]) -> i64 {
    let mut sum = 0;
    let mut numbers: Vec<i64> = Vec::with_capacity(32);
    let mut aux: Vec<i64> = Vec::with_capacity(32);
    let mut history: Vec<Vec<i64>> = Vec::new();
    for line in input.split(|&b| b == b'\n') {
        write_history(line, &mut numbers, &mut aux, &mut history);

        for i in (1..history.len()).rev() {
            let diff = history[i][0];
            let prev = history[i - 1][0];
            history[i - 1].insert(0, prev - diff);
        }

        sum += history.first().unwrap().first().unwrap();

        history.clear();
        numbers.clear();
        aux.clear();
    }

    sum
}

fn write_history(
    line: &[u8],
    numbers: &mut Vec<i64>,
    aux: &mut Vec<i64>,
    history: &mut Vec<Vec<i64>>,
) {
    line.split(|&b| b == b' ')
        .map(|s| atoi::<i64>(s).unwrap())
        .collect_into(numbers);

    while !all_zeroes(numbers) {
        history.push(numbers.clone());
        for i in 0..(numbers.len() - 1) {
            aux.push(numbers[i + 1] - numbers[i]);
        }
        numbers.clear();
        numbers.append(aux);
    }
}

fn all_zeroes(numbers: &[i64]) -> bool {
    for &n in numbers {
        if n != 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = b"0 3 6 9 12 15\n\
                           1 3 6 10 15 21\n\
                           10 13 16 21 30 45";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 114);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 2);
    }
}
