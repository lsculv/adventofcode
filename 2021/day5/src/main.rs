use std::fs;

fn part_1(input: &str) -> u64 {
    // Parse the input
    let mut lines: Vec<[[i32; 2]; 2]> = Vec::new();
    let mut coordinate_buf = [[0i32; 2]; 2];

    for line in input.lines() {
        line.split(&[' ', ','])
            .filter_map(|s| s.parse::<i32>().ok())
            .enumerate()
            .for_each(|(i, n)| coordinate_buf[i % 2][i / 2] = n);
        lines.push(coordinate_buf);
    }

    lines.retain(|buf| buf[0][0] == buf[0][1] || buf[1][0] == buf[1][1]);

    for line in lines.iter_mut() {
        line[0].sort_unstable();
        line[1].sort_unstable();
    }

    let mut vent_field = vec![vec![0i32; 1000]; 1000];
    for line in lines {
        for x in line[0][0]..=line[0][1] {
            for y in line[1][0]..=line[1][1] {
                vent_field[x as usize][y as usize] += 1;
            }
        }
    }

    vent_field.into_iter().flatten().filter(|n| n > &1).count() as u64
}

fn part_2(input: &str) -> u64 {
    // Parse the input
    let mut lines: Vec<[[i32; 2]; 2]> = Vec::new();
    let mut coordinate_buf = [[0i32; 2]; 2];

    for line in input.lines() {
        line.split(&[' ', ','])
            .filter_map(|s| s.parse::<i32>().ok())
            .enumerate()
            .for_each(|(i, n)| coordinate_buf[i % 2][i / 2] = n);
        lines.push(coordinate_buf);
    }

    let (mut flat, diag): (Vec<[[i32; 2]; 2]>, Vec<[[i32; 2]; 2]>) = lines
        .into_iter()
        .partition(|buf| buf[0][0] == buf[0][1] || buf[1][0] == buf[1][1]);

    for line in flat.iter_mut() {
        line[0].sort_unstable();
        line[1].sort_unstable();
    }

    let mut vent_field = vec![vec![0i32; 1000]; 1000];
    for line in flat {
        for x in line[0][0]..=line[0][1] {
            for y in line[1][0]..=line[1][1] {
                vent_field[x as usize][y as usize] += 1;
            }
        }
    }

    let mut x1: i32;
    let mut y1: i32;
    let mut x2: i32;
    let mut y2: i32;
    for line in diag {
        if line[0][0] < line[0][1] {
            x1 = line[0][0];
            x2 = line[0][1];
            y1 = line[1][0];
            y2 = line[1][1];
        } else {
            x1 = line[0][1];
            x2 = line[0][0];
            y1 = line[1][1];
            y2 = line[1][0];
        }

        for x in x1..=x2 {
            vent_field[x as usize][(((y1 - y2) / (x1 - x2)) * (x - x1) + y1) as usize] += 1;
        }
    }

    vent_field.into_iter().flatten().filter(|n| n > &1).count() as u64
}

fn main() {
    let input = fs::read_to_string("../inputs/day5.txt").expect("Could not open input file");
    println!("Part 1:\t{}", part_1(&input));
    println!("Part 2:\t{}", part_2(&input));
}
/*
fn parse(input: &str) -> Vec<[[i32; 2]; 2]> {
    let mut lines: Vec<[[i32; 2]; 2]> = Vec::new();
    let mut coordinate_buf = [[0i32; 2]; 2];
    println!("BEFORE PARSING");
    for line in input.lines() {
        line.split(&[' ', ','])
            .filter_map(|s| s.parse::<i32>().ok())
            .enumerate()
            .for_each(|(i, n)| coordinate_buf[i % 2][i / 2] = n);
        lines.push(coordinate_buf);
    }
    lines
}
fn cull(lines: &mut Vec<[[i32; 2]; 2]>) {
    println!("BEFORE NON HORI/VERT CULLING");
    lines.retain(|buf| buf[0][0] == buf[0][1] || buf[1][0] == buf[1][1]);
}

fn sort(lines: &mut Vec<[[i32; 2]; 2]>) {
    println!("BEFORE SORTING");
    for line in lines.iter_mut() {
        line[0].sort_unstable();
        line[1].sort_unstable();
    }
}

fn value_set(lines: Vec<[[i32; 2]; 2]>) -> Vec<Vec<i32>> {
    println!("BEFORE VENT FIELD VALUE SETTING");
    let mut vent_field = vec![vec![0i32; 1000]; 1000];
    println!("AFTER ALLOCATION");
    for line in lines {
        for x in line[0][0]..=line[0][1] {
            for y in line[1][0]..=line[1][1] {
                println!("WE GOT INSIDE THE LOOP");
                vent_field[x as usize][y as usize] += 1;
            }
        }
    }
    vent_field
}

fn count(vent_field: Vec<Vec<i32>>) -> u64 {
    println!("BEFORE COUNTING");
    vent_field.into_iter().flatten().filter(|n| n > &1).count() as u64
}
fn part_1(input: &str) -> u64 {
    let mut lines = parse(input);
    println!("{:?}", lines);
    cull(&mut lines);
    println!("{:?}", lines);
    sort(&mut lines);
    println!("{:?}", lines);
    let vent_field = value_set(lines);
    count(vent_field)
}
*/
