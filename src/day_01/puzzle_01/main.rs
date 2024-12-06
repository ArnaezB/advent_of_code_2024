use std::fs;
use std::iter::zip;

fn main() {
    let file = fs::read_to_string("src/day_01/input").unwrap();
    let mut list_one: Vec<i32> = vec![];
    let mut list_two: Vec<i32> = vec![];

    for line in file.lines() {
        let items: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        list_one.push(items[0]);
        list_two.push(items[1]);
    }

    list_one.sort();
    list_two.sort();

    let mut result = 0;

    for (one, two) in zip(list_one, list_two) {
        result += (one - two).abs();
    }

    println!("{}", result);
}