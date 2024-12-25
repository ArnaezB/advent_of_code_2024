use std::usize;

fn main() {
    let mut garden = parse_file();
    let mut result = 0;

    for x in 0..usize::pow(garden.len(), 2) {
        let pos = [x % garden.len(), x / garden.len()];
        let plant = &garden[pos[0]][pos[1]];

        if plant.visited {
            continue;
        }
        // println!("Region {}: ", plant.plant);

        let (perim, area) = find_perimeter_and_area(pos, &mut garden);
        result += perim * area;
        // println!("Result: P{} A{}", perim, area);
    }

    print!("{}", result)
}

fn find_perimeter_and_area(start: [usize; 2], garden: &mut Garden) -> (i32, i32) {
    let dirs: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    let mut perimeter = 0;
    let mut area = 1;
    garden[start[0]][start[1]].visited = true;

    for dir in dirs {
        let next = [start[0] as i32 + dir[0], start[1] as i32 + dir[1]];
        if let Some((p, a)) =
            _find_perimeter_and_area(next, garden, &garden[start[0]][start[1]].clone())
        {
            perimeter += p;
            area += a;
        }
    }

    (perimeter, area)
}

fn _find_perimeter_and_area(
    start: [i32; 2],
    garden: &mut Garden,
    target: &GardenPlot,
) -> Option<(i32, i32)> {
    let dirs: [[i32; 2]; 4] = [[1, 0], [0, 1], [-1, 0], [0, -1]];
    let mut perimeter = 0;
    let mut area = 1;

    if start[0] < 0
        || start[1] < 0
        || start[0] >= garden.len() as i32
        || start[1] >= garden.len() as i32
    {
        // println!("At {}{}. Out", start[0], start[1]);
        return Some((1, 0));
    };

    let start = [start[0] as usize, start[1] as usize];

    let mut plot = &mut garden[start[0]][start[1]];

    if plot.plant != target.plant {
        // println!("At {}{} = {}. Other", start[0], start[1], plot.plant);
        return Some((1, 0));
    }

    if plot.visited {
        // println!("At {}{} = {}. Visited", start[0], start[1], plot.plant);
        return None;
    }
    // println!("At {}{} = {}. Valid", start[0], start[1], plot.plant);

    plot.visit();

    for dir in dirs {
        let next = [start[0] as i32 + dir[0], start[1] as i32 + dir[1]];
        // println!("Going {:?}", next);
        match _find_perimeter_and_area(next, garden, target) {
            None => (),
            Some((p, a)) => {
                perimeter += p;
                area += a;
            }
        }
    }

    Some((perimeter, area))
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