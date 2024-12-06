use std::{
    fs::{self, File},
    io::{self, Read},
};

type Grid<'a> = Vec<Vec<u8>>;

fn parse_grid(input: &[u8]) -> Grid {
    let mut input = input.to_vec();
    input
        .split_mut(|&b| b == b'\n')
        .map(|slice| slice.to_vec())
        .collect()
}

fn index_grid<'a>(grid: &'a Grid, x: usize, y: usize) -> Option<&'a u8> {
    grid.get(y)?.get(x)
}

fn index_grid_mut<'a>(grid: &'a mut Grid, x: usize, y: usize) -> Option<&'a mut u8> {
    grid.get_mut(y)?.get_mut(x)
}

#[derive(Clone, Debug)]
struct Guard {
    x: usize,
    y: usize,
    facing: Facing,
}

impl Guard {
    fn from_grid(grid: &Grid) -> Self {
        let (facing, x, y) = grid
            .iter()
            .enumerate()
            .filter_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .find(|(_, b)| b"^>v<".contains(b))
                    .map(|(x, &f)| (f.into(), x, y))
            })
            .next()
            .unwrap();
        Self { x, y, facing }
    }

    fn travel(&mut self, grid: &mut Grid) -> Option<usize> {
        *index_grid_mut(grid, self.x, self.y).unwrap() = b'X';
        let mut visited = 1;
        let mut no_change = 0;
        loop {
            if no_change == visited {
                return None;
            }
            let (x_delta, y_delta) = self.facing.delta();

            let new_x = (self.x as isize + x_delta) as usize;
            let new_y = (self.y as isize + y_delta) as usize;
            let Some(tile) = index_grid(grid, new_x, new_y) else {
                break;
            };

            match tile {
                b'.' => {
                    *index_grid_mut(grid, new_x, new_y).unwrap() = b'X';
                    visited += 1;
                    no_change = 0;
                    self.x = new_x;
                    self.y = new_y;
                }
                b'X' => {
                    no_change += 1;
                    self.x = new_x;
                    self.y = new_y;
                }
                b'#' => {
                    self.facing = self.facing.turn();
                }
                _ => unreachable!(),
            };
        }

        Some(visited)
    }
}

#[derive(Copy, Clone, Debug)]
enum Facing {
    Up,
    Right,
    Left,
    Down,
}

impl Facing {
    fn delta(&self) -> (isize, isize) {
        match self {
            Facing::Up => (0, -1),
            Facing::Right => (1, 0),
            Facing::Left => (-1, 0),
            Facing::Down => (0, 1),
        }
    }

    fn turn(&self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Left => Facing::Up,
            Facing::Down => Facing::Left,
        }
    }
}

impl From<u8> for Facing {
    fn from(value: u8) -> Self {
        match value {
            b'^' => Self::Up,
            b'>' => Self::Right,
            b'v' => Self::Down,
            b'<' => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl From<Facing> for u8 {
    fn from(value: Facing) -> u8 {
        match value {
            Facing::Up => b'^',
            Facing::Right => b'>',
            Facing::Left => b'<',
            Facing::Down => b'v',
        }
    }
}

fn part1(input: &[u8]) -> usize {
    let mut grid: Grid = parse_grid(input);
    let mut guard = Guard::from_grid(&grid);
    guard.travel(&mut grid).unwrap()
}

fn part2(input: &[u8]) -> u32 {
    let grid: Grid = parse_grid(input);
    let x_max = grid.first().unwrap().len();
    let y_max = grid.len();

    let guard = Guard::from_grid(&grid);
    let mut positions = 0;
    for x in 0..x_max {
        for y in 0..y_max {
            let mut guard_copy = guard.clone();
            let mut grid_copy = grid.clone();
            *index_grid_mut(&mut grid_copy, x, y).unwrap() = b'#';

            if guard_copy.travel(&mut grid_copy).is_none() {
                positions += 1;
            }
        }
    }

    positions
}

fn main() -> io::Result<()> {
    let input_file = "day06.txt";
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
        let input = b"....#.....\n\
                      .........#\n\
                      ..........\n\
                      ..#.......\n\
                      .......#..\n\
                      ..........\n\
                      .#..^.....\n\
                      ........#.\n\
                      #.........\n\
                      ......#...";
        assert_eq!(part1(input), 41);
    }

    #[test]
    fn test_part2() {
        let input = b"....#.....\n\
                      .........#\n\
                      ..........\n\
                      ..#.......\n\
                      .......#..\n\
                      ..........\n\
                      .#..^.....\n\
                      ........#.\n\
                      #.........\n\
                      ......#...";
        assert_eq!(part2(input), 6);
    }
}
