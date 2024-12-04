use std::{
    fs::{self, File},
    io::{self, Read},
};

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Coord { x, y }
    }

    fn transform(&self, x_delta: isize, y_delta: isize) -> Self {
        let x = (self.x as isize + x_delta) as usize;
        let y = (self.y as isize + y_delta) as usize;
        Coord { x, y }
    }

    fn index(&self, grid: &Vec<&[u8]>) -> Option<u8> {
        grid.get(self.y)?.get(self.x).copied()
    }

    fn apply(&self, direction: Direction) -> Coord {
        match direction {
            Direction::North => self.transform(0, 1),
            Direction::NorthEast => self.transform(1, 1),
            Direction::NorthWest => self.transform(-1, 1),
            Direction::South => self.transform(0, -1),
            Direction::SouthEast => self.transform(1, -1),
            Direction::SouthWest => self.transform(-1, -1),
            Direction::East => self.transform(1, 0),
            Direction::West => self.transform(-1, 0),
        }
    }

    fn check(&self, grid: &Vec<&[u8]>, direction: Direction, expected: u8) -> bool {
        match self.apply(direction).index(grid) {
            Some(x) => x == expected,
            None => false,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    East,
    West,
}

impl Direction {
    fn directions() -> [Direction; 8] {
        use Direction::*;
        [
            North, NorthEast, NorthWest, South, SouthEast, SouthWest, East, West,
        ]
    }
}

fn part1(input: &[u8]) -> u32 {
    let grid: Vec<&[u8]> = input.split(|&b| b == b'\n').collect();
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, &b)| match b {
                b'X' => Some(Coord::new(x, y)),
                _ => None,
            })
        })
        .map(|coord| {
            let mut xmas_count = 0;
            'outer: for direction in Direction::directions() {
                let mut marker = coord;
                for &letter in b"MAS" {
                    marker = marker.apply(direction);
                    if let Some(l) = marker.index(&grid) {
                        if l != letter {
                            continue 'outer;
                        }
                    } else {
                        continue 'outer;
                    }
                }
                xmas_count += 1;
            }
            xmas_count
        })
        .sum()
}

fn part2(input: &[u8]) -> u32 {
    let grid: Vec<&[u8]> = input.split(|&b| b == b'\n').collect();
    grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter().enumerate().filter_map(move |(x, &b)| match b {
                b'A' => Some(Coord::new(x, y)),
                _ => None,
            })
        })
        .map(|coord| {
            use Direction::*;
            let has_top_left = (coord.check(&grid, NorthWest, b'S')
                && coord.check(&grid, SouthEast, b'M'))
                || (coord.check(&grid, NorthWest, b'M') && coord.check(&grid, SouthEast, b'S'));
            let has_top_right = (coord.check(&grid, NorthEast, b'S')
                && coord.check(&grid, SouthWest, b'M'))
                || (coord.check(&grid, NorthEast, b'M') && coord.check(&grid, SouthWest, b'S'));
            let is_x = has_top_left && has_top_right;
            is_x as u32
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input_file = "day04.txt";
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
        let input = b"MMMSXXMASM\n\
                      MSAMXMSMSA\n\
                      AMXSXMAAMM\n\
                      MSAMASMSMX\n\
                      XMASAMXAMM\n\
                      XXAMMXXAMA\n\
                      SMSMSASXSS\n\
                      SAXAMASAAA\n\
                      MAMMMXMMMM\n\
                      MXMXAXMASX";
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_part2() {
        let input = b"MMMSXXMASM\n\
                      MSAMXMSMSA\n\
                      AMXSXMAAMM\n\
                      MSAMASMSMX\n\
                      XMASAMXAMM\n\
                      XXAMMXXAMA\n\
                      SMSMSASXSS\n\
                      SAXAMASAAA\n\
                      MAMMMXMMMM\n\
                      MXMXAXMASX";
        assert_eq!(part2(input), 9);
    }
}
