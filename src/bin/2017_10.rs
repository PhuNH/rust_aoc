fn main() {
    let input = aoc::read_input();
    let lengths: Vec<usize> = input.trim().split(",").map(|s| s.parse().unwrap()).collect();
    one(lengths);
    let chars: Vec<_> = input.trim().chars().collect();
    two(chars);
}

fn one(lengths: Vec<usize>) {
    let mut list = (0..=255).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    aoc::knot_hash_round(&lengths, &mut list, &mut current_position, &mut skip_size);
    println!("{}", list[0] * list[1]);
}

fn two(chars: Vec<char>) {
    for n in aoc::knot_hash(chars) {
        print!("{:02x}", n);
    }
}