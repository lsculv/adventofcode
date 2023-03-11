use std::{collections::HashSet, fs};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Board {
    lines: [[u32; 5]; 10],
    matches: [[bool; 5]; 10],
}

impl Board {
    fn from(array: [[u32; 5]; 10]) -> Self {
        Board {
            lines: array,
            matches: [[false; 5]; 10],
        }
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (header, tail) = input.split_once('\n').unwrap();
    let nums: Vec<u32> = header.split(',').map(|c| c.parse().unwrap()).collect();

    let mut line_buf = [[0u32; 5]; 10];
    let mut boards: Vec<Board> = Vec::new();
    tail.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u32>>()
        .chunks(25)
        .for_each(|chunk| {
            // Setting the rows
            for i in 0..25 {
                line_buf[i / 5][i % 5] = chunk[i]
            }
            // Setting the columns
            for i in 0..25 {
                line_buf[i / 5 + 5][i % 5] = chunk[i / 5 + 5 * (i % 5)]
            }
            boards.push(Board::from(line_buf));
        });

    (nums, boards)
}

fn score_board(winning_board: &Board, winning_num: u32) -> u32 {
    let mut score = 0;
    for line in 0..5 {
        for (i, val) in winning_board.lines[line].iter().enumerate() {
            if !winning_board.matches[line][i] {
                score += val;
            }
        }
    }
    score *= winning_num;

    score
}

fn part_1(nums: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    // Find the winning board
    let mut winning_board: Board = Board::from([[0; 5]; 10]);
    let mut winning_num = 0u32;
    'outer: for num in nums {
        for board in boards.iter_mut() {
            for (i, line) in board.lines.iter().enumerate() {
                for (j, val) in line.iter().enumerate() {
                    if val == num {
                        board.matches[i][j] = true;
                    }
                }
            }
            for k in 0..10 {
                if board.matches[k] == [true; 5] {
                    winning_board = *board;
                    winning_num = *num;
                    break 'outer;
                }
            }
        }
    }

    score_board(&winning_board, winning_num)
}

fn part_2(nums: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    const TOTAL_BOARDS: usize = 100;

    // Find the last winning board
    let mut last_winning_board: Board = Board::from([[0; 5]; 10]);
    let mut last_winning_num = 0u32;
    let mut winners: HashSet<[[u32; 5]; 10]> = HashSet::new();
    'outer: for num in nums {
        for board in boards.iter_mut() {
            for (i, line) in board.lines.iter().enumerate() {
                for (j, val) in line.iter().enumerate() {
                    if val == num {
                        board.matches[i][j] = true;
                    }
                }
            }
            'line_check: for k in 0..10 {
                if board.matches[k] == [true; 5] {
                    winners.insert(board.lines);
                    break 'line_check;
                }
            }
            if winners.len() == TOTAL_BOARDS {
                last_winning_board = *board;
                last_winning_num = *num;
                break 'outer;
            }
        }
    }

    score_board(&last_winning_board, last_winning_num)
}

fn main() {
    let input = fs::read_to_string("../inputs/day4.txt").expect("Could not open input file");
    let (nums, boards) = parse_input(&input);
    println!("Part 1:\t{}", part_1(&nums, boards.clone()));
    println!("Part 2:\t{}", part_2(&nums, boards));
}
