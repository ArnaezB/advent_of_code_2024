#[allow(dead_code)]
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;

fn main() {
    let (mut disk, mut slices, mut empty_slices) = parse_file();

    for i in (0..slices.len()).rev() {
        let Slice { id, start, end } = slices[i];
        let size = end - start;
        match first_of_size_n(&empty_slices, size) {
            Some((index, vec_index)) => {
                if vec_index > start { continue; } else {
                    disk[vec_index..vec_index + size].copy_from_slice(&vec![DiskBlock { id }; size]);
                    disk[start..end].copy_from_slice(&vec![DiskBlock { id: -1 }; size])
                }
                let Slice { start, end, .. } = &empty_slices[index];
                if start + size > *end {
                    empty_slices.remove(index);
                } else {
                    empty_slices[index] = Slice {
                        id: -1,
                        start: start + size,
                        end: *end,
                    }
                }
            }
            None => ()
        }

        slices.pop();
    }

    println!("{:?}", checksum(disk));
}

fn checksum(vec: Vec<DiskBlock>) -> u64 {
    let mut result = 0;

    for (block, i) in vec.iter().zip(0u64..) {
        if block.id == -1 { continue; }
        let partial = i * block.id as u64;

        result += partial;
    }

    result
}

fn first_of_size_n(vec: &Vec<Slice>, n: usize) -> Option<(usize, usize)> {
    for (i, block) in vec.iter().enumerate() {
        if block.end - block.start >= n {
            return Some((i, block.start));
        }
    }

    None
}

fn parse_file() -> (Vec<DiskBlock>, Vec<Slice>, Vec<Slice>) {
    let input = include_str!("../input");

    let mut disk_map = vec![];
    let mut slices = vec![];
    let mut empty_slices = vec![];

    let mut is_block = true;
    let mut id = 0;
    let mut prev_block_end = 0;


    for char in input.chars() {
        let char_value = char.to_digit(10).unwrap_or_else(|| {
            println!("Error at {}", char);
            panic!();
        }) as usize;

        if is_block {
            push_blocks(&mut disk_map, &char_value, &id);
            slices.push(Slice { id, start: prev_block_end, end: prev_block_end + char_value });
            id += 1;
            prev_block_end += char_value;
        } else {
            push_blocks(&mut disk_map, &char_value, &-1);
            empty_slices.push(Slice { id: -1, start: prev_block_end, end: prev_block_end + char_value });
            prev_block_end += char_value;
        }
        is_block = !is_block;
    }

    (disk_map, slices, empty_slices)
}

#[derive(Debug)]
struct Slice {
    id: i32,
    start: usize,
    end: usize,
}

#[derive(Copy, Clone)]
struct DiskBlock {
    id: i32,
}

impl Deref for DiskBlock {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.id
    }
}

impl Display for DiskBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self.id {
            -1 | -2 => ".",
            c => &*c.to_string()
        };
        write!(f, "{}", c)
    }
}

fn disk_to_str(disk: &Vec<DiskBlock>) -> String {
    let mut str = String::new();

    for block in disk {
        str.push_str(&*format!("{block}"))
    }

    str
}

fn push_blocks(vec: &mut Vec<DiskBlock>, n: &usize, id: &i32) {
    for _ in 0..*n {
        vec.push(DiskBlock { id: *id })
    }
}