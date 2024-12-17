use itertools::Itertools;
use std::fmt::{Display, Formatter};

fn main() {
    let mut disk = parse_file();

    // println!("{}", disk);

    disk.defragment();

    let checksum = disk.checksum();

    println!("{}", checksum)
}

fn parse_file() -> DiskMap {
    let input = include_str!("../input");
    let mut disk_map = DiskMap::new();
    let mut is_block = true;
    let mut id = 0;

    for char in input.chars() {
        let char_value = char.to_digit(10).unwrap_or_else(|| {
            println!("Error at {}", char);
            panic!();
        });

        if is_block {
            disk_map.add_blocks(DiskBlock::new(id), char_value);
            id += 1;
        } else {
            disk_map.add_free_spaces(char_value);
        }
        is_block = !is_block;
    }

    disk_map
}

#[derive(Clone, Copy)]
struct DiskBlock {
    id: i32,
}

impl DiskBlock {
    fn new(id: i32) -> Self {
        Self { id }
    }
    fn empty() -> DiskBlock {
        DiskBlock::new(-1)
    }
}

struct DiskMap {
    blocks: Vec<DiskBlock>,
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let strings = self.blocks.iter()
            .map(|DiskBlock { id }| {
                match id {
                    -1 => String::from('.'),
                    _ => id.to_string()
                }
            }).join("");

        write!(f, "{}", strings)
    }
}

impl DiskMap {
    fn new() -> Self {
        Self { blocks: vec![] }
    }
    fn add_blocks(&mut self, block: DiskBlock, n: u32) {
        for _ in 0..n {
            self.blocks.push(block.clone());
        }
    }
    fn add_free_spaces(&mut self, n: u32) {
        self.add_blocks(DiskBlock::empty(), n)
    }
    fn move_block(&mut self, old_i: usize, new_i: usize) {
        self.blocks[new_i] = self.blocks[old_i];
        self.blocks[old_i] = DiskBlock::empty();
    }
    fn defragment(&mut self) {
        let mut reader = DiskMapReader::new(self);

        loop {
            let f = reader.get_first_free_block(self);
            let b = reader.get_last_block(self);

            if f >= b {
                break;
            }

            self.move_block(b as usize, f as usize);
            // println!("{}", self);
        }
    }

    fn checksum(self) -> u64 {
        let mut result = 0;

        for (block, i) in self.blocks.iter().zip(0u64..) {
            if block.id == -1 { break; }
            let partial = i * block.id as u64;

            // println!("{} * {} = {}", i, block.id, partial);

            result += partial;
        }

        result
    }
}

struct DiskMapReader {
    last_first_free: usize,
    last_last_block: usize,
}

impl DiskMapReader {
    fn new(map: &DiskMap) -> DiskMapReader {
        Self { last_first_free: 0, last_last_block: map.blocks.len() - 1 }
    }
    fn get_first_free_block(&mut self, map: &DiskMap) -> u32 {
        let mut i = self.last_first_free;

        while map.blocks[i].id != -1 && i < map.blocks.len() {
            i += 1;
        }

        self.last_first_free = i;
        i as u32
    }
    fn get_last_block(&mut self, map: &DiskMap) -> u32 {
        let mut i = self.last_last_block;

        while map.blocks[i].id == -1 && i < map.blocks.len() {
            i -= 1;
        }

        self.last_last_block = i;
        i as u32
    }
}