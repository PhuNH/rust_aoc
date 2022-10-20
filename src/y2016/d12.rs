use super::assembunny::Program;

pub fn main() {
    let input = aoc::read_input(module_path!());
    let lines = input.lines().collect::<Vec<_>>();
    one(&lines);
    two(lines);
}

fn one(lines: &Vec<&str>) {
    let mut program = Program::make(lines.clone());
    program.run();
    println!("{}", program.get_register("a"));
}

fn two(lines: Vec<&str>) {
    let mut program = Program::make_init(lines, vec!["c"], vec![1]);
    program.run();
    println!("{}", program.get_register("a"));
}