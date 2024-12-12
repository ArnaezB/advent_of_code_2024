use std::collections::HashSet;
use std::fs;
use std::hash::Hash;

fn main() {
    let data = parse_file();
    let directions: [[i16; 2]; 4] = [[-1, 0], [0, 1], [1, 0], [0, -1]];

    let steps = find_path_exit(
        &data.obstacles,
        data.starting_position,
        data.height,
        data.width,
        &directions,
        0,
    )
    .unwrap_or(01234567);

    println!("{steps}")
}

fn find_path_exit(
    obstacles: &Vec<[i16; 2]>,
    position: [i16; 2],
    height: i16,
    width: i16,
    directions: &[[i16; 2]; 4],
    mut i_dir: usize,
) -> Result<usize, ()> {
    let mut visited_pos: HashSet<VisitedPosition> = HashSet::new();
    let mut obstacles_loop: HashSet<VisitedPosition> = HashSet::new();

    let mut now_at = position;

    visited_pos.insert(VisitedPosition { pos: now_at });

    loop {
        let direction = directions[i_dir];
        let next = [now_at[0] + direction[0], now_at[1] + direction[1]];

        if next[0] < 0 || next[0] > height || next[1] < 0 || next[1] > width {
            return Ok(obstacles_loop.len());
        }

        if obstacles.contains(&next) {
            i_dir = (i_dir + 1) % directions.len();
            continue;
        }

        if !obstacles_loop.contains(&VisitedPosition { pos: next })
            && !visited_pos.contains(&{ VisitedPosition { pos: next } })
        {
            if let Err(()) =
                find_exit_extra_obstacle(obstacles, next, now_at, height, width, directions, i_dir)
            {
                // println!("Found loop with new obstacle at {:?}", next);
                obstacles_loop.insert(VisitedPosition { pos: next });
            }
        };

        now_at = next;
        visited_pos.insert(VisitedPosition { pos: now_at });
    }
}

fn find_exit_extra_obstacle(
    obstacles: &Vec<[i16; 2]>,
    extra_obstacle: [i16; 2],
    position: [i16; 2],
    height: i16,
    width: i16,
    directions: &[[i16; 2]; 4],
    mut i_dir: usize,
) -> Result<(), ()> {
    let mut visited_pos_dirs = HashSet::new();
    let mut now_at = position;

    visited_pos_dirs.insert(VisitedPositionWithDir {
        pos: position,
        dir: i_dir,
    });

    loop {
        let direction = directions[i_dir];

        let next = [now_at[0] + direction[0], now_at[1] + direction[1]];

        if next[0] < 0 || next[0] > height || next[1] < 0 || next[1] > width {
            return Ok(());
        }

        if next == extra_obstacle || obstacles.contains(&next) {
            i_dir = (i_dir + 1) % directions.len();
            continue;
        }

        now_at = next;
        if !visited_pos_dirs.insert(VisitedPositionWithDir {
            pos: now_at,
            dir: i_dir,
        }) {
            // println!("Looping at {:?} {:?}", now_at, directions[i_dir]);
            return Err(());
        }
    }
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

#[derive(Eq, Hash, PartialEq)]
struct VisitedPositionWithDir {
    pos: [i16; 2],
    dir: usize,
}

#[derive(Eq, Hash, PartialEq)]
struct VisitedPosition {
    pos: [i16; 2],
}
