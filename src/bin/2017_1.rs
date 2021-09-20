fn main() {
    let input = aoc::read_input();
    let chars: Vec<_> = input.trim().chars().collect();
    one(&chars);
    two(&chars);
}

fn find_sum(chars: &Vec<char>, dist: usize) -> u32 {
    chars.iter().enumerate().filter(|(i, &c)| {
        c == chars[(i + dist) % chars.len()]
    }).map(|(_, &c)| c.to_digit(10u32).unwrap()).sum()
}

fn one(chars: &Vec<char>) {
    println!("{}", find_sum(chars, 1));
}

fn two(chars: &Vec<char>) {
    println!("{}", find_sum(chars, chars.len()/2));
}