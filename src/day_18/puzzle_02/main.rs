use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashSet;
use itertools::Itertools;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::Instant;

const SIZE: usize = 71;
const FALLS: u32 = 1024;

fn main() {
    let mut memory_state = parse_file();

    let start = Instant::now();
    let mut result = None;
    let mut counter = 0;
    memory_state.simulate_n_falls(FALLS);

    loop {
        let path = match memory_state.cost_of_minimal_path_to_exit() {
            Ok(path) => path,
            Err(_) => break,
        };
        counter += 1;

        let last_block =
            memory_state.simulate_falls_until_block(HashSet::from_iter(path.into_iter()));

        result = last_block;
    }

    println!("Path recalculated {} times", counter);

    if let Some(mut byte) = result {
        println!(
            "{},{} in {}ms",
            byte[1],
            byte[0],
            start.elapsed().as_millis()
        )
    }
}

fn parse_file() -> CorruptedMemory {
    let mut falling_bytes = vec![];
    let input = include_str!("../input");

    for line in input.lines() {
        let values = line
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        falling_bytes.push([values[1], values[0]])
    }

    falling_bytes.reverse();

    CorruptedMemory::new(falling_bytes)
}

struct CorruptedMemory {
    falling_bytes: Vec<[usize; 2]>,
    memory: [[bool; SIZE]; SIZE],
}

impl CorruptedMemory {
    fn new(falling_bytes: Vec<[usize; 2]>) -> Self {
        Self {
            falling_bytes,
            memory: [[true; SIZE]; SIZE],
        }
    }
    fn simulate_n_falls(&mut self, n: u32) {
        (0..n).for_each(|_| {
            self.simulate_next_fall();
            ()
        })
    }

    fn simulate_next_fall(&mut self) -> Option<[usize; 2]> {
        if let Some(byte) = self.falling_bytes.pop() {
            self.memory[byte[0]][byte[1]] = false;
            return Some(byte);
        }
        None
    }

    fn simulate_falls_until_block(&mut self, blocks: HashSet<[usize; 2]>) -> Option<[usize; 2]> {
        loop {
            match self.falling_bytes.pop() {
                Some(byte) => {
                    self.memory[byte[0]][byte[1]] = false;

                    if blocks.contains(&byte) {
                        return Some(byte);
                    }
                }
                None => return None,
            }
        }
    }

    fn cost_of_minimal_path_to_exit(&self) -> Result<Vec<[usize; 2]>, ()> {
        let mut parents = [[None; SIZE]; SIZE];
        let mut pq: PriorityQueue<_, _, DefaultHashBuilder> = PriorityQueue::with_default_hasher();
        let mut closed = HashSet::new();
        let mut pq_set = HashSet::new();

        pq.push([0, 0], Reverse(0));
        pq_set.insert([0, 0]);

        while let Some((node, Reverse(cost))) = pq.pop() {
            if node == [SIZE - 1; 2] {
                return Ok(Self::path_from_parent_matrix(parents, node));
            }

            closed.insert(node);

            for neighbor in self.neighbors_of(node) {
                if closed.contains(&neighbor)
                    || pq_set.contains(&neighbor)
                    || !self.memory[neighbor[0]][neighbor[1]]
                {
                    continue;
                }

                parents[neighbor[0]][neighbor[1]] = Some(node);
                pq.push(neighbor, Reverse(cost + 1));
                pq_set.insert(neighbor);
            }
        }

        Err(())
    }

    fn path_from_parent_matrix(
        parents: [[Option<[usize; 2]>; SIZE]; SIZE],
        end: [usize; 2],
    ) -> Vec<[usize; 2]> {
        let mut path = vec![];
        let mut current = end;
        path.push(end);

        while let Some(parent) = parents[current[0]][current[1]] {
            path.push(parent);
            current = parent;
        }

        path.reverse();
        path
    }

    fn neighbors_of(&self, node: [usize; 2]) -> Vec<[usize; 2]> {
        [[1, 0], [-1, 0], [0, 1], [0, -1]]
            .iter()
            .map(|d| {
                [
                    (node[0] as i32 + d[0]) as usize,
                    (node[1] as i32 + d[1]) as usize,
                ]
            })
            .filter(|&neighbor| {
                neighbor[0] >= 0
                    && neighbor[1] >= 0
                    && neighbor[0] < SIZE
                    && neighbor[1] < SIZE
                    && self.memory[neighbor[0]][neighbor[1]]
            })
            .collect()
    }
}