use std::time::Instant;

const MIN_SAVE: i32 = 100;
fn main() {
    let (start_pos, mut racetrack) = parse_file();

    let start = Instant::now();

    populate_distances(start_pos, &mut racetrack);
    let n_cheats = find_cheats(start_pos, &racetrack);

    println!("{} in {}ms", n_cheats, start.elapsed().as_millis());
}

fn find_cheats(start: [usize; 2], racetrack: &Vec<Vec<i32>>) -> usize {
    let mut cheats = vec![];
    let mut current = start;

    while let Some(next) = next2(current, racetrack) {
        cheats.extend(two_ahead(current, racetrack));
        current = next;
    }

    cheats.into_iter().filter(|&x| x >= MIN_SAVE).count()
}

fn two_ahead(node: [usize; 2], racetrack: &Vec<Vec<i32>>) -> Vec<i32> {
    let close_coords: [[isize; 2]; 4] = [[-1, 0], [1, 0], [0, -1], [0, 1]];
    let far_coord: [[isize; 2]; 4] = [[-2, 0], [2, 0], [0, -2], [0, 2]];

    let size = racetrack.len().try_into().unwrap();
    let node_cost = racetrack[node[0]][node[1]];

    let mut cheats = vec![];

    for (c, f) in close_coords.iter().zip(far_coord) {
        let next_close = [node[0] as isize + c[0], node[1] as isize + c[1]];
        let next_far = [node[0] as isize + f[0], node[1] as isize + f[1]];

        if next_far[0] >= 0 && next_far[1] >= 0 && next_far[0] < size && next_far[1] < size {
            if racetrack[next_close[0] as usize][next_close[1] as usize] == -1
                && racetrack[next_far[0] as usize][next_far[1] as usize] != -1
            {
                let new_cost =
                    racetrack[next_far[0] as usize][next_far[1] as usize] - (node_cost + 2);
                if new_cost > 0 {
                    cheats.push(new_cost)
                }
            }
        }
    }

    cheats
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
        .filter(|&next| {
            next[0] >= 0
                && next[1] >= 0
                && next[0] < size
                && next[1] < size
                && racetrack[next[0]][next[1]] == cost + 1
        })
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
            next != exclude
                && next[0] >= 0
                && next[1] >= 0
                && next[0] < size
                && next[1] < size
                && racetrack[next[0]][next[1]] == 0
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