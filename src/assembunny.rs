use std::collections::HashMap;
use std::cell::Cell;

enum Value {
    Literal(i32),
    Register(String),
}

impl Value {
    fn make(input: &str) -> Value {
        match input.parse::<i32>().ok() {
            None => Value::Register(String::from(input)),
            Some(l) => Value::Literal(l)
        }
    }

    fn get(&self, registers: &HashMap<String, Cell<i32>>) -> i32 {
        match self {
            Value::Literal(l) => *l,
            Value::Register(s) => {
                let register = registers.get(s).unwrap().get();
                register
            }
        }
    }
}

enum Instruction {
    Copy(Value, String),
    Inc(String),
    Dec(String),
    Jnz(Value, Value),
}

impl Instruction {
    fn make(input: &str) -> Instruction {
        let words = input.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                let source = Value::make(words[1]);
                let target = String::from(words[2]);
                Instruction::Copy(source, target)
            },
            "inc" => Instruction::Inc(String::from(words[1])),
            "dec" => Instruction::Dec(String::from(words[1])),
            _ => {
                let cond = Value::make(words[1]);
                let dist = Value::make(words[2]);
                Instruction::Jnz(cond, dist)
            }
        }
    }

    fn run(&self, registers: &HashMap<String, Cell<i32>>) {
        let ip = registers.get("ip").unwrap();
        let mut ip_dist = 1;
        match self {
            Instruction::Copy(value, register_name) => {
                let register = registers.get(register_name).unwrap();
                register.set(value.get(registers));
            }
            Instruction::Inc(register_name) => {
                let register = registers.get(register_name).unwrap();
                register.set(register.get() + 1);
            },
            Instruction::Dec(register_name) => {
                let register = registers.get(register_name).unwrap();
                register.set(register.get() - 1);
            },
            Instruction::Jnz(value, dist) => {
                if value.get(registers) != 0 {
                    ip_dist = dist.get(registers);
                }
            }
        }
        ip.set(ip.get() + ip_dist);
    }
}

pub struct Program {
    instructions: Vec<Instruction>
}

impl Program {
    pub fn make(lines: Vec<&str>) -> Program {
        let instructions = lines.iter().map(|&l| Instruction::make(l)).collect::<Vec<_>>();
        Program { instructions }
    }

    pub fn run(&self, registers: &HashMap<String, Cell<i32>>) {
        loop {
            let ip = registers.get("ip").unwrap().get();
            if ip < 0 || ip as usize >= self.instructions.len() { break }
            self.instructions[ip as usize].run(registers);
        }
    }
}


