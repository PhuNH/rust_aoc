use std::collections::HashMap;

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
    Set(&'a str, Value<'a>),
    Sub(&'a str, Value<'a>),
    Mul(&'a str, Value<'a>),
    Jnz(Value<'a>, Value<'a>),
}

fn process_input(lines: Vec<&str>) -> Vec<Instruction> {
    lines.into_iter().map(|l| {
        let words: Vec<_> = l.split_whitespace().collect();
        match words[0] {
            "set" => Instruction::Set(words[1], Value::from(words[2])),
            "sub" => Instruction::Sub(words[1], Value::from(words[2])),
            "mul" => Instruction::Mul(words[1], Value::from(words[2])),
            _ => Instruction::Jnz(Value::from(words[1]), Value::from(words[2])),
        }
    }).collect()
}

fn main() {
    let input = aoc::read_input();
    let instructions = process_input(input.lines().collect());
    one(&instructions);
    two();
}

fn run_program<'a>(instructions: &Vec<Instruction<'a>>, registers: &mut HashMap<&'a str, i64>) -> usize {
    let mut count = 0;
    let mut c = 0;
    loop {
        let ip = *registers.get("ip").unwrap();
        if ip < 0 || ip as usize >= instructions.len() { break; }

        let mut dist = 1;
        match &instructions[ip as usize] {
            Instruction::Set(c, y) => {
                let value = y.get(registers);
                registers.insert(c, value);
            }
            Instruction::Sub(c, y) => {
                *registers.entry(c).or_insert(0) -= y.get(registers);
            }
            Instruction::Mul(c, y) => {
                *registers.entry(c).or_insert(0) *= y.get(registers);
                count += 1;
            }
            Instruction::Jnz(x, y) => {
                if x.get(registers) != 0 {
                    dist = y.get(registers);
                }
            }
        }
        registers.insert("ip", ip + dist);
        c += 1;
        if c < 1000 {
            println!("{:?}", registers);
        }
    }
    count
}

fn one(instructions: &Vec<Instruction>) {
    let mut registers = HashMap::new();
    registers.insert("ip", 0);
    let mul_count = run_program(instructions, &mut registers);
    println!("{}", mul_count)
}

fn two() {
    let count = (106700u32..=123700).step_by(17).filter(|n| !aoc::is_prime(*n)).count();
    println!("{}", count);
}