use std::collections::HashMap;

const ITERS: usize = 75;

fn main() {
    let mut map = parse_file();

    for _ in 0..ITERS {
        let mut tmp_map = HashMap::new();

        for (k, v) in map {
            let new_keys = next_step(k);

            for nk in new_keys {
                match tmp_map.get(&nk) {
                    None => {
                        tmp_map.insert(nk, v);
                    }
                    Some(x) => {
                        tmp_map.insert(nk, x + v);
                    }
                }
            }
        }

        map = tmp_map;
    }

    let result = map.into_values().sum::<u64>();

    println!("{}", result)
}

fn parse_file() -> HashMap<u64, u64> {
    let mut map = HashMap::new();
    let input = include_str!("../input");

    for n in input.split(' ') {
        let n = n.parse::<u64>().unwrap();

        match map.get(&n) {
            None => {
                map.insert(n, 1);
            }
            Some(v) => {
                map.insert(n, v + 1);
            }
        }
    }

    map
}

fn next_step(val: u64) -> Vec<u64> {
    let log: u64 = if val == 0 {
        0
    } else {
        (u64::ilog10(val) + 1) as u64
    };

    match val {
        0 => vec![1],
        x if log % 2 == 0 => {
            let pow = u64::pow(10, (log / 2u64) as u32);
            vec![x / pow, x % pow]
        }
        x => vec![x * 2024],
    }
}