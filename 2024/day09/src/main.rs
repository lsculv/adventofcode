#![feature(array_chunks)]
#![feature(array_repeat)]

use std::{
    array,
    fs::{self, File},
    io::{self, Read},
};

type FileSystem = Vec<DiskSection>;

#[derive(Debug, Clone, Copy)]
struct DiskSection {
    full_blocks: u8,
    free_blocks: u8,
}

fn part1(input: &[u8]) -> usize {
    let mut chunks = input.array_chunks::<2>();
    let mut filesystem: FileSystem = chunks
        .by_ref()
        .map(|[full, free]| DiskSection {
            full_blocks: full - b'0',
            free_blocks: free - b'0',
        })
        .collect();
    if let Some(full) = chunks.remainder().first() {
        filesystem.push(DiskSection {
            full_blocks: full - b'0',
            free_blocks: 0,
        });
    }

    let mut checksum = 0;
    let mut pos = 0;
    for i in 0..filesystem.len() {
        let mut section = filesystem[i];
        for _ in 0..section.full_blocks {
            checksum += i * pos;
            pos += 1;
        }

        for j in ((i + 1)..filesystem.len()).rev() {
            let mut defrag_section = filesystem[j];
            while defrag_section.full_blocks > 0 && section.free_blocks > 0 {
                defrag_section.full_blocks -= 1;
                defrag_section.free_blocks += 1;

                section.free_blocks -= 1;
                section.full_blocks += 1;

                checksum += j * pos;
                pos += 1;
            }

            filesystem[j] = defrag_section;
            if section.free_blocks == 0 {
                filesystem[i] = section;
                break;
            }
        }

        let mut seen_empty = false;
        let has_gap = filesystem.iter().skip(i).any(|section| {
            seen_empty = section.free_blocks > 0;
            seen_empty && section.full_blocks > 0
        });

        if !has_gap {
            break;
        }
    }

    checksum
}

type FileSystemP2 = Vec<DiskSectionP2>;

#[derive(Debug, Clone, Copy)]
enum Block {
    Full(u32),
    Free,
    Unused,
}

#[derive(Debug, Clone)]
struct DiskSectionP2 {
    file_size: u8,
    full_blocks: u8,
    free_blocks: u8,
    blocks: [Block; 18],
}

fn part2(input: &[u8]) -> usize {
    let mut chunks = input.array_chunks::<2>();
    let mut filesystem: FileSystemP2 = chunks
        .by_ref()
        .enumerate()
        .map(|(i, [full, free])| {
            let full_blocks = *full - b'0';
            let free_blocks = *free - b'0';
            let full = full_blocks as usize;
            let free = free_blocks as usize;
            let i = i as u32;
            let mut blocks = array::repeat(Block::Unused);
            blocks[0..full].fill(Block::Full(i));
            blocks[full..(full + free)].fill(Block::Free);

            DiskSectionP2 {
                blocks,
                full_blocks,
                file_size: full_blocks,
                free_blocks,
            }
        })
        .collect();
    if let Some(full) = chunks.remainder().first() {
        let i = filesystem.len() as u32;
        let full_blocks = *full - b'0';
        let free_blocks = 0;
        let full = full_blocks as usize;

        let mut blocks = array::repeat(Block::Unused);
        blocks[0..full].fill(Block::Full(i));
        filesystem.push(DiskSectionP2 {
            blocks,
            full_blocks,
            file_size: full_blocks,
            free_blocks,
        });
    }

    'outer: for i in (0..filesystem.len()).rev() {
        let mut section = filesystem[i].clone();

        for j in 0..i {
            let mut candidate = filesystem[j].clone();
            if candidate.free_blocks >= section.file_size {
                let start = candidate.full_blocks as usize;
                let end = start + section.file_size as usize;
                candidate.blocks[start..end].fill(Block::Full(i as u32));

                let start = 0;
                let end = section.file_size as usize;
                section.blocks[start..end].fill(Block::Free);

                candidate.free_blocks -= section.file_size;
                candidate.full_blocks += section.file_size;

                section.free_blocks += section.file_size;
                section.full_blocks = 0;

                filesystem[i] = section;
                filesystem[j] = candidate;
                continue 'outer;
            }
        }
    }

    let mut pos = 0;
    filesystem
        .into_iter()
        .flat_map(|section| section.blocks)
        .map(|block| match block {
            Block::Full(id) => {
                pos += 1;
                (pos - 1) * id as usize
            }
            Block::Free => {
                pos += 1;
                0
            }
            Block::Unused => 0,
        })
        .sum()
}

fn main() -> io::Result<()> {
    let input_file = "day09.txt";
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
        let input = b"2333133121414131402";
        assert_eq!(part1(input), 1928);
    }

    #[test]
    fn test_part2() {
        let input = b"2333133121414131402";
        assert_eq!(part2(input), 2858);
    }
}
