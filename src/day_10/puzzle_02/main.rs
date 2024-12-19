fn main() {
    let Map {
        grid,
        starting_positions,
    } = parse_file();
    let mut result = 0;

    for pos in starting_positions {
        result += calc_trailhead_score(&grid, pos).len();
    }

    println!("{:?}", result)
}

fn calc_trailhead_score(matrix: &Vec<Vec<u32>>, start: [i32; 2]) -> Vec<[i32; 2]> {
    _calc_trailhead_score(matrix, start, 0)
}

fn _calc_trailhead_score(matrix: &Vec<Vec<u32>>, at: [i32; 2], want_value: u32) -> Vec<[i32; 2]> {
    let dirs: [[i32; 2]; 4] = [[-1, 0], [1, 0], [0, 1], [0, -1]];

    if at[0] < 0 || at[0] >= matrix.len() as i32 || at[1] < 0 || at[1] >= matrix.len() as i32 {
        return vec![];
    }
    match matrix[at[0] as usize][at[1] as usize] {
        v if v != want_value => return vec![],
        9 => return vec![at],
        _ => (),
    }

    let mut result = vec![];

    for dir in dirs {
        result.extend(
            _calc_trailhead_score(matrix, [at[0] + dir[0], at[1] + dir[1]], want_value + 1).iter(),
        );
    }

    result
}

fn parse_file() -> Map {
    let input = include_str!("../input");
    let mut starting_positions = vec![];
    let mut grid = vec![];

    for (i, line) in input.lines().enumerate() {
        grid.push(vec![]);

        for (j, c) in line.chars().enumerate() {
            let digit = c.to_digit(10).unwrap();
            grid[i].push(digit);
            if digit == 0 {
                starting_positions.push([i as i32, j as i32])
            }
        }
    }

    Map {
        grid,
        starting_positions,
    }
}

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<u32>>,
    starting_positions: Vec<[i32; 2]>,
}