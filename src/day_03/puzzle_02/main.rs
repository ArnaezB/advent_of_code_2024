use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("src/day_03/input").unwrap();

    let mut enabled = true;
    let mut result: u32 = 0;

    for (i, character) in file.chars().enumerate() {
        if character == 'm' && enabled {
            match process_mul_op(&file[i..]) {
                Some((x,y)) => result += x * y,
                None => ()
            }
        } else if character == 'd' {
            match process_do_or_dont(&file[i..]) {
                Some(true) => enabled = true,
                Some(false) => enabled = false,
                None => ()
            }
        }
    }

    println!("{result}");
}

fn process_mul_op(s: &str) -> Option<(u32, u32)> {
    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();

    match regex.find(s) {
        Some(m) => {
            if m.start() == 0 {
                Some(extract_numbers(&s[0..m.end()]))
            } else {
                None
            }
        }
        _ => None
    }
}

fn extract_numbers(s: &str) -> (u32, u32) {
    let (i_first_number, i_second_number) = (s.find('(').unwrap() + 1, s.find(',').unwrap() + 1);

    let first_number = s[i_first_number..i_second_number-1].parse::<u32>().unwrap();
    let second_number = s[i_second_number..s.len()-1].parse::<u32>().unwrap();

    (first_number, second_number)
}

fn process_do_or_dont(s: &str) -> Option<bool> {
    if &s[0..4] == "do()" {
        return Some(true)
    } else if &s[0..7] == "don't()" {
        return Some(false)
    }

    None
}