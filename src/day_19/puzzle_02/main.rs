use hashbrown::HashMap;
use std::time::Instant;

fn main() {
    let (towels, designs) = parse_file();
    let start = Instant::now();

    let mut cache = HashMap::new();
    let sum: u64 = designs
        .iter()
        .map(|d| valid_design(&*d, &towels, &mut cache))
        .sum();

    println!("{} in {}ms", sum, start.elapsed().as_millis())
}

fn valid_design<'a>(
    design: &'a str,
    towels: &Vec<String>,
    cache: &mut HashMap<&'a str, u64>,
) -> u64 {
    if design.len() == 0 {
        return 1;
    }

    if let Some(cached) = cache.get(design) {
        return *cached;
    }

    towels
        .iter()
        .filter(|&t| design.starts_with(&*t))
        .map(|t| {
            let computed = valid_design(&design[t.len()..], towels, cache);
            cache.insert(&design[t.len()..], computed);

            computed
        })
        .sum()
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