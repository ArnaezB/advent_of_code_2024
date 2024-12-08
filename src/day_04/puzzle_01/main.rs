use std::fs;

fn main() {
    let directions = [[0, 1], [0, -1], [1, 0], [-1, 0], [1, 1], [1, -1], [-1, 1], [-1, -1]];
    let matrix = file_to_2d_vec();
    let width = matrix.len();
    let mut result = 0;

    for (i, c) in matrix.iter().flatten().enumerate() {
        let (row, col) = (i / width, i % width);
        if *c == 'X' {
            for direction in directions {
                if find_xmas(&direction, &matrix, row, col) {
                    result += 1
                }
            }
        }
    }

    print!("{result}")
}

fn file_to_2d_vec() -> Vec<Vec<char>> {
    let file = fs::read_to_string("src/day_04/input").unwrap();
    let mut matrix: Vec<Vec<char>> = vec!();

    for line in file.lines() {
        matrix.push(
            line.split("")
                .filter(|x| { *x != "" })
                .map(|x| { x.chars().take(1).next().unwrap() })
                .collect::<Vec<char>>()
        )
    }

    matrix
}

fn find_xmas(direction: &[i32; 2], matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let chars = ['M', 'A', 'S'];
    let mut matched = 0;

    loop {
        let (next_x, next_y) = (i as i32 + direction[0] * (matched + 1), j as i32 + direction[1] * (matched + 1));
        if next_x < 0 || next_y < 0 || next_x >= matrix.len() as i32 || next_y >= matrix[0].len() as i32 {
            return false;
        }

        if matrix[next_x as usize][next_y as usize] == chars[matched as usize] {
            matched += 1;
        } else {
            return false;
        }

        if matched == 3 {
            return true;
        }
    }
}