use std::{
    fs::{self, File},
    io::{self, Read},
};

type Grid<'a> = Vec<&'a [u8]>;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        let x = x as isize;
        let y = y as isize;
        Self { x, y }
    }
}

fn is_in_range(x_max: usize, y_max: usize, (x, y): (isize, isize)) -> bool {
    x < x_max as isize && y < y_max as isize && x >= 0 && y >= 0
}

fn part1(input: &[u8]) -> u64 {
    let grid: Grid = input.split(|&b| b == b'\n').collect();
    let x_max = grid[0].len();
    let y_max = grid.len();
    let mut antinodes = Vec::new();
    let mut antennae_locations = vec![Vec::new(); b'z' as usize];
    for (y, row) in grid.iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            let is_valid_freq =
                byte.is_ascii_lowercase() || byte.is_ascii_uppercase() || byte.is_ascii_digit();
            if is_valid_freq {
                antennae_locations[*byte as usize].push(Coord::new(x, y));
            }
        }
    }

    for locations in antennae_locations {
        for location in &locations[..] {
            for other in &locations[..] {
                if other == location {
                    continue;
                }

                let x_delta = other.x - location.x;
                let y_delta = other.y - location.y;
                let antinode1 = (other.x + x_delta, other.y + y_delta);
                let antinode2 = (location.x - x_delta, location.y - y_delta);

                if is_in_range(x_max, y_max, antinode1) {
                    antinodes.push(antinode1);
                }
                if is_in_range(x_max, y_max, antinode2) {
                    antinodes.push(antinode2);
                }
            }
        }
    }

    antinodes.sort_unstable();
    antinodes.dedup();

    // let a: String = input.iter().map(|&b| b as char).collect();
    // println!("{a}");
    // println!();
    // for (y, row) in input.split(|&b| b == b'\n').enumerate() {
    //     for (x, b) in row.iter().enumerate() {
    //         if antinodes.contains(&(x as isize, y as isize)) {
    //             print!("#");
    //         } else {
    //             print!("{}", *b as char);
    //         }
    //     }
    //     println!();
    // }
    // println!();

    antinodes.len() as u64
}

fn part2(input: &[u8]) -> u64 {
    let grid: Grid = input.split(|&b| b == b'\n').collect();
    let x_max = grid[0].len();
    let y_max = grid.len();
    let mut antinodes = Vec::new();
    let mut antennae_locations = vec![Vec::new(); b'z' as usize];
    for (y, row) in grid.iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            let is_valid_freq =
                byte.is_ascii_lowercase() || byte.is_ascii_uppercase() || byte.is_ascii_digit();
            if is_valid_freq {
                antennae_locations[*byte as usize].push(Coord::new(x, y));
            }
        }
    }

    for locations in antennae_locations {
        for location in &locations[..] {
            for other in &locations[..] {
                if other == location {
                    continue;
                }

                let x_delta = other.x - location.x;
                let y_delta = other.y - location.y;

                for direction in [-1, 1] {
                    let mut x_test = location.x;
                    let mut y_test = location.y;
                    x_test += direction * x_delta;
                    y_test += direction * y_delta;
                    while is_in_range(x_max, y_max, (x_test, y_test)) {
                        antinodes.push((x_test, y_test));
                        x_test += direction * x_delta;
                        y_test += direction * y_delta;
                    }
                }
            }
        }
    }

    antinodes.sort_unstable();
    antinodes.dedup();

    antinodes.len() as u64
}

fn main() -> io::Result<()> {
    let input_file = "day08.txt";
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
        let input = b"............\n\
                      ........0...\n\
                      .....0......\n\
                      .......0....\n\
                      ....0.......\n\
                      ......A.....\n\
                      ............\n\
                      ............\n\
                      ........A...\n\
                      .........A..\n\
                      ............\n\
                      ............";
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn test_part2() {
        let input = b"............\n\
                      ........0...\n\
                      .....0......\n\
                      .......0....\n\
                      ....0.......\n\
                      ......A.....\n\
                      ............\n\
                      ............\n\
                      ........A...\n\
                      .........A..\n\
                      ............\n\
                      ............";
        assert_eq!(part2(input), 34);
    }
}
