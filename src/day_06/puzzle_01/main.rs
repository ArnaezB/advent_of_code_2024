use std::collections::HashSet;
use std::fs;

fn main() {
    let data = parse_file();
    let directions: [[i16; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    let mut selected_direction = 0;
    let mut now_at = data.starting_position;

    let mut visited_positions: HashSet<[i16; 2]> = HashSet::new();
    visited_positions.insert(now_at);

    loop {
        let next = [
            now_at[0] + directions[selected_direction][0],
            now_at[1] + directions[selected_direction][1],
        ];

        if next[0] < 0 || next[0] > data.height || next[1] < 0 || next[1] > data.width {
            break;
        }
        if data.obstacles.contains(&next) {
            selected_direction = (selected_direction + 1) % directions.len();
            continue;
        }

        now_at = next;
        visited_positions.insert(now_at);
    }

    println!("{}", visited_positions.len())
}

fn parse_file() -> InputData {
    let file = fs::read_to_string("src/day_06/input").unwrap();
    let mut obstacles: Vec<[i16; 2]> = vec![];
    let mut starting_position: [i16; 2] = [0; 2];

    let mut max_i = 0;
    let mut max_j = 0;

    for (i, line) in file.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            match char {
                '^' => starting_position = [i as i16, j as i16],
                '#' => obstacles.push([i as i16, j as i16]),
                _ => (),
            }
            if j > max_j {
                max_j = j
            };
        }
        if i > max_i {
            max_i = i
        };
    }

    InputData {
        starting_position,
        obstacles,
        width: max_j as i16,
        height: max_i as i16,
    }
}

#[derive(Debug)]
struct InputData {
    starting_position: [i16; 2],
    obstacles: Vec<[i16; 2]>,
    width: i16,
    height: i16,
}
