use std::fs;
use regex::Regex;

fn main() {
    let file = fs::read_to_string("src/day_03/input").unwrap();

    let regex = Regex::new(r"mul\(\d+,\d+\)").unwrap();
    let mut result = 0;
    for coincidence in regex.find_iter(&file){
        let substring = &file[coincidence.start()..coincidence.end()];

        let (first, second) = extract_numbers(substring);
        result += first * second;
    }

    println!("{result}");
}

fn extract_numbers(s: &str) -> (u32, u32) {
    let (i_first_number, i_second_number) = (s.find('(').unwrap() + 1, s.find(',').unwrap() + 1);

    let first_number = s[i_first_number..i_second_number-1].parse::<u32>().unwrap();
    let second_number = s[i_second_number..s.len()-1].parse::<u32>().unwrap();

    (first_number, second_number)
}