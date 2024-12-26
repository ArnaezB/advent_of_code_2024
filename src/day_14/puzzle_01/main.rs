use std::usize;

const HEIGHT: usize = 103;
const WIDTH: usize = 101;
const ITERS: usize = 100;

// const HEIGHT: usize = 7;
// const WIDTH: usize = 11;

fn main() {
    let mut robots = parse_file();

    robots.iter_mut().for_each(|r| r.pass(ITERS));

    let grid = robots_to_matrix(robots);

    let safety_scores = get_safety_score_by_quadrant(&grid);

    let result = safety_scores.iter().fold(1, |acc, e| acc * e);

    println!("{}", result)
}

fn robots_to_matrix(robots: Vec<Robot>) -> Grid {
    let mut matrix: Grid = [0; WIDTH * HEIGHT];

    for Robot { position, .. } in robots {
        matrix[position[0] * WIDTH + position[1]] += 1;
    }

    matrix
}

fn get_safety_score_by_quadrant(matrix: &Grid) -> [usize; 4] {
    let mut quadrant_one = vec![];
    for i in 0..HEIGHT / 2 {
        quadrant_one.push(&matrix[i * WIDTH..WIDTH / 2 + i * WIDTH])
    }

    let mut quadrant_two = vec![];
    for i in 0..HEIGHT / 2 {
        quadrant_two.push(
            &matrix[((HEIGHT / 2 + 1) * WIDTH + i * WIDTH)
                ..((HEIGHT / 2 + 1) * WIDTH + WIDTH / 2 + i * WIDTH)],
        )
    }

    let mut quadrant_three = vec![];
    for i in 0..HEIGHT / 2 {
        quadrant_three
            .push(&matrix[i * WIDTH + WIDTH / 2 + 1..WIDTH / 2 + i * WIDTH + WIDTH / 2 + 1])
    }

    let mut quadrant_four = vec![];
    for i in 0..HEIGHT / 2 {
        quadrant_four.push(
            &matrix[(HEIGHT / 2 + 1) * WIDTH + WIDTH / 2 + 1 + i * WIDTH
                ..WIDTH / 2 + (HEIGHT / 2 + 1) * WIDTH + WIDTH / 2 + 1 + i * WIDTH],
        )
    }

    [
        count_present(quadrant_one),
        count_present(quadrant_two),
        count_present(quadrant_three),
        count_present(quadrant_four),
    ]
}

fn count_present(quadrant: Vec<&[usize]>) -> usize {
    let mut count = 0;

    for row in quadrant {
        for e in row {
            count += e;
        }
    }

    count
}

fn parse_file() -> Vec<Robot> {
    let mut robots = vec![];
    let input = include_str!("../input");

    for line in input.lines() {
        let data = line.split(' ').collect::<Vec<&str>>();

        let position = extract_values(data[0])
            .iter()
            .map(|x| usize::try_from(*x).unwrap())
            .collect::<Vec<usize>>();
        let velocity = extract_values(data[1]);

        robots.push(Robot {
            position: [position[0], position[1]],
            velocity,
        });
    }

    robots
}

fn extract_values(params: &str) -> [i32; 2] {
    let comma = params.find(',').unwrap();

    let y = params[2..comma].parse::<i32>().unwrap();
    let x = params[comma + 1..params.len()].parse::<i32>().unwrap();

    [x, y]
}

#[derive(Debug)]
struct Robot {
    position: [usize; 2],
    velocity: [i32; 2],
}

impl Robot {
    fn next(&mut self) {
        self.position = [
            i32::rem_euclid(self.position[0] as i32 + self.velocity[0], HEIGHT as i32) as usize,
            i32::rem_euclid(self.position[1] as i32 + self.velocity[1], WIDTH as i32) as usize,
        ];
    }

    fn pass(&mut self, time: usize) {
        self.position = [
            i32::rem_euclid(
                self.position[0] as i32 + time as i32 * self.velocity[0],
                HEIGHT as i32,
            ) as usize,
            i32::rem_euclid(
                self.position[1] as i32 + time as i32 * self.velocity[1],
                WIDTH as i32,
            ) as usize,
        ];
    }
}

type Grid = [usize; WIDTH * HEIGHT];