use std::{fs, thread};

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

    // Using threads just for experimentation with them
    // not that they are necessary here
    let mut safe_reports = 0;
    let mut threads = vec!();
    for report in list {
        threads.push(thread::spawn(|| {
            process_line(report)
        }));
    }

    for thread in threads {
        match thread.join().unwrap() {
            true => safe_reports += 1,
            false => (),
        }
    }

    println!("{}", safe_reports);
}

fn process_line(report: Vec<u32>) -> bool {
    let mut is_safe = true;
    let ascending_order = report[0] < report[1];

    for i in 0..(report.len() - 1) {
        let (current, next) = (report[i], report[i + 1]);
        let diff = current.abs_diff(next);

        if diff == 0 || diff > 3 { is_safe = false; break}
        if (ascending_order && current > next) || (!ascending_order && current < next) {
            is_safe = false;
            break;
        }
    }
    is_safe
}