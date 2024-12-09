use std::collections::HashMap;
use std::fs;

fn main() {
    let input = parse_file();

    let hashmap = to_fb_hashmap(&input.rules);

    let mut result = 0;
    for mut instruction in input.instructions {
        if !validate_instruction(&instruction, &hashmap) {
            reorder_instruction(&mut instruction, &hashmap);
            result += instruction[((instruction.len() / 2) as f32).ceil() as usize]
        }
    }

    print!("{result}")
}

fn reorder_instruction(instruction: &mut [u32], rules: &HashMap<u32, Vec<u32>>) {
    for i in 1..instruction.len() {
        let page = instruction[i];

        let cannot_be_before = match rules.get(&page) {
            Some(value) => value,
            None => &vec!()
        };

        let slice = &instruction[0..i];
        for j in 0..slice.len() {
            let item = slice[j];
            if cannot_be_before.contains(&item) {
                let tmp = instruction[i];
                instruction[i] = instruction[j];
                instruction[j] = tmp;
                return reorder_instruction(instruction, rules);
            }
        }
    }
}

fn to_fb_hashmap(items: &Vec<(u32, u32)>) -> HashMap<u32, Vec<u32>> {
    let mut hashmap: HashMap<u32, Vec<u32>> = HashMap::new();

    for (key, value) in items {
        match hashmap.get_mut(key) {
            Some(vec) => vec.push(*value),
            None => { hashmap.insert(*key, vec!(*value)); }
        }
    }

    hashmap
}

fn parse_file() -> InputData {
    let file = fs::read_to_string("src/day_05/input").unwrap();
    let mut rules: Vec<(u32, u32)> = vec!();
    let mut instructions: Vec<Vec<u32>> = vec!();

    let mut reading_rules = true;

    for line in file.lines() {
        if line.is_empty() {
            reading_rules = false;
            continue;
        }

        if reading_rules {
            let numbers = line.split('|')
                .map(|x| { x.parse::<u32>().unwrap() })
                .collect::<Vec<u32>>();
            rules.push((numbers[0], numbers[1]));
        } else {
            instructions.push(
                line.split(',')
                    .map(|x| { x.parse::<u32>().unwrap() })
                    .collect::<Vec<u32>>()
            )
        }
    }

    InputData { rules, instructions }
}

fn validate_instruction(instruction: &Vec<u32>, valid_order: &HashMap<u32, Vec<u32>>) -> bool {
    let empty_vec = vec!();

    for (i, e) in instruction.iter().enumerate() {
        let cannot_be_before_e = match valid_order.get(e) {
            Some(e) => e,
            _ => &empty_vec
        };

        for j in &instruction[0..i] {
            if cannot_be_before_e.contains(j) {
                return false;
            }
        }
    }

    true
}

#[derive(Debug)]
struct InputData {
    rules: Vec<(u32, u32)>,
    instructions: Vec<Vec<u32>>,
}