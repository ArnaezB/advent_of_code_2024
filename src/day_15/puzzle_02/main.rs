use crate::WarehouseItem::{Blockade, Robot};
use std::cmp::{Ordering, PartialEq};
use std::collections::HashSet;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::time::Instant;

fn main() {
    let (mut warehouse, movements) = parse_file();

    let start = Instant::now();
    for movement in movements {
        warehouse.move_robot(movement);
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
                robot = Some([i, y * 2])
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

fn pretty_print(matrix: &Vec<Vec<char>>) {
    for line in matrix {
        for item in line {
            print!("{}", item)
        }
        println!();
    }

    println!()
}

fn pretty_log(matrix: &Vec<Vec<char>>, file: &mut File, direction: &Direction) {
    for row in matrix {
        for c in row {
            write!(file, "{}", c).unwrap();
        }
        writeln!(file).unwrap();
    }

    writeln!(file).unwrap();
    writeln!(file, "Next: {:?}", direction).unwrap();
}

fn parse_matrix_line(matrix: &mut Vec<Vec<char>>, line: &str) {
    let mut tmp = vec![];
    for c in line.chars() {
        let new_c = match c {
            '#' => String::from("##"),
            'O' => String::from("[]"),
            x => format!("{x}."),
        };
        tmp.extend(new_c.chars())
    }

    matrix.push(tmp)
}

fn parse_instruction_line(movements: &mut Movements, line: &str) {
    movements.extend(line.chars().map(|c| Direction::try_from(c).unwrap()))
}

#[derive(Debug)]
struct Warehouse {
    matrix: Vec<Vec<char>>,
    robot: [usize; 2],
}

#[derive(Debug, PartialEq)]
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
                if direction == Direction::Left || direction == Direction::Right {
                    self.push_box_horizontally(self.get_box_at(next_pos).unwrap(), direction);
                } else {
                    self.push_box_vertically(self.get_box_at(next_pos).unwrap(), direction)
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

    fn push_box_horizontally(
        &mut self,
        box_coords: ([usize; 2], [usize; 2]),
        direction: Direction,
    ) {
        match self.find_free_space(box_coords.0, &direction) {
            Some(free_coords) => {
                let row = &mut self.matrix[box_coords.0[0]];
                if direction == Direction::Left {
                    let mut slice = &mut row[free_coords[1]..box_coords.1[1] + 2];
                    slice.rotate_left(1);
                } else {
                    let mut slice = &mut row[box_coords.0[1] - 1..free_coords[1] + 1];
                    slice.rotate_right(1);
                };
                self.robot = [self.robot[0], row.iter().position(|c| *c == '@').unwrap()]
            }
            _ => (),
        }
    }

    fn push_box_vertically(&mut self, box_coords: ([usize; 2], [usize; 2]), direction: Direction) {
        let direction_coords = direction.to_array();

        let mut boxes = self.get_boxes_pushed_by_vertically(box_coords.0, &direction_coords);
        boxes.extend(self.get_boxes_pushed_by_vertically(box_coords.1, &direction_coords));

        let mut boxes = Vec::from_iter(boxes.iter());

        boxes.sort_by(|&&a, &b| match a[0].cmp(&b[0]) {
            Ordering::Equal => a[1].cmp(&b[1]),
            o => o,
        });

        if direction == Direction::Up {
            boxes.iter().for_each(|[x, y]| {
                self.matrix[(*x as i32 + direction_coords[0]) as usize][*y] = self.matrix[*x][*y];
                self.matrix[*x][*y] = '.';
            });
        } else {
            boxes.iter().rev().for_each(|[x, y]| {
                self.matrix[(*x as i32 + direction_coords[0]) as usize][*y] = self.matrix[*x][*y];
                self.matrix[*x][*y] = '.';
            });
        }

        if boxes.len() > 0 {
            self.matrix[self.robot[0]][self.robot[1]] = '.';
            self.robot = [
                (self.robot[0] as i32 + direction_coords[0]) as usize,
                self.robot[1],
            ];
            self.matrix[self.robot[0]][self.robot[1]] = '@';
        }
    }

    fn get_boxes_pushed_by_vertically(
        &self,
        coords: [usize; 2],
        direction: &[i32; 2],
    ) -> HashSet<[usize; 2]> {
        let mut boxes = HashSet::new();

        if let Err(()) = self._gbpbv(coords, direction, &mut boxes) {
            HashSet::new()
        } else {
            boxes
        }
    }

    fn _gbpbv(
        &self,
        coords: [usize; 2],
        direction: &[i32; 2],
        set: &mut HashSet<[usize; 2]>,
    ) -> Result<(), ()> {
        match self.get_item_at(coords) {
            Some(WarehouseItem::Box) => {
                let box_coords = self.get_box_at(coords)?;
                self._gbpbv(
                    [
                        (box_coords.0[0] as i32 + direction[0]) as usize,
                        box_coords.0[1],
                    ],
                    direction,
                    set,
                )?;
                self._gbpbv(
                    [
                        (box_coords.1[0] as i32 + direction[0]) as usize,
                        box_coords.1[1],
                    ],
                    direction,
                    set,
                )?;
                set.insert(box_coords.0);
                set.insert(box_coords.1);
                Ok(())
            }
            Some(Blockade) => Err(()),
            _ => Ok(()),
        }
    }

    fn get_box_at(&self, position: [usize; 2]) -> Result<([usize; 2], [usize; 2]), ()> {
        match self.matrix[position[0]][position[1]] {
            '[' => Ok((position, [position[0], position[1] + 1])),
            ']' => Ok(([position[0], position[1] - 1], position)),
            _ => Err(()),
        }
    }

    fn get_item_at(&self, position: [usize; 2]) -> Option<WarehouseItem> {
        match WarehouseItem::try_from(self.matrix[position[0]][position[1]]) {
            Ok(wi) => Some(wi),
            Err(()) => None,
        }
    }

    fn find_free_space(&self, position: [usize; 2], direction: &Direction) -> Option<[usize; 2]> {
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

        for (c, i) in self.matrix.iter().flatten().zip(0u64..) {
            if *c == '[' {
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
            '[' | ']' => Ok(WarehouseItem::Box),
            '@' => Ok(Robot),
            _ => Err(()),
        }
    }
}

type Movements = Vec<Direction>;