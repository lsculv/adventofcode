use std::cell::OnceCell;

fn main() {
    let input = include_bytes!("../day10.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

#[derive(Debug, Clone, Copy)]
enum Pipe {
    /// `|`
    Vertical,
    /// `-`
    Horizontal,
    /// `L`
    NorthEast,
    /// `J`
    NorthWest,
    /// `7`
    SouthWest,
    /// `F`
    SouthEast,
    /// `.`
    Empty,
    /// `S`
    Start,
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Pipe::Vertical,
            b'-' => Pipe::Horizontal,
            b'L' => Pipe::NorthEast,
            b'J' => Pipe::NorthWest,
            b'7' => Pipe::SouthWest,
            b'F' => Pipe::SouthEast,
            b'S' => Pipe::Start,
            _ => Pipe::Empty,
        }
    }
}

impl From<[Direction; 2]> for Pipe {
    fn from(value: [Direction; 2]) -> Self {
        type D = Direction;
        let value = (value[0], value[1]);
        match value {
            (D::Up, D::Right) | (D::Right, D::Up) => Pipe::NorthEast, //L
            (D::Up, D::Down) | (D::Down, D::Up) => Pipe::Vertical,
            (D::Up, D::Left) | (D::Left, D::Up) => Pipe::NorthWest,
            (D::Right, D::Down) | (D::Down, D::Right) => Pipe::SouthEast,
            (D::Right, D::Left) | (D::Left, D::Right) => Pipe::Horizontal,
            (D::Down, D::Left) | (D::Left, D::Down) => Pipe::SouthWest,
            _ => panic!("Invalid combination"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl TryFrom<(Pipe, Direction)> for Direction {
    type Error = ();

    fn try_from(value: (Pipe, Direction)) -> Result<Self, Self::Error> {
        let pipe = value.0;
        let dir = value.1;
        match (pipe, dir) {
            (Pipe::Vertical, _) => Ok(dir),

            (Pipe::Horizontal, _) => Ok(dir),

            (Pipe::NorthEast, Direction::Down) => Ok(Direction::Right),
            (Pipe::NorthEast, Direction::Left) => Ok(Direction::Up),

            (Pipe::NorthWest, Direction::Down) => Ok(Direction::Left),
            (Pipe::NorthWest, Direction::Right) => Ok(Direction::Up),

            (Pipe::SouthWest, Direction::Up) => Ok(Direction::Left),
            (Pipe::SouthWest, Direction::Right) => Ok(Direction::Down),

            (Pipe::SouthEast, Direction::Up) => Ok(Direction::Right),
            (Pipe::SouthEast, Direction::Left) => Ok(Direction::Down),

            (Pipe::Start, _) => Ok(dir),
            _ => Err(()),
        }
    }
}

type Coord = (usize, usize);
type MarkedPipe = (Pipe, bool);

fn part_1(input: &[u8]) -> usize {
    let mut sp = (0, 0);
    let mut pipes: Vec<Vec<Pipe>> = input
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &b)| {
                    if b == b'S' {
                        sp = (i, j);
                    }
                    Pipe::from(b)
                })
                .collect()
        })
        .collect();

    let entrance = find_entrance(&mut pipes, sp);

    // Traverse the loop
    let len = traverse(&pipes, sp, entrance);

    len / 2
}

fn part_2(input: &[u8]) -> usize {
    let mut sp = (0, 0);
    let mut pipes: Vec<Vec<Pipe>> = input
        .split(|&b| b == b'\n')
        .enumerate()
        .map(|(i, line)| {
            line.iter()
                .enumerate()
                .map(|(j, &b)| {
                    if b == b'S' {
                        sp = (i, j);
                    }
                    Pipe::from(b)
                })
                .collect()
        })
        .collect();

    let entrance = find_entrance(&mut pipes, sp);
    let mut pipes: Vec<Vec<MarkedPipe>> = pipes
        .into_iter()
        .map(|line| line.into_iter().map(|p| (p, false)).collect())
        .collect();
    pipes[sp.0][sp.1].1 = true;
    traverse_and_mark(&mut pipes, sp, entrance);

    count_inside(&pipes)
}

fn count_inside(pipes: &Vec<Vec<MarkedPipe>>) -> usize {
    let mut count = 0;
    let mut inside = false;

    for line in pipes {
        'inner: for pipe in line {
            if pipe.1 && matches!(pipe.0, Pipe::Vertical | Pipe::NorthEast | Pipe::NorthWest) {
                inside = !inside;
                continue 'inner;
            }

            if inside && !pipe.1 {
                count += 1;
            }
        }
    }
    count
}

fn find_entrance(pipes: &mut [Vec<Pipe>], sp: Coord) -> ((usize, usize), Direction) {
    let entrance = OnceCell::new();
    let mut entrance_dirs = [Direction::Up, Direction::Up];
    let mut found = 0;
    // Up
    let pipe = pipes[sp.0 - 1][sp.1];
    if matches!(pipe, Pipe::Vertical | Pipe::SouthWest | Pipe::SouthEast) {
        let _ = entrance.set(((sp.0 - 1, sp.1), Direction::Up));
        entrance_dirs[found] = Direction::Up;
        found += 1;
    }
    // Right
    let pipe = pipes[sp.0][sp.1 + 1];
    if matches!(pipe, Pipe::Horizontal | Pipe::NorthWest | Pipe::SouthWest) {
        let _ = entrance.set(((sp.0, sp.1 + 1), Direction::Right));
        entrance_dirs[found] = Direction::Right;
        found += 1;
    }

    // Down
    let pipe = pipes[sp.0 + 1][sp.1];
    if matches!(pipe, Pipe::Vertical | Pipe::NorthWest | Pipe::NorthEast) {
        let _ = entrance.set(((sp.0 + 1, sp.1), Direction::Down));
        entrance_dirs[found] = Direction::Down;
        found += 1;
    }
    // Left
    let pipe = pipes[sp.0][sp.1 - 1];
    if matches!(pipe, Pipe::Horizontal | Pipe::NorthEast | Pipe::SouthEast) {
        let _ = entrance.set(((sp.0, sp.1 - 1), Direction::Left));
        entrance_dirs[found] = Direction::Left;
    }

    pipes[sp.0][sp.1] = Pipe::from(entrance_dirs);

    entrance.into_inner().unwrap()
}

fn traverse(pipes: &[Vec<Pipe>], sp: Coord, entrance: (Coord, Direction)) -> usize {
    let (mut c, mut dir) = entrance;
    let mut len = 1;
    while c != sp {
        len += 1;
        dir = match Direction::try_from((pipes[c.0][c.1], dir)) {
            Ok(d) => d,
            Err(_) => {
                panic!("Tried to enter a pipe incorrectly!!!");
            }
        };

        match dir {
            Direction::Up => c.0 -= 1,
            Direction::Right => c.1 += 1,
            Direction::Down => c.0 += 1,
            Direction::Left => c.1 -= 1,
        }
    }
    len
}

fn traverse_and_mark(pipes: &mut [Vec<MarkedPipe>], sp: Coord, entrance: (Coord, Direction)) {
    let (mut c, mut dir) = entrance;
    while c != sp {
        dir = match Direction::try_from((pipes[c.0][c.1].0, dir)) {
            Ok(d) => {
                pipes[c.0][c.1].1 = true;
                d
            }
            Err(_) => {
                panic!("Tried to enter a pipe incorrectly!!!");
            }
        };

        match dir {
            Direction::Up => c.0 -= 1,
            Direction::Right => c.1 += 1,
            Direction::Down => c.0 += 1,
            Direction::Left => c.1 -= 1,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT1: &[u8] = b".....\n\
                            .S-7.\n\
                            .|.|.\n\
                            .L-J.\n\
                            .....";

    const INPUT2: &[u8] = b"...F7.\n\
                            ..FJ|.\n\
                            .SJ.L7\n\
                            .|F--J\n\
                            .LJ...";

    const INPUT3: &[u8] = b"...........\n\
                            .S-------7.\n\
                            .|F-----7|.\n\
                            .||.....||.\n\
                            .||.....||.\n\
                            .|L-7.F-J|.\n\
                            .|..|.|..|.\n\
                            .L--J.L--J.\n\
                            ...........";

    const INPUT4: &[u8] = b"..........\n\
                            .S------7.\n\
                            .|F----7|.\n\
                            .||....||.\n\
                            .||....||.\n\
                            .|L-7F-J|.\n\
                            .|..||..|.\n\
                            .L--JL--J.\n\
                            ..........";

    const INPUT5: &[u8] = b".F----7F7F7F7F-7....\n\
                            .|F--7||||||||FJ....\n\
                            .||.FJ||||||||L7....\n\
                            FJL7L7LJLJ||LJ.L-7..\n\
                            L--J.L7...LJS7F-7L7.\n\
                            ....F-J..F7FJ|L7L7L7\n\
                            ....L7.F7||L7|.L7L7|\n\
                            .....|FJLJ|FJ|F7|.LJ\n\
                            ....FJL-7.||.||||...\n\
                            ....L---J.LJ.LJLJ...";

    const INPUT6: &[u8] = b"....................\n\
                            FF7FSF7F7F7F7F7F---7\n\
                            L|LJ||||||||||||F--J\n\
                            FL-7LJLJ||||||LJL-77\n\
                            F--JF--7||LJLJ7F7FJ-\n\
                            L---JF-JLJ.||-FJLJJ7\n\
                            |F|F-JF---7F7-L7L|7|\n\
                            |FFJF7L7F-JF7|JL---7\n\
                            7-L-JL7||F7|L7F-7F7|\n\
                            L.L7LFJ|||||FJL7||LJ\n\
                            L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT1), 4);
        assert_eq!(part_1(INPUT2), 8);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(INPUT1), 1);
        assert_eq!(part_2(INPUT2), 1);
        assert_eq!(part_2(INPUT3), 4);
        assert_eq!(part_2(INPUT4), 4);
        assert_eq!(part_2(INPUT5), 8);
        assert_eq!(part_2(INPUT6), 10);
    }
}
