#[derive(Clone)]
struct Generator {
    factor: u64,
    prev: u64,
    criteria: u64,
}

impl Generator {
    fn next(&mut self) -> u64 {
        let next_prev = self.prev * self.factor % (i32::MAX as u64);
        self.prev = next_prev;
        next_prev
    }

    fn tighter_next(&mut self) -> u64 {
        let next_prev = self.prev * self.factor % (i32::MAX as u64);
        self.prev = next_prev;
        if next_prev % self.criteria == 0 { next_prev }
        else { self.tighter_next() }
    }

    fn equal(&mut self, other: &mut Generator, type_of_next: fn(&mut Generator) -> u64) -> bool {
        format!("{:032b}", type_of_next(self))[16..] == format!("{:032b}", type_of_next(other))[16..]
    }
}

pub fn main() {
    let input = aoc::read_input(module_path!());
    let lines: Vec<Vec<_>> = input.lines().map(|l| l.split_whitespace().collect()).collect();
    let gen_a = Generator { factor: 16807, prev: lines[0][4].parse().unwrap(), criteria: 4 };
    let gen_b = Generator { factor: 48271, prev: lines[1][4].parse().unwrap(), criteria: 8 };
    one(gen_a.clone(), gen_b.clone());
    two(gen_a, gen_b);
}

fn one(mut gen_a: Generator, mut gen_b: Generator) {
    let count: u32 = (0..40000000).map(|_| {
        if gen_a.equal(&mut gen_b, Generator::next) { 1 } else { 0 }
    }).sum();
    println!("{}", count);
}

fn two(mut gen_a: Generator, mut gen_b: Generator) {
    let count: u32 = (0..5000000).map(|_| {
        if gen_a.equal(&mut gen_b, Generator::tighter_next) { 1 } else { 0 }
    }).sum();
    println!("{}", count);
}