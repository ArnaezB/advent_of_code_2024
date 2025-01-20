use std::time::Instant;

fn main() {
    let (towels, designs) = parse_file();
    let start = Instant::now();

    let valids = designs
        .iter()
        .map(|d| valid_design(&*d, &towels))
        .filter(|&b| b)
        .count();

    println!("{} in {}ms", valids, start.elapsed().as_millis())
}

fn valid_design<'a>(design: &'a str, towels: &Vec<String>) -> bool {
    design.len() == 0
        || towels
            .iter()
            .any(|t| design.starts_with(&*t) && valid_design(&design[t.len()..], towels))
}

fn parse_file() -> (Vec<String>, Vec<String>) {
    let input = include_str!("../input");
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .split(", ")
            .map(|x| x.to_owned())
            .collect(),
        lines.skip(1).map(|x| x.to_owned()).collect(),
    )
}