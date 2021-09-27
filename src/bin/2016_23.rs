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

fn factorial(n: u32) -> u32 {
    if n == 2 { 2 }
    else { n * factorial(n-1) }
}

fn get_first_argument(program: &Program, instr_index: usize) -> u32 {
    program.get_instruction_string(instr_index).split_whitespace().nth(1).unwrap().parse().unwrap()
}

fn two(lines: Vec<&str>) {
    let program = Program::make_init(lines.clone(), vec!["a"], vec![12]);
    // program.run();
    // just read the damn code by yourself
    println!("{}", factorial(12) +
        get_first_argument(&program, 19) * get_first_argument(&program, 20));
}