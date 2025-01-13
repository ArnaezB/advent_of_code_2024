use hashbrown::hash_map::DefaultHashBuilder;
use hashbrown::HashSet;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::time::Instant;

const SIZE: usize = 71;
const FALLS: u32 = 1024;

fn main() {
    let mut memory_state = parse_file();

    let start = Instant::now();
    memory_state.simulate_n_falls(FALLS);

    let cost = memory_state.cost_of_minimal_path_to_exit();

    println!("{} in {}ms", cost.unwrap(), start.elapsed().as_millis())
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
        (0..n).for_each(|_| self.simulate_next_fall())
    }

    fn simulate_next_fall(&mut self) {
        if let Some(byte) = self.falling_bytes.pop() {
            self.memory[byte[0]][byte[1]] = false;
        }
    }

    fn cost_of_minimal_path_to_exit(&self) -> Result<u32, ()> {
        let mut pq: PriorityQueue<_, _, DefaultHashBuilder> = PriorityQueue::with_default_hasher();
        let mut closed = HashSet::new();
        let mut pq_set = HashSet::new();

        pq.push([0, 0], Reverse(0));
        pq_set.insert([0, 0]);

        while let Some((node, Reverse(cost))) = pq.pop() {
            if node == [SIZE - 1; 2] {
                return Ok(cost);
            }

            closed.insert(node);

            for neighbor in self.neighbors_of(node) {
                if closed.contains(&neighbor)
                    || pq_set.contains(&neighbor)
                    || !self.memory[neighbor[0]][neighbor[1]]
                {
                    continue;
                }

                pq.push(neighbor, Reverse(cost + 1));
                pq_set.insert(neighbor);
            }
        }

        Err(())
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