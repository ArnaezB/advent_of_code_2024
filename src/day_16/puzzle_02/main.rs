use fxhash::FxHashSet;
use hashbrown::hash_map::DefaultHashBuilder;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::Instant;

const STEP_COST: u32 = 1;
const TURN_STEP_COST: u32 = 1000 + STEP_COST;

fn main() {
    let labyrinth = parse_file();

    let start = Instant::now();
    let dijkstra = labyrinth.dijkstra();
    let nodes_in_optimal_path = labyrinth.all_nodes_in_minimal_paths(&dijkstra.1);

    println!(
        "{} in {}ms",
        nodes_in_optimal_path,
        start.elapsed().as_millis()
    );
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
    fn pretty_print_paths(&self, paths: &Vec<Option<usize>>) {
        let mut counter = 0;

        for option in paths {
            if counter % 4 == 0 {
                print!("[")
            }
            print!(
                "{}",
                match option {
                    Some(x) => {
                        let c = self.index_to_coords(*x).0;
                        format!("[{},{}]", c[0], c[1])
                    }
                    None => String::from("."),
                }
            );
            counter += 1;
            if counter % 4 == 0 {
                print!("]");
            }
            if counter == self.labyrinth[0].len() * 4 {
                println!();
                counter = 0;
            }
        }
    }
    fn pretty_print_costs(&self, costs: &Vec<u32>) {
        let mut counter = 0;

        for c in costs {
            if counter % 4 == 0 {
                print!("[")
            }
            print!(
                "{} ",
                match *c {
                    u32::MAX => String::from("-"),
                    x => x.to_string(),
                }
            );
            counter += 1;

            if counter % 4 == 0 {
                print!("]");
            }

            if counter == self.labyrinth[0].len() * 4 {
                println!();
                counter = 0;
            }
        }
    }

    fn all_nodes_in_minimal_paths(&self, costs: &Vec<u32>) -> usize {
        let mut set = FxHashSet::default();
        let mut explorable_nodes: Vec<([usize; 2], u32, Direction)> = vec![];

        let (end_cost, end_direction) = (self.coords_dir_to_index(&self.end, &Direction::Up)..)
            .take(4)
            .enumerate()
            .map(|(it, e)| (costs[e], it))
            .min_by(|a, b| a.0.cmp(&b.0))
            .unwrap();

        explorable_nodes.push((
            self.end,
            end_cost,
            Direction::try_from(end_direction).unwrap(),
        ));
        set.insert(self.end);

        while let Some((current_coords, current_cost, current_dir)) = explorable_nodes.pop() {
            for (_, neighbor_coords) in self.neighbours_of(&current_coords) {
                let valid_neighbor_directions = self.valid_costs(
                    current_cost,
                    current_dir,
                    self.coords_dir_to_index(&neighbor_coords, &Direction::Up),
                    &costs,
                );

                if valid_neighbor_directions.len() > 0 {
                    set.insert(neighbor_coords);
                }
                explorable_nodes.extend(
                    valid_neighbor_directions
                        .iter()
                        .map(|(d, c)| (neighbor_coords, *c, *d)),
                );
            }
        }

        set.len()
    }

    fn valid_costs(
        &self,
        target: u32,
        target_direction: Direction,
        coords: usize,
        costs: &Vec<u32>,
    ) -> Vec<(Direction, u32)> {
        (coords..)
            .take(4)
            .enumerate()
            .map(|(it, e)| (Direction::try_from(it).unwrap(), costs[e]))
            .filter(|(it_dir, cost)| {
                *cost != u32::MAX && cost + Self::distance(&target_direction, it_dir) == target
            })
            .collect()
    }

    fn dijkstra(&self) -> (Vec<Option<usize>>, Vec<u32>) {
        let mut distances = self.initialize_3d_matrix(u32::MAX);
        let mut seen = self.initialize_3d_matrix(false);
        let mut paths = self.initialize_3d_matrix(None);
        let mut pq: PriorityQueue<_, _, DefaultHashBuilder> = PriorityQueue::with_default_hasher();

        distances[self.coords_dir_to_index(&self.start, &Direction::Right)] = 0;
        pq.push(Node::new(self.start, Direction::Right), Reverse(0u32));

        while let Some((node, _)) = pq.pop() {
            let Node {
                coords: current_coords,
                direction: current_direction,
            } = node;

            let current_index = self.coords_dir_to_index(&current_coords, &current_direction);
            seen[current_index] = true;

            for (neighbor_direction, neighbor_coords) in self.neighbours_of(&current_coords) {
                let neighbor_index =
                    self.coords_dir_to_index(&neighbor_coords, &neighbor_direction);
                if seen[neighbor_index] {
                    continue;
                }

                let new_distance_to_neighbor = distances[current_index]
                    + Self::distance(&current_direction, &neighbor_direction);

                if distances[neighbor_index] > new_distance_to_neighbor {
                    distances[neighbor_index] = new_distance_to_neighbor;
                    paths[neighbor_index] = Some(current_index);

                    pq.push(
                        Node::new(neighbor_coords, neighbor_direction),
                        Reverse(new_distance_to_neighbor),
                    );
                }
            }
        }

        (paths, distances)
    }

    fn initialize_3d_matrix<T: Copy>(&self, default: T) -> Vec<T> {
        vec![default; self.labyrinth.len() * self.labyrinth[0].len() * 4]
    }

    fn coords_dir_to_index(&self, coords: &[usize; 2], direction: &Direction) -> usize {
        coords[0] * self.labyrinth[0].len() * 4 + coords[1] * 4 + *direction as usize
    }

    fn index_to_coords(&self, index: usize) -> ([usize; 2], Direction) {
        let x = index / (self.labyrinth[0].len() * 4);
        let y = (index - x * self.labyrinth[0].len() * 4) / 4;
        let d = (index - x * self.labyrinth[0].len() * 4 - y * 4)
            .try_into()
            .unwrap();

        ([x, y], d)
    }

    fn distance(from: &Direction, to: &Direction) -> u32 {
        if from == to {
            STEP_COST
        } else {
            TURN_STEP_COST
        }
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

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }
}

impl TryFrom<usize> for Direction {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Direction::Up),
            1 => Ok(Direction::Right),
            2 => Ok(Direction::Down),
            3 => Ok(Direction::Left),
            _ => Err(()),
        }
    }
}