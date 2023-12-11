fn main() {
    let input = include_bytes!("../day11.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &[u8]) -> usize {
    let space = parse_input(input);
    let (rows, cols) = get_space_expansions(&space);
    let galaxies = find_galaxies(&space);
    sum_distances(&galaxies, &cols, &rows, 2)
}

fn part_2(input: &[u8]) -> usize {
    let space = parse_input(input);
    let (rows, cols) = get_space_expansions(&space);
    let galaxies = find_galaxies(&space);
    sum_distances(&galaxies, &cols, &rows, 1_000_000)
}

type Coord = (isize, isize);

fn parse_input(input: &[u8]) -> Vec<Vec<u8>> {
    let space: Vec<Vec<u8>> = input
        .split(|&b| b == b'\n')
        .map(|line| line.to_vec())
        .collect();
    space
}

fn find_galaxies(space: &[Vec<u8>]) -> Vec<Coord> {
    let mut galaxies = Vec::new();
    for i in 0..space.len() {
        for j in 0..space[0].len() {
            if space[i][j] == b'#' {
                galaxies.push((i as isize, j as isize))
            }
        }
    }

    galaxies
}

fn sum_distances(galaxies: &[Coord], cols: &[usize], rows: &[usize], expansion: isize) -> usize {
    let mut sum = 0;
    for galaxy in galaxies.iter() {
        for other in galaxies.iter() {
            if galaxy == other {
                continue;
            }
            let (y1, x1) = galaxy;
            let (y2, x2) = other;
            let mut y_dist = (y1 - y2).abs();
            let mut x_dist = (x1 - x2).abs();
            let hori = if x1 > x2 { x2..=x1 } else { x1..=x2 };
            let vert = if y1 > y2 { y2..=y1 } else { y1..=y2 };
            for row in rows.iter() {
                let row = *row as isize;
                if vert.contains(&&row) {
                    y_dist += expansion - 1;
                }
            }

            for col in cols.iter() {
                let col = *col as isize;
                if hori.contains(&&col) {
                    x_dist += expansion - 1;
                }
            }

            sum += x_dist + y_dist;
        }
    }
    sum as usize / 2
}

fn get_space_expansions(space: &[Vec<u8>]) -> (Vec<usize>, Vec<usize>) {
    let mut rows_to_expand = Vec::new();
    for (i, row) in space.iter().enumerate() {
        if !row.contains(&b'#') {
            rows_to_expand.push(i);
        }
    }

    let mut cols_to_expand = Vec::new();
    'outer: for j in 0..space[0].len() {
        for row in space.iter() {
            if row[j] == b'#' {
                continue 'outer;
            }
        }
        cols_to_expand.push(j);
    }

    (rows_to_expand, cols_to_expand)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &[u8] = b"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(INPUT), 374);
    }

    #[test]
    fn test_part_2() {
        let space = parse_input(INPUT);
        let (rows, cols) = get_space_expansions(&space);
        let galaxies = find_galaxies(&space);
        let ten = sum_distances(&galaxies, &cols, &rows, 10);
        let hundred = sum_distances(&galaxies, &cols, &rows, 100);
        assert_eq!(ten, 1030);
        assert_eq!(hundred, 8410);
    }
}
