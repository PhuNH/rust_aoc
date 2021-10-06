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
    println!("{}", program.get_register("a"));
}

fn two(lines: Vec<&str>) {
    let program = Program::make_init(lines.clone(), vec!["a"], vec![12]);
    // program.run();
    // just read the damn code by yourself
    println!("{}", aoc::factorial(12) +
        program.get_first_argument(19) * program.get_first_argument(20));
}