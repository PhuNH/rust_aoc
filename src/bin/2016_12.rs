use aoc::assembunny::Program;
use std::collections::HashMap;
use std::cell::Cell;

fn main() {
    let input = aoc::read_input();
    let lines = input.lines().collect::<Vec<_>>();
    let program = Program::make(lines);
    one(&program);
    two(&program);
}

fn one(program: &Program) {
    let mut registers = HashMap::new();
    ["a", "b", "c", "d", "ip"].iter().for_each(|&r| {
        registers.insert(String::from(r), Cell::new(0));
    });
    program.run(&registers);
    println!("{}", registers.get("a").unwrap().get());
}

fn two(program: &Program) {
    let mut registers = HashMap::new();
    ["a", "b", "d", "ip"].iter().for_each(|&r| {
        registers.insert(String::from(r), Cell::new(0));
    });
    registers.insert(String::from("c"), Cell::new(1));
    program.run(&registers);
    println!("{}", registers.get("a").unwrap().get());
}