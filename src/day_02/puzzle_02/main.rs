use std::{fs};

fn main() {
    let file = fs::read_to_string("src/day_02/input").unwrap();
    let mut list: Vec<Vec<u32>> = vec!();

    for line in file.lines() {
        let items = line
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        list.push(items);
    }

    let mut safe_reports = 0;
    for report in list {
        match process(&report) {
            Ok(_) => safe_reports += 1,
            Err((i, j)) => {
                let sub_a = &[&report[0..i], &report[i + 1..]].concat();
                let sub_b = &[&report[0..j], &report[j + 1..]].concat();

                let (result_a, result_b) = (process(sub_a), process(sub_b));
                match (result_a, result_b) {
                    (_, Ok(_)) => safe_reports += 1,
                    (Ok(_), _) => safe_reports += 1,
                    _ => ()
                }
            }
        }
    }
    println!("{}", safe_reports);
}

fn process(report: &Vec<u32>) -> Result<(), (usize, usize)> {
    let ascending_order = report[0] < report[report.len() - 1];

    for i in 0..(report.len() - 1) {
        let (current, next) = (report[i], report[i + 1]);
        let diff = current.abs_diff(next);

        if diff == 0 || diff > 3 {
            return Err((i, i+1));
        }

        if (ascending_order && current > next) || (!ascending_order && current < next) {
            return Err((i, i+1));
        }
    }
    Ok(())
}