use std::fs;

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

    list_two.sort();

    let mut total_score: i64 = 0;

    for item in list_one {
        let (low, high) = (
            list_two.partition_point(|&x| { x < item }),
            list_two.partition_point(|&x| { x <= item })
        );

        let repetitions = (high - low) as i64;
        total_score += repetitions * item as i64;
    }


    println!("{}", total_score);
}