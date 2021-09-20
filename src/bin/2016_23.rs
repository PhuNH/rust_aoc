use aoc::assembunny::Program;

fn main() {
    let input = aoc::read_input();
    let lines: Vec<_> = input.lines().collect();
    one(&lines);
    two(lines);
}

fn one(lines: &Vec<&str>) {
    let mut program = Program::make_init(lines.clone(), vec!["a"], vec![7]);
    program.run();
    println!("{}", program.get("a"));
}

fn two(lines: Vec<&str>) {
    let mut program = Program::make_init(lines.clone(), vec!["a"], vec![12]);
    program.run();
    println!("{}", program.get("a"));
}