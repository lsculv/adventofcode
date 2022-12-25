use itertools::Itertools;
use std::{collections::HashSet, fs, hash::Hash, slice::Windows};

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut unique = HashSet::new();
    iter.into_iter().all(move |x| unique.insert(x))
}

fn find_packet_start(datastream: &str) -> usize {
    datastream
        .char_indices()
        // get each continious window over 4 values
        .collect::<Vec<(usize, char)>>()
        .windows(4)
        // find the first occurance of a slice of characters that is unique
        .find_or_first(|marker| {
            let mut characters: Vec<char> = Vec::new();
            // iterate over all the characters in the slice and collect them in a vector
            for character in marker.iter().map(|c| c.1) {
                characters.push(character);
            }
            // check if the characters are unique
            has_unique_elements(characters)
        })
        .unwrap()[3]
        .0 + 1
}

fn find_message_start(datastream: &str) -> usize {
    datastream
    .char_indices()
    // get each continious window over 4 values
    .collect::<Vec<(usize, char)>>()
    .windows(14)
    // find the first occurance of a slice of characters that is unique
    .find_or_first(|marker| {
        let mut characters: Vec<char> = Vec::new();
        // iterate over all the characters in the slice and collect them in a vector
        for character in marker.iter().map(|c| c.1) {
            characters.push(character);
        }
        // check if the characters are unique
        has_unique_elements(characters)
    })
    .unwrap()[13]
    .0 + 1
}

fn main() {
    let path = "../inputs/day6.txt";
    let datastream = fs::read_to_string(path).expect("File Read Error");

    println!("Part 1:\n{}", find_packet_start(&datastream));
    println!("Part 2:\n{}", find_message_start(&datastream));
}
