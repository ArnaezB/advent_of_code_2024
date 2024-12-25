use std::cmp::PartialEq;
use std::collections::HashSet;

fn main() {
    let mut garden = parse_file();
    let mut result = 0;

    for i in 0..usize::pow(garden.len(), 2) {
        let x = (i / garden.len());
        let y = (i % garden.len());

        let plot = &garden[x][y];

        if plot.visited {
            continue;
        }

        let (sides, area) =
            find_sides_area_of_region(plot.plant.clone(), [x as i32, y as i32], &mut garden);
        result += sides * area;
    }

    print!("{}", result)
}

fn find_sides_area_of_region(target: char, coords: [i32; 2], garden: &mut Garden) -> (i32, i32) {
    _find_sides_area_of_region(coords, target, None, garden)
}

fn _find_sides_area_of_region(
    coords: [i32; 2],
    target: char,
    prev_dir: Option<&Direction>,
    garden: &mut Garden,
) -> (i32, i32) {
    if coords[0] < 0
        || coords[1] < 0
        || coords[0] >= garden.len() as i32
        || coords[1] >= garden.len() as i32
    {
        return (0, 0);
    }

    let mut plot = &mut garden[coords[0] as usize][coords[1] as usize];
    if plot.visited || plot.plant != target {
        return (0, 0);
    }

    plot.visited = true;

    let borders = get_borders_for_coords(coords, garden, &target);
    let n_inner_corners = get_inner_corners(&borders, garden, coords, target);

    let mut n_corners = get_corners(borders) + n_inner_corners;
    let mut total_area = 1;

    for (dir, name) in Direction::get_tuples() {
        if let Some(prev_dir) = prev_dir {
            if name == prev_dir.opposite() {
                continue;
            }
        };

        let next_coords = [coords[0] + dir[0], coords[1] + dir[1]];

        let (corners, area) = _find_sides_area_of_region(next_coords, target, Some(&name), garden);

        n_corners += corners;
        total_area += area;
    }

    (n_corners, total_area)
}

fn get_inner_corners(
    borders: &Vec<Direction>,
    garden: &Garden,
    coords: [i32; 2],
    target: char,
) -> i32 {
    let mut inverse_borders = get_inverse_borders(borders);

    if inverse_borders.len() == 1 {
        return 0;
    };
    inverse_borders.sort();
    let pairs = take_pairs(inverse_borders);

    let mut inner_corners = 0;

    for [first, second] in pairs {
        if first.is_contiguous_to(&second) {
            let diagonal = first.diagonal_coords_with(&second);

            let new_coords = [coords[0] + diagonal[0], coords[1] + diagonal[1]];

            if new_coords[0] < 0
                || new_coords[1] < 0
                || new_coords[0] >= garden.len() as i32
                || new_coords[1] >= garden.len() as i32
            {
                continue;
            }

            if garden[new_coords[0] as usize][new_coords[1] as usize].plant != target {
                inner_corners += 1
            }
        }
    }
    inner_corners
}

fn take_pairs(vec: Vec<Direction>) -> Vec<[Direction; 2]> {
    if vec.len() == 2 {
        return vec![[vec[0], vec[1]]];
    }

    let mut pairs = vec![];
    let len = vec.len() as i32;

    for i in 0i32..len {
        let prev = i32::rem_euclid(i - 1, len) as usize;
        pairs.push([vec[i as usize], vec[prev]])
    }

    pairs
}

fn get_inverse_borders(borders: &Vec<Direction>) -> Vec<Direction> {
    let borders: HashSet<Direction> = borders.iter().cloned().collect();
    let all: HashSet<Direction> = Direction::all().iter().cloned().collect();

    (&all - &borders).iter().cloned().collect()
}

fn get_borders_for_coords(coords: [i32; 2], garden: &Garden, target: &char) -> Vec<Direction> {
    let mut borders = vec![];

    for (dir, name) in Direction::get_tuples() {
        let new_coords = [coords[0] + dir[0], coords[1] + dir[1]];

        if new_coords[0] < 0
            || new_coords[1] < 0
            || new_coords[0] >= garden.len() as i32
            || new_coords[1] >= garden.len() as i32
        {
            borders.push(name);
            continue;
        }

        let plot = &garden[new_coords[0] as usize][new_coords[1] as usize];

        if plot.plant != *target {
            borders.push(name);
        }
    }

    borders
}

fn get_corners(borders: Vec<Direction>) -> i32 {
    match borders.len() {
        4 => 4,
        3 => 2,
        2 => match borders[0].is_contiguous_to(&borders[1]) {
            true => 1,
            false => 0,
        },
        1 => 0,
        _ => 0,
    }
}

fn parse_file() -> Garden {
    let input = include_str!("../input");
    let mut garden = vec![];

    for line in input.lines() {
        let mut garden_row = vec![];

        for plant in line.chars() {
            garden_row.push(GardenPlot::new(plant));
        }

        garden.push(garden_row);
    }

    garden
}

#[derive(Clone)]
struct GardenPlot {
    plant: char,
    visited: bool,
}

impl GardenPlot {
    fn new(plant: char) -> Self {
        Self {
            plant,
            visited: false,
        }
    }

    fn visit(&mut self) {
        self.visited = true
    }
}

type Garden = Vec<Vec<GardenPlot>>;

#[derive(PartialEq, Debug, Eq, Hash, Clone, Ord, PartialOrd, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn is_contiguous_to(&self, other: &Direction) -> bool {
        if *self == *other {
            return false;
        }

        match (self, other) {
            (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => false,
            (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => false,
            _ => true,
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Right => Direction::Left,
        }
    }

    fn all() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Up,
            Direction::Left,
            Direction::Down,
        ]
    }

    fn get_tuples() -> [([i32; 2], Direction); 4] {
        [
            ([1, 0], Direction::Down),
            ([0, 1], Direction::Right),
            ([-1, 0], Direction::Up),
            ([0, -1], Direction::Left),
        ]
    }

    fn to_array(&self) -> [i32; 2] {
        match self {
            Direction::Up => [-1, 0],
            Direction::Down => [1, 0],
            Direction::Left => [0, -1],
            Direction::Right => [0, 1],
        }
    }

    fn diagonal_coords_with(&self, other: &Self) -> [i32; 2] {
        let coords_self = self.to_array();
        let coords_other = other.to_array();

        [
            coords_self[0] + coords_other[0],
            coords_self[1] + coords_other[1],
        ]
    }
}