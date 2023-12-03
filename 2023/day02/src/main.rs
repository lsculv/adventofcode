fn main() {
    let input = include_str!("../day02.txt");
    println!("Part 1: {}", part_1(input));
    println!("Part 2: {}", part_2(input));
}

fn part_1(input: &str) -> usize {
    let mut sum = 0;
    for (i, line) in input.lines().enumerate() {
        let mut possible = true;
        let line = line.split_once(':').unwrap().1;
        'outer: for cubes in line.split([',', ';']) {
            let (count, color) = cubes.trim_start().split_once(' ').unwrap();
            let count = count.parse::<usize>().unwrap();
            match color {
                "red" => {
                    if count > 12 {
                        possible = false;
                        break 'outer;
                    }
                }
                "green" => {
                    if count > 13 {
                        possible = false;
                        break 'outer;
                    }
                }
                "blue" => {
                    if count > 14 {
                        possible = false;
                        break 'outer;
                    }
                }
                _ => unreachable!("rgb are the only colors"),
            };
        }

        if possible {
            sum += i + 1
        }
    }

    sum
}

fn part_2(input: &str) -> usize {
    let mut sum = 0;
    let mut reds = Vec::with_capacity(8);
    let mut greens = Vec::with_capacity(8);
    let mut blues = Vec::with_capacity(8);
    for line in input.lines() {
        let line = line.split_once(':').unwrap().1;
        for cubes in line.split([',', ';']) {
            let (count, color) = cubes.trim_start().split_once(' ').unwrap();
            let count = count.parse::<usize>().unwrap();
            match color {
                "red" => reds.push(count),
                "green" => greens.push(count),
                "blue" => blues.push(count),
                _ => unreachable!("rgb are the only colors"),
            };
        }

        sum += reds.iter().max().unwrap_or(&0)
            * greens.iter().max().unwrap_or(&0)
            * blues.iter().max().unwrap_or(&0);

        reds.clear();
        greens.clear();
        blues.clear();
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_1(input), 8);
    }

    #[test]
    fn test_part_2() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part_2(input), 2286);
    }
}
