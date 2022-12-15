use itertools::Itertools;
use std::fs;

// an assignment pair will contain two ranges of assignments,
// each with a bottom and top of the range
type AssignmentPair = ((u32, u32), (u32, u32));

fn pair_contains(assignment: AssignmentPair) -> bool {
    // check if either assignment is contained within the other
    (assignment.0 .0 <= assignment.1 .0 && assignment.0 .1 >= assignment.1 .1)
        || (assignment.1 .0 <= assignment.0 .0 && assignment.1 .1 >= assignment.0 .1)
}

fn pair_overlaps(assignment: AssignmentPair) -> bool {
    // check if either assignment overlaps the other
    (assignment.0 .1 >= assignment.1 .0 && assignment.0 .0 <= assignment.1 .1)
        || (assignment.1 .1 >= assignment.0 .0 && assignment.1 .0 <= assignment.0 .1)
}

fn parse_range(range: &str) -> (u32, u32) {
    range
        // get top and bottom of the range
        .split('-')
        // convert numeric string to integer
        .map(|bound| bound.parse::<u32>().unwrap())
        // collect into tuple and unwrap the returned Option
        .collect_tuple()
        .unwrap()
}

fn parse_assignments_part_one(assignments: &str) -> i64 {
    assignments
        .lines()
        // get each line as the numeric strings that make up its ranges
        .map(|line| line.split(',').collect::<Vec<&str>>())
        // turn each pair of ranges into an AssinmentPair
        .map(|ranges| (parse_range(ranges[0]), parse_range(ranges[1])))
        // get only the AssignmentPairs where one contains the other
        .filter(|assignment| pair_contains(*assignment))
        // map all passing AssignmentPairs to 1 for summing up total overlaps
        .map(|_| 1)
        .sum()
}

fn parse_assignments_part_two(assignments: &str) -> i64 {
    assignments
        .lines()
        // get each line as the numeric strings that make up its ranges
        .map(|line| line.split(',').collect::<Vec<&str>>())
        // turn each pair of ranges into an AssinmentPair
        .map(|ranges| (parse_range(ranges[0]), parse_range(ranges[1])))
        // get only the AssignmentPairs where one overlaps with the other
        .filter(|assignment| pair_overlaps(*assignment))
        // map all passing AssignmentPairs to 1 for summing up total overlaps
        .map(|_| 1)
        .sum()
}

fn main() {
    let path = "../inputs/day4.txt";
    let assignments = fs::read_to_string(path).expect("File Read Error");

    println!("Part 1:\n{}", parse_assignments_part_one(&assignments));
    println!("Part 2:\n{}", parse_assignments_part_two(&assignments));
}
