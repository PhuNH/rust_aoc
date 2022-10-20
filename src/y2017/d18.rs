use std::collections::{HashMap, VecDeque};

enum Value<'a> {
    Number(i64),
    Register(&'a str),
}

impl<'a> Value<'a> {
    fn from(s: &str) -> Value {
        match s.parse::<i64>() {
            Ok(number) => Value::Number(number),
            Err(_) => Value::Register(s)
        }
    }

    fn get(&self, registers: &mut HashMap<&'a str, i64>) -> i64 {
        match self {
            Value::Number(n) => *n,
            Value::Register(c) => *registers.entry(c).or_insert(0)
        }
    }
}

enum Instruction<'a> {
    Snd(Value<'a>),
    Set(&'a str, Value<'a>),
    Add(&'a str, Value<'a>),
    Mul(&'a str, Value<'a>),
    Mod(&'a str, Value<'a>),
    Rcv(Value<'a>),
    Jgz(Value<'a>, Value<'a>),
}

fn process_input(lines: Vec<&str>) -> Vec<Instruction> {
    lines.into_iter().map(|l| {
        let words: Vec<_> = l.split_whitespace().collect();
        match words[0] {
            "snd" => Instruction::Snd(Value::from(words[1])),
            "set" => Instruction::Set(words[1], Value::from(words[2])),
            "add" => Instruction::Add(words[1], Value::from(words[2])),
            "mul" => Instruction::Mul(words[1], Value::from(words[2])),
            "mod" => Instruction::Mod(words[1], Value::from(words[2])),
            "rcv" => Instruction::Rcv(Value::from(words[1])),
            _ => Instruction::Jgz(Value::from(words[1]), Value::from(words[2])),
        }
    }).collect()
}

pub fn main() {
    let input = aoc::read_input(module_path!());
    let instructions = process_input(input.lines().collect());
    one(&instructions);
    two(&instructions);
}

fn run_program<'a>(instructions: &Vec<Instruction<'a>>, registers: &mut HashMap<&'a str, i64>, stop_at: u32) {
    let mut count = 0;
    loop {
        if count == stop_at { break; }

        let ip = *registers.get("ip").unwrap();
        if ip < 0 || ip as usize >= instructions.len() { break; }

        let mut dist = 1;
        match &instructions[ip as usize] {
            Instruction::Snd(x) => {
                let value = x.get(registers);
                registers.insert("snd", value);
            }
            Instruction::Set(c, y) => {
                let value = y.get(registers);
                registers.insert(c, value);
            }
            Instruction::Add(c, y) => {
                *registers.entry(c).or_insert(0) += y.get(registers);
            }
            Instruction::Mul(c, y) => {
                *registers.entry(c).or_insert(0) *= y.get(registers);
            }
            Instruction::Mod(c, y) => {
                *registers.entry(c).or_insert(0) %= y.get(registers);
            }
            Instruction::Rcv(x) => {
                if x.get(registers) != 0 {
                    if count + 1 == stop_at {
                        println!("{}", registers.get("snd").unwrap());
                    }
                    count += 1;
                }
            }
            Instruction::Jgz(x, y) => {
                if x.get(registers) > 0 {
                    dist = y.get(registers);
                }
            }
        }
        registers.insert("ip", ip + dist);
    }
}

fn one(instructions: &Vec<Instruction>) {
    let mut registers = HashMap::new();
    registers.insert("ip", 0);
    registers.insert("snd", 0);
    run_program(instructions, &mut registers, 1);
}

struct Program<'a> {
    instructions: &'a Vec<Instruction<'a>>,
    registers: HashMap<&'a str, i64>,
    state: u32,
    waiting: Option<&'a str>,
    queue: VecDeque<i64>,
    count: u32,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a Vec<Instruction<'a>>, id: i64) -> Program<'a> {
        let mut registers = HashMap::new();
        registers.insert("ip", 0);
        registers.insert("p", id);
        Program { instructions, registers, state: 0, waiting: None, queue: VecDeque::new(), count: 0 }
    }

    fn run(&mut self, other: &mut Program) {
        self.state = 1;
        loop {
            let ip = *self.registers.get("ip").unwrap();
            if ip < 0 || ip as usize >= self.instructions.len() { break; }

            let mut dist = 1;
            match &self.instructions[ip as usize] {
                Instruction::Snd(x) => {
                    let value = x.get(&mut self.registers);
                    self.queue.push_back(value);
                    self.count += 1;
                }
                Instruction::Set(c, y) => {
                    let value = y.get(&mut self.registers);
                    self.registers.insert(c, value);
                }
                Instruction::Add(c, y) => {
                    *self.registers.entry(c).or_insert(0) += y.get(&mut self.registers);
                }
                Instruction::Mul(c, y) => {
                    *self.registers.entry(c).or_insert(0) *= y.get(&mut self.registers);
                }
                Instruction::Mod(c, y) => {
                    *self.registers.entry(c).or_insert(0) %= y.get(&mut self.registers);
                }
                Instruction::Rcv(x) => {
                    if let Value::Register(r) = x {
                        if !other.queue.is_empty() {
                            let received = other.queue.pop_front().unwrap();
                            self.registers.insert(r, received);
                            self.waiting = None;
                        } else {
                            if other.state == 2 {
                                self.waiting = None;
                                self.state = 2;
                            } else if other.state == 0 || !self.queue.is_empty() {
                                self.waiting = Some(r);
                            } else {
                                self.waiting = None;
                                self.state = 2;
                                other.waiting = None;
                                other.state = 2;
                            }
                            break;
                        }
                    }
                }
                Instruction::Jgz(x, y) => {
                    if x.get(&mut self.registers) > 0 {
                        dist = y.get(&mut self.registers);
                    }
                }
            }
            self.registers.insert("ip", ip + dist);
        }
    }
}

fn two(instructions: &Vec<Instruction>) {
    let mut p_0 = Program::new(instructions, 0);
    let mut p_1 = Program::new(instructions, 1);
    loop {
        p_0.run(&mut p_1);
        p_1.run(&mut p_0);
        if p_0.state == 2 && p_1.state == 2 { break; }
    }
    println!("{}", p_1.count);
}