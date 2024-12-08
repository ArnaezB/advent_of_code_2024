use std::fs;

fn main() {
    let matrix = file_to_2d_vec();
    let width = matrix.len();
    let mut result = 0;

    for (i, c) in matrix.iter().flatten().enumerate() {
        let (row, col) = (i / width, i % width);
        if *c == 'A' && find_xmas(&matrix, row, col) {
            result += 1
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

fn find_xmas(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // [-1,-1] & [1,1]
    // [-1,1] & [1,-1]
    if i == 0 || i == matrix.len() - 1 || j == 0 || j == matrix.len() - 1 {
        return false;
    }

    let down_right = match (matrix[i - 1][j - 1], matrix[i + 1][j + 1]) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false
    };

    let up_left = match (matrix[i - 1][j + 1], matrix[i + 1][j - 1]) {
        ('M', 'S') => true,
        ('S', 'M') => true,
        _ => false
    };

    down_right && up_left
}