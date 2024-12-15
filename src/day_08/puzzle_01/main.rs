use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let (map, size) = parse_file();
    let mut positions_with_antinodes = HashSet::new();

    for entry in &map {
        for (a, b) in entry.1.iter().tuple_combinations() {
            let antinodes = find_antinodes(a, b);
            for (i, node) in validate_new_antinodes(&antinodes, &size).iter().enumerate() {
                if *node == 1 { continue; }
                positions_with_antinodes.insert(antinodes[i]);
            }
        }
    }

    println!("{}", positions_with_antinodes.len())
}

fn validate_new_antinodes(antinodes: &Vec<[i32; 2]>, size: &i32) -> Vec<i32> {
    let mut result = vec![1, 1];

    for (i, antinode) in antinodes.iter().enumerate() {
        let [x, y] = antinode;
        if *x >= 0 && *y >= 0 && x < size && y < size { result[i] = 0 }
    }

    result
}

fn find_antinodes(a: &[i32; 2], b: &[i32; 2]) -> Vec<[i32; 2]> {
    let slope: [i32; 2] = [(a[0] - b[0]), (a[1] - b[1])];

    let antinode_a = [a[0] + slope[0], a[1] + slope[1]];
    let antinode_b = [b[0] - slope[0], b[1] - slope[1]];

    vec![antinode_a, antinode_b]
}

fn parse_file() -> (HashMap<char, Vec<[i32; 2]>>, i32) {
    let input = include_str!("../input");
    let mut map: HashMap<char, Vec<[i32; 2]>> = HashMap::new();
    let mut size = 0;

    for (line, i) in input.lines().zip(0..) {
        for (char, j) in line.chars().zip(0..) {
            if char != '.' {
                match map.get_mut(&char) {
                    Some(positions) => {
                        positions.push([i, j]);
                    }
                    None => {
                        map.insert(char, vec![[i, j]]);
                    }
                }
            }
        }
        size += 1
    }

    (map, size)
}