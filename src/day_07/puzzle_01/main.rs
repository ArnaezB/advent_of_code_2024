use std::cmp::Ordering;

fn main() {
    let ops = parse_file();
    let mut result = 0;

    for op in ops {
        if valid_ops(&op) {
            result += op[0];
        }
    }
    println!("{}", result)
}

fn valid_ops(op: &Vec<i64>) -> bool {
    _valid_ops(op[0], &op[2..], op[1])
}

fn _valid_ops(result: i64, vec: &[i64], acc: i64) -> bool {
    match acc.cmp(&result) {
        Ordering::Greater => false,
        Ordering::Equal => {
            if vec.len() == 0 {
                return true;
            }
            _valid_ops(result, &vec[1..], acc * vec[0]) || _valid_ops(result, &vec[1..], acc + vec[0])
        }
        Ordering::Less => {
            if vec.len() == 0 {
                return false;
            }
            _valid_ops(result, &vec[1..], acc * vec[0]) || _valid_ops(result, &vec[1..], acc + vec[0])
        }
    }
}

fn parse_file() -> Vec<Vec<i64>> {
    let input = include_str!("../input");
    let mut ops: Vec<Vec<i64>> = vec![];

    for line in input.lines() {
        let slices = line.split(&[' ', ':']);
        ops.push(slices.filter(|x| *x != "").map(|x| {
            x.parse().unwrap()
        }).collect::<Vec<i64>>())
    }

    ops
}