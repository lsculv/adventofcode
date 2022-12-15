use std::fs;

fn parse_inventory(inventory: &str) -> Vec<u32> {
    let mut parsed_inventory: Vec<u32> = vec![];
    // iterate over each individual collection of items in the whole inventory
    for item in inventory.split("\n\n").collect::<Vec<&str>>() {
        parsed_inventory.push(
            // split string again in to each number
            item.split_whitespace()
                // convert each string to integer type
                .map(|item_str| item_str.parse::<u32>().unwrap())
                .sum(),
        );
    }
    // sort the vector of integer totals
    parsed_inventory.sort_unstable_by(|a, b| b.cmp(a));
    return parsed_inventory;
}

fn main() {
    // read in the provided input from a text file
    let path = "../inputs/day1.txt";
    let inventory_input = fs::read_to_string(path).expect("File Read Error");
    let elf_calories = parse_inventory(&inventory_input);

    // print out solutions
    println!("Part 1:\n{}", elf_calories[0]);
    println!("Part 2:\n{}", elf_calories[0..3].iter().sum::<u32>());
}
