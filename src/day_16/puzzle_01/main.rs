use hashbrown::hash_map::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use std::cmp::{PartialEq, Reverse};
use std::hash::Hash;
use std::time::Instant;

const STEP_COST: u32 = 1;
const TURN_STEP_COST: u32 = 1000 + STEP_COST;

fn main() {
    let labyrinth = parse_file();

    let start = Instant::now();
    let result = labyrinth.a_star();
    let elapsed = start.elapsed().as_millis();

    println!("{:?} in {:?}ms", result, elapsed);
}

fn parse_file() -> Labyrinth {
    let input = include_str!("../input");
    let mut labyrinth: Vec<Vec<bool>> = vec![];

    for line in input.lines() {
        let mut row = vec![];

        line.chars().for_each(|c| match c {
            '#' => row.push(false),
            _ => row.push(true),
        });

        labyrinth.push(row);
    }

    Labyrinth {
        start: [labyrinth.len() - 2, 1],
        end: [1, labyrinth[0].len() - 2],
        labyrinth,
    }
}

struct Labyrinth {
    start: [usize; 2],
    end: [usize; 2],
    labyrinth: Vec<Vec<bool>>,
}

impl Labyrinth {
     fn a_star(&self) -> Result<u32, ()> {
        let mut distances = self.initialize_3d_matrix(u32::MAX);
        let mut seen = self.initialize_3d_matrix(false);
        let mut pq: PriorityQueue<_, _, DefaultHashBuilder> = PriorityQueue::with_default_hasher();

        distances[self.coords_dir_to_index(&self.start, &Direction::Right)] = 0;
        pq.push(Node::new(self.start, Direction::Right), Reverse(0u32));

        while let Some((node, _)) = pq.pop() {
            let Node {
                coords: current_coords,
                direction: current_direction,
            } = node;

            let current_index = self.coords_dir_to_index(&current_coords, &current_direction);

            if current_coords == self.end {
                return Ok(distances[current_index]);
            }

            let current_index = self.coords_dir_to_index(&current_coords, &current_direction);
            seen[current_index] = true;

            for (neighbor_direction, neighbor_coords) in self.neighbours_of(&current_coords) {
                let neighbor_index = self.coords_dir_to_index(&neighbor_coords, &neighbor_direction);

                if seen[neighbor_index] { continue }
                let tentative_distance = distances[current_index] + Self::distance(&current_direction, &neighbor_direction);

                if tentative_distance < distances[neighbor_index] {
                    distances[neighbor_index] = tentative_distance;

                    pq.push(
                        Node::new(neighbor_coords, neighbor_direction),
                        Reverse(tentative_distance + Self::manhattan_distance(&neighbor_coords, &self.end)),
                    );
                }
            }
        }

        Err(())
    }

    fn initialize_3d_matrix<T: Copy>(&self, default: T) -> Vec<T> {
        vec![default; self.labyrinth.len() * self.labyrinth[0].len() * 4]
    }

    fn coords_dir_to_index(&self, coords: &[usize; 2], direction: &Direction) -> usize {
        coords[0] * self.labyrinth[0].len() * 4 + coords[1] * 4 + *direction as usize
    }

    fn distance(from: &Direction, to: &Direction) -> u32 {
        if from == to {
            STEP_COST
        } else {
            TURN_STEP_COST
        }
    }

    fn manhattan_distance(from: &[usize;2], to: &[usize;2]) -> u32 {
        (from[0].abs_diff(to[0]) + from[1].abs_diff(to[1])).try_into().unwrap()
    }

    fn neighbours_of(&self, coords: &[usize; 2]) -> Vec<(Direction, [usize; 2])> {
        [
            (Direction::Up, [-1, 0]),
            (Direction::Right, [0, 1]),
            (Direction::Down, [1, 0]),
            (Direction::Left, [0, -1]),
        ]
        .map(|(d, c)| {
            (
                d,
                [
                    (coords[0] as i32 + c[0]) as usize,
                    (coords[1] as i32 + c[1]) as usize,
                ],
            )
        })
        .into_iter()
        .filter(|(_, [x, y])| self.labyrinth[*x][*y])
        .collect()
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Node {
    coords: [usize; 2],
    direction: Direction,
}

impl Node {
    fn new(coords: [usize; 2], direction: Direction) -> Self {
        Self { coords, direction }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}