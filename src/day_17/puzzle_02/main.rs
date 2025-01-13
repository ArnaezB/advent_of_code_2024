use itertools::Itertools;
use std::time::Instant;

fn main() {
    let mut a = 0u64;

    let mut input = parse_file();
    let mut queue = vec![];

    let start = Instant::now();
    queue.push(((input.tape.len() - 1) as i64, a));

    while let Some((index, value)) = queue.pop() {
        if index < 0 {
            a = value;
            break;
        }

        let target = input.tape[index as usize];

        for k in (0..=0b111).rev() {
            let a_candidate = (value << 3) + k;

            input.reg_a = a_candidate;
            let obtained = u8::try_from(input.debug().chars().nth(0).unwrap()).unwrap() - 48;
            if target == obtained {
                queue.push((index - 1, a_candidate))
            }
        }
    }

    println!("{} in {}ms", a, start.elapsed().as_millis())
}

fn parse_file() -> CpuState {
    let mut input = include_str!("../input").lines();

    let a = input.next().unwrap()[12..].parse::<u64>().unwrap();
    let b = input.next().unwrap()[12..].parse::<u64>().unwrap();
    let c = input.next().unwrap()[12..].parse::<u64>().unwrap();
    &input.next();
    let tape = input.next().unwrap()[9..]
        .chars()
        .filter(|c| *c != ',')
        .map(|c| u8::try_from(c).unwrap() - 48)
        .collect();

    CpuState::new(a, b, c, tape)
}

#[derive(Debug, Clone)]
struct CpuState {
    reg_a: u64,
    reg_b: u64,
    reg_c: u64,

    tape: Vec<u8>,

    ptr: u32,
    out: Vec<u8>,
}

impl CpuState {
    fn new(reg_a: u64, reg_b: u64, reg_c: u64, tape: Vec<u8>) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            tape,
            ptr: 0,
            out: vec![],
        }
    }

    fn reset(&mut self) {
        self.ptr = 0;
        self.out = vec![];
        self.reg_b = 0;
        self.reg_c = 0;
    }

    fn debug(&mut self) -> String {
        self.reset();

        while usize::try_from(self.ptr).unwrap() < self.tape.len() {
            let op_code = self.tape[self.ptr as usize];
            let operand = self.tape[(self.ptr + 1) as usize] as u64;
            let combo = match self.operand_combo(operand) {
                Ok(x) => x,
                Err(()) => return self.output(),
            };

            self.ptr += 2;

            match op_code {
                0 => self.adv(combo),
                1 => self.bxl(operand),
                2 => self.bst(combo),
                3 => self.jnz(operand),
                4 => self.bxc(),
                5 => self.out(combo),
                6 => self.bdv(combo),
                7 => self.cdv(combo),
                _ => return self.output(),
            }
        }

        self.output()
    }

    fn operand_combo(&self, operand: u64) -> Result<u64, ()> {
        match operand {
            x if x >= 0 && x <= 3 => Ok(x.into()),
            4 => Ok(self.reg_a),
            5 => Ok(self.reg_b),
            6 => Ok(self.reg_c),
            _ => Err(()),
        }
    }

    fn output(&self) -> String {
        self.out
            .iter()
            .map(|&u| char::try_from(u + 48).unwrap())
            .join(",")
    }

    fn adv(&mut self, combo: u64) {
        self.reg_a = self.reg_a >> combo
    }
    fn bxl(&mut self, literal: u64) {
        self.reg_b = self.reg_b ^ literal
    }
    fn bst(&mut self, combo: u64) {
        self.reg_b = combo & 0b111
    }
    fn jnz(&mut self, literal: u64) {
        if self.reg_a != 0 {
            self.ptr = literal as u32;
        }
    }
    fn bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_c
    }
    fn out(&mut self, combo: u64) {
        self.out.push((combo & 0b111) as u8)
    }
    fn bdv(&mut self, combo: u64) {
        self.reg_b = self.reg_a >> combo
    }
    fn cdv(&mut self, combo: u64) {
        self.reg_c = self.reg_a >> combo
    }
}