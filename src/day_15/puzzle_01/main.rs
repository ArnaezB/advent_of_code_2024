use crate::WarehouseItem::{Blockade, Robot};
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::time::Instant;

fn main() {
    let (mut warehouse, movements) = parse_file();

    let start = Instant::now();
    for mov in movements {
        warehouse.move_robot(mov);
    }

    let result = warehouse.gps_sum();
    println!("{} in {}ms", result, start.elapsed().as_millis());
}

fn parse_file() -> (Warehouse, Movements) {
    let input = include_str!("../input");
    let mut matrix = vec![];
    let mut robot = None;
    let mut movements = vec![];

    let mut is_parsing_matrix = true;

    for (i, line) in input.lines().enumerate() {
        if line.is_empty() {
            is_parsing_matrix = false;
            continue;
        }
        if is_parsing_matrix {
            parse_matrix_line(&mut matrix, line);
            if let Some(y) = line.find('@') {
                robot = Some([i, y])
            }
        } else {
            parse_instruction_line(&mut movements, line);
        }
    }

    (
        Warehouse {
            matrix,
            robot: robot.unwrap(),
        },
        movements,
    )
}

fn pretty_print(matrix: &Vec<Vec<char>>, file: &mut File) {
    for item in matrix {
        writeln!(file, "{:?}", item).unwrap();
    }
    writeln!(file).unwrap();
}

fn parse_matrix_line(matrix: &mut Vec<Vec<char>>, line: &str) {
    matrix.push(line.chars().collect::<Vec<char>>());
}

fn parse_instruction_line(movements: &mut Movements, line: &str) {
    movements.extend(line.chars().map(|c| Direction::try_from(c).unwrap()))
}

#[derive(Debug)]
struct Warehouse {
    matrix: Vec<Vec<char>>,
    robot: [usize; 2],
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum WarehouseItem {
    Box,
    Blockade,
    Robot,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::Up),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            '<' => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}

impl Warehouse {
    fn move_robot(&mut self, direction: Direction) {
        let direction_array = direction.to_array();
        let next_pos = [
            (self.robot[0] as i32 + direction_array[0]) as usize, // we dont need to check because of the nature of the input
            (self.robot[1] as i32 + direction_array[1]) as usize,
        ];

        match self.get_item_at(next_pos) {
            Some(Blockade) => (),
            Some(WarehouseItem::Box) => {
                if let Some(free_pos) = self.find_free_space(next_pos, direction) {
                    self.matrix[free_pos[0]][free_pos[1]] = 'O';
                    self.matrix[self.robot[0]][self.robot[1]] = '.';
                    self.matrix[next_pos[0]][next_pos[1]] = '@';
                    self.robot = next_pos;
                }
            }
            None => {
                self.matrix[self.robot[0]][self.robot[1]] = '.';
                self.robot = next_pos;
                self.matrix[self.robot[0]][self.robot[1]] = '@';
            }
            _ => (),
        }
    }

    fn get_item_at(&self, position: [usize; 2]) -> Option<WarehouseItem> {
        match WarehouseItem::try_from(self.matrix[position[0]][position[1]]) {
            Ok(wi) => Some(wi),
            Err(()) => None,
        }
    }

    fn find_free_space(&self, position: [usize; 2], direction: Direction) -> Option<[usize; 2]> {
        let mut check_position = position;
        let direction_array = direction.to_array();

        loop {
            let item = self.get_item_at(check_position);
            match item {
                Some(WarehouseItem::Box) => {
                    check_position = [
                        (check_position[0] as i32 + direction_array[0]) as usize,
                        (check_position[1] as i32 + direction_array[1]) as usize,
                    ]
                }
                None => return Some(check_position),
                _ => return None,
            }
        }
    }

    fn gps_sum(&self) -> u64 {
        let mut sum = 0;
        let width = self.matrix[0].len() as u64;

        for (e, i) in self.matrix.iter().flatten().zip(0u64..) {
            if let Ok(WarehouseItem::Box) = WarehouseItem::try_from(*e) {
                let x = i % width;
                let y = i / width;

                sum += 100 * y + x;
            }
        }

        sum
    }
}

impl Direction {
    fn to_array(&self) -> [i32; 2] {
        match self {
            Direction::Up => [-1, 0],
            Direction::Right => [0, 1],
            Direction::Down => [1, 0],
            Direction::Left => [0, -1],
        }
    }
}

impl Deref for Warehouse {
    type Target = Vec<Vec<char>>;

    fn deref(&self) -> &Self::Target {
        &self.matrix
    }
}

impl TryFrom<char> for WarehouseItem {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Blockade),
            'O' => Ok(WarehouseItem::Box),
            '@' => Ok(Robot),
            _ => Err(()),
        }
    }
}

type Movements = Vec<Direction>;