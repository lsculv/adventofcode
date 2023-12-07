#![feature(slice_split_once)]

use atoi::atoi;

fn main() {
    let input = include_bytes!("../day06.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let (times, distances) = input.split_once(|&b| b == b'\n').unwrap();
    let times: Vec<usize> = times
        .split(|&b| b == b' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| atoi(s).unwrap())
        .collect();

    let distances: Vec<usize> = distances
        .split(|&b| b == b' ')
        .skip(1)
        .filter(|s| !s.is_empty())
        .map(|s| atoi(s).unwrap())
        .collect();

    product_of_ways(&times, &distances)
}

fn part_2(input: &[u8]) -> usize {
    let (time, distance) = input.split_once(|&b| b == b'\n').unwrap();

    let time: usize = atoi(
        &time
            .split_once(|&b| b == b':')
            .unwrap()
            .1
            .iter()
            .filter(|&&b| b != b' ')
            .copied()
            .collect::<Vec<u8>>(),
    )
    .unwrap();

    let distance: usize = atoi(
        &distance
            .split_once(|&b| b == b':')
            .unwrap()
            .1
            .iter()
            .filter(|&&b| b != b' ')
            .copied()
            .collect::<Vec<u8>>(),
    )
    .unwrap();

    product_of_ways(&[time], &[distance])
}

fn product_of_ways(times: &[usize], distances: &[usize]) -> usize {
    times
        .iter()
        .zip(distances.iter())
        .map(|(&time, &dist)| {
            let time = time as f64;
            let dist = dist as f64;

            let mut upper = ((time + f64::sqrt(time.powi(2) - 4.0 * dist)) / 2.0).floor() as usize;
            let mut lower = ((time - f64::sqrt(time.powi(2) - 4.0 * dist)) / 2.0).ceil() as usize;

            if (time as usize - upper) * upper == dist as usize {
                upper -= 1;
            }

            if (time as usize - lower) * lower == dist as usize {
                lower += 1;
            }

            upper - lower + 1
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = b"Time:      7  15   30\n\
                           Distance:  9  40  200";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 288);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT), 71503);
    }
}
