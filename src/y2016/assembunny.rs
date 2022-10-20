use std::collections::HashMap;
use std::cell::Cell;

#[derive(Clone)]
enum Value<'a> {
    Literal(i32),
    Register(&'a str),
}

impl<'a> Value<'a> {
    fn make(input: &str) -> Value {
        match input.parse::<i32>().ok() {
            None => Value::Register(input),
            Some(l) => Value::Literal(l)
        }
    }

    fn get(&self, registers: &Registers) -> i32 {
        match self {
            Value::Literal(l) => *l,
            Value::Register(s) => {
                let register = registers.get(s).get();
                register
            }
        }
    }
}

#[derive(Clone)]
enum Instruction<'a, 'b> {
    Cpy(Value<'a>, &'b str),
    Inc(&'a str),
    Dec(&'a str),
    Jnz(Value<'a>, Value<'b>),
    Tgl(Value<'a>),
}

impl Instruction<'_, '_> {
    fn cpy(registers: &Registers, value: &Value, register_name: &str) {
        let register = registers.get(register_name);
        register.set(value.get(registers));
    }

    fn inc(registers: &Registers, register_name: &str) {
        let register = registers.get(register_name);
        register.set(register.get() + 1);
    }

    fn dec(registers: &Registers, register_name: &str) {
        let register = registers.get(register_name);
        register.set(register.get() - 1);
    }

    fn jnz(registers: &Registers, value: &Value, dist: &Value, ip_dist: &mut i32) {
        if value.get(registers) != 0 {
            *ip_dist = dist.get(registers);
        }
    }
}

#[derive(Clone, PartialEq)]
enum Morph {
    Original, // others
    Intermediate, // Jnz, Inc
    Final, // Cpy, Dec
}

#[derive(Clone)]
struct MorphingInstruction<'a, 'b, 'c> {
    instr: Instruction<'a, 'b>,
    morph: Morph,
    string: &'c str,
}

impl MorphingInstruction<'_, '_, '_> {
    fn make(input: &str) -> MorphingInstruction {
        let words = input.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "cpy" => {
                let source = Value::make(words[1]);
                let target = words[2];
                MorphingInstruction { instr: Instruction::Cpy(source, target), morph: Morph::Final, string: input }
            },
            "inc" => MorphingInstruction { instr: Instruction::Inc(words[1]), morph: Morph::Intermediate, string: input },
            "dec" => MorphingInstruction { instr: Instruction::Dec(words[1]), morph: Morph::Final, string: input },
            "jnz" => {
                let cond = Value::make(words[1]);
                let dist = Value::make(words[2]);
                MorphingInstruction { instr: Instruction::Jnz(cond, dist), morph: Morph::Intermediate, string: input }
            },
            _ => {
                let dist = Value::make(words[1]);
                MorphingInstruction { instr: Instruction::Tgl(dist), morph: Morph::Original, string: input }
            }
        }
    }

    fn morph(&mut self) {
        if self.morph == Morph::Intermediate {
            self.morph = Morph::Final;
        } else {
            self.morph = Morph::Intermediate;
        }
    }

    fn run(&self, program: &mut Program) {
        let ip = program.registers.get("ip");
        let mut ip_dist = 1;
        match &self.instr {
            Instruction::Cpy(value, register_name) => {
                if self.morph != Morph::Intermediate { Instruction::cpy(&program.registers, value, register_name); }
                else { Instruction::jnz(&program.registers, value, &Value::Register(register_name.clone()), &mut ip_dist); }
            },
            Instruction::Inc(register_name) => {
                if self.morph != Morph::Final { Instruction::inc(&program.registers, register_name); }
                else { Instruction::dec(&program.registers, register_name); }
            },
            Instruction::Dec(register_name) => {
                if self.morph != Morph::Intermediate { Instruction::dec(&program.registers, register_name); }
                else { Instruction::inc(&program.registers, register_name); }
            },
            Instruction::Jnz(value, dist) => {
                if self.morph != Morph::Final { Instruction::jnz(&program.registers, value, dist, &mut ip_dist); }
                else {
                    if let Value::Register(register_name) = dist {
                        Instruction::cpy(&program.registers, value, register_name);
                    }
                }
            },
            Instruction::Tgl(dist) => {
                if self.morph == Morph::Original {
                    let instr_to_tgl_index = (ip.get() + dist.get(&program.registers)) as usize;
                    if instr_to_tgl_index < program.instructions.len() {
                        program.instructions[instr_to_tgl_index].morph();
                    }
                } else if self.morph == Morph::Intermediate {
                    if let Value::Register(register_name) = dist {
                        Instruction::inc(&program.registers, register_name);
                    }
                } else {
                    if let Value::Register(register_name) = dist {
                        Instruction::dec(&program.registers, register_name);
                    }
                }
            }
        }
        ip.set(ip.get() + ip_dist);
    }
}

struct Registers<'a> {
    registers: HashMap<&'a str, Cell<i32>>,
}

impl<'a> Registers<'a> {
    fn new() -> Registers<'a> {
        let mut registers = HashMap::new();
        ["a", "b", "c", "d", "ip"].iter().for_each(|&r| {
            registers.insert(r, Cell::new(0));
        });
        Registers { registers }
    }

    fn update(&mut self, names: Vec<&'a str>, values: Vec<i32>) {
        if names.len() > values.len() {
            panic!("There are more names than values.");
        }

        names.iter().enumerate().for_each(|(i, &name)| {
            self.registers.insert(name, Cell::new(values[i]));
        });
    }

    fn get(&self, register_name: &str) -> &Cell<i32> {
        self.registers.get(register_name).unwrap()
    }
}

pub struct Program<'a, 'b, 'c, 'd> {
    instructions: Vec<MorphingInstruction<'a, 'b, 'c>>,
    registers: Registers<'d>,
}

impl Program<'_, '_, '_, '_> {
    pub fn make(lines: Vec<&str>) -> Program {
        let instructions = lines.into_iter().map(|l| MorphingInstruction::make(l)).collect::<Vec<_>>();
        let registers = Registers::new();
        Program { instructions, registers }
    }

    pub fn make_init<'a, 'c>(lines: Vec<&'a str>, names: Vec<&'c str>, values: Vec<i32>) -> Program<'a, 'a, 'a, 'c> {
        let instructions = lines.into_iter().map(|l| MorphingInstruction::make(l)).collect::<Vec<_>>();
        let mut registers = Registers::new();
        registers.update(names, values);
        Program { instructions, registers }
    }

    pub fn run(&mut self) {
        loop {
            let ip = self.registers.get("ip").get();
            if ip < 0 || ip as usize >= self.instructions.len() { break }
            let instr = self.instructions[ip as usize].clone();
            instr.run(self);
        }
    }

    pub fn get_register(&self, register_name: &str) -> i32 {
        self.registers.get(register_name).get()
    }

    pub fn get_instruction_string(&self, instr_index: usize) -> &str {
        assert!(instr_index < self.instructions.len(), "Invalid index");
        self.instructions[instr_index].string
    }

    pub fn get_first_argument(&self, instr_index: usize) -> u32 {
        self.get_instruction_string(instr_index).split_whitespace().nth(1).unwrap().parse().unwrap()
    }
}


