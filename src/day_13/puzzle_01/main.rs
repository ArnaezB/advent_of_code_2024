use itertools::Itertools;

fn main() {
    let instructions = parse_file();
    let mut result = 0;

    for instruction in instructions {
        let Instruction {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (r_x, r_y)
        } = instruction;

        // This ecuations have been obtained by solving a 2 by 2 matrix (ecuations system of dim 2)
        let a_n = ((a_x * r_y - a_y * r_x) as f64 / (b_y * a_x - b_x * a_y) as f64);
        if a_n % 1f64 != 0f64 { continue; }
        let a_n = a_n as i32;

        let b_n = ((r_x - a_n * b_x) as f64 / a_x as f64);
        if b_n % 1f64 != 0f64 { continue; }
        let b_n = b_n as i32;

        result += b_n * 3 + a_n;
    }

    println!("{result}")
}

fn parse_file() -> Vec<Instruction> {
    let input = include_str!("../input");
    let mut vec = vec![];

    for pack in &input.lines().chunks(4) {
        let pack = pack.take(3).collect::<Vec<&str>>();

        let button_a = parse_instructions(pack[0]);
        let button_b = parse_instructions(pack[1]);
        let prize = parse_end(pack[2]);

        println!("Read: {:?} {:?} {:?}", button_a, button_b, prize);

        vec.push(Instruction { button_a, button_b, prize })
    }

    vec
}

fn parse_instructions(instruction: &str) -> (i32, i32) {
    parse_parametrized(instruction, 12, 3)
}

fn parse_end(instruction: &str) -> (i32, i32) {
    parse_parametrized(instruction, 9, 4)
}

fn parse_parametrized(instruction: &str, start_first: usize, space: usize) -> (i32, i32) {
    let comma = instruction.find(',').unwrap();
    let x = instruction[start_first..comma].parse::<i32>().unwrap();
    let y = instruction[comma + space..].parse::<i32>().unwrap();

    (x, y)
}

struct Instruction {
    button_a: (i32, i32),
    button_b: (i32, i32),
    prize: (i32, i32),
}