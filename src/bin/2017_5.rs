fn main() {
    let input = aoc::read_input();
    let jumps: Vec<_> = input.lines().map(|l| l.parse::<i32>().unwrap()).collect();
    one(jumps.clone());
    two(jumps);
}

fn one(mut jumps: Vec<i32>) {
    let mut ip = 0i32;
    let mut offset;
    let mut step = 0;
    loop {
        step += 1;
        offset = jumps[ip as usize];
        jumps[ip as usize] += 1;
        ip += offset;
        if ip as usize >= jumps.len() {
            break;
        }
    }
    println!("{}", step);
}

fn two(mut jumps: Vec<i32>) {
    let mut ip = 0i32;
    let mut offset;
    let mut step = 0;
    loop {
        step += 1;
        offset = jumps[ip as usize];
        if offset >= 3 { jumps[ip as usize] -= 1; }
        else { jumps[ip as usize] += 1; }
        ip += offset;
        if ip as usize >= jumps.len() {
            break;
        }
    }
    println!("{}", step);
}