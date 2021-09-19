type Tile = char;
const TRAP: Tile = '^';
const SAFE: Tile = '.';

fn produce_tile(prev: (Tile, Tile, Tile)) -> Tile {
    match prev {
        (TRAP, TRAP, SAFE) => TRAP,
        (SAFE, TRAP, TRAP) => TRAP,
        (TRAP, SAFE, SAFE) => TRAP,
        (SAFE, SAFE, TRAP) => TRAP,
        _ => SAFE
    }
}

fn produce_row(prev: Vec<Tile>) -> Vec<Tile> {
    prev.iter().enumerate().map(|(i, c)| {
        if i == 0 { produce_tile((SAFE, *c, prev[i+1])) }
        else if i == prev.len()-1 { produce_tile((prev[i-1], *c, SAFE)) }
        else { produce_tile((prev[i-1], *c, prev[i+1])) }
    }).collect()
}

fn main() {
    let input = aoc::read_input().trim_end().chars().collect();
    one(&input);
    two(&input);
}

fn count(input: &Vec<char>, rows: usize) -> usize {
    let mut row = input.clone();
    let mut count = row.iter().filter(|&t| *t == SAFE).collect::<Vec<_>>().len();
    for _ in 1..rows {
        row = produce_row(row);
        count += row.iter().filter(|&t| *t == SAFE).collect::<Vec<_>>().len();
    }
    count
}

fn one(input: &Vec<char>) {
    println!("{}", count(input, 40));
}

fn two(input: &Vec<char>) {
    println!("{}", count(input, 400000));
}