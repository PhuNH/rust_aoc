use aoc::assembunny::Program;

fn main() {
    let input = aoc::read_input();
    let lines: Vec<_> = input.lines().collect();
    let program = Program::make(lines);
    let threshold = program.get_first_argument(1) * program.get_first_argument(2);
    println!("{}", find_a(threshold));
}

fn find_a(threshold: u32) -> u32 {
    let mut d = 0;
    let mut c = 1;
    loop {
        d = 2 * d + c;
        if d > threshold && c == 0 { break d - threshold; }
        c = 1 - c;
    }
}
