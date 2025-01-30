use std::time::Instant;

const MIN_SAVE: i32 = 100;
const CHEAT_SIZE: i32 = 20;
fn main() {
    let (start_pos, mut racetrack) = parse_file();

    let start = Instant::now();

    populate_distances(start_pos, &mut racetrack);
    let n_cheats = find_cheats(start_pos, &racetrack);

    println!("{} in {}ms", n_cheats, start.elapsed().as_millis());
}

fn find_cheats(start: [usize; 2], racetrack: &Vec<Vec<i32>>) -> i32 {
    let mut cheats = 0;
    let mut current = start;
    cheats += reachable_cell_and_valid(current, racetrack);

    while let Some(next) = next2(current, racetrack) {
        cheats += reachable_cell_and_valid(next, racetrack);
        current = next;
    }
    cheats
}

fn reachable_cell_and_valid(center: [usize; 2], racetrack: &Vec<Vec<i32>>) -> i32 {
    let mut nodes = 0;

    let cost = racetrack[center[0]][center[1]];
    let center = [center[0] as i32, center[1] as i32];

    for it in 1..=CHEAT_SIZE {
        for j in 0..=it {
            let a = [center[0] - it + j, center[1] + j];
            let b = [center[0] + center[0] - a[0], center[1] - j];

            if j > 0 && j < it {
                let c = [center[0] + it - j, a[1]];
                let d = [a[0], center[1] - j];

                if valid_position(&c, racetrack, cost + it) {
                    nodes += 1
                }

                if valid_position(&d, racetrack, cost + it) {
                    nodes += 1
                }
            }

            if valid_position(&a, racetrack, cost + it) {
                nodes += 1
            }

            if valid_position(&b, racetrack, cost + it) {
                nodes += 1
            }
        }
    }

    nodes
}

fn valid_position(pos: &[i32; 2], racetrack: &Vec<Vec<i32>>, from_cost: i32) -> bool {
    let size: i32 = racetrack.len().try_into().unwrap();

    pos[0] > 0
        && pos[0] < size - 1
        && pos[1] > 0
        && pos[1] < size - 1
        && from_cost + MIN_SAVE <= racetrack[pos[0] as usize][pos[1] as usize]
}

fn populate_distances(start: [usize; 2], racetrack: &mut Vec<Vec<i32>>) {
    let mut current = start;
    let mut ms = 2;
    racetrack[start[0]][start[1]] = 1;

    while let Some([i, j]) = next(current, racetrack, start) {
        racetrack[i][j] = ms;
        ms += 1;
        current = [i, j];
    }
}

fn next2(node: [usize; 2], racetrack: &Vec<Vec<i32>>) -> Option<[usize; 2]> {
    let size = racetrack.len();
    let cost = racetrack[node[0]][node[1]];
    [[1, 0], [-1, 0], [0, 1], [0, -1]]
        .into_iter()
        .map(|d| {
            [
                (node[0] as i32 + d[0]) as usize,
                (node[1] as i32 + d[1]) as usize,
            ]
        })
        .filter(|&next| next[0] < size && next[1] < size && racetrack[next[0]][next[1]] == cost + 1)
        .next()
}

fn next(node: [usize; 2], racetrack: &Vec<Vec<i32>>, exclude: [usize; 2]) -> Option<[usize; 2]> {
    let size = racetrack.len();
    [[1, 0], [-1, 0], [0, 1], [0, -1]]
        .into_iter()
        .map(|d| {
            [
                (node[0] as i32 + d[0]) as usize,
                (node[1] as i32 + d[1]) as usize,
            ]
        })
        .filter(|&next| {
            next != exclude && next[0] < size && next[1] < size && racetrack[next[0]][next[1]] == 0
        })
        .next()
}

fn parse_file() -> ([usize; 2], Vec<Vec<i32>>) {
    let mut start = [0; 2];
    let input = include_str!("../input");
    let mut vec = vec![];

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            if c == '#' {
                row.push(-1);
            } else {
                row.push(0);
                if c == 'S' {
                    start = [i, j];
                }
            }
        }

        vec.push(row)
    }
    (start, vec)
}