fn main() {
    let input = aoc::read_input();
    let lines: Vec<Vec<_>> = input.lines().map(|l| l.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect()).collect();
    one(&lines);
    two(&lines);
}

fn one(lines: &Vec<Vec<u32>>) {
    let sum: u32 = lines.iter().map(|l| l.iter().max().unwrap() - l.iter().min().unwrap()).sum();
    println!("{}", sum);
}

fn two(lines: &Vec<Vec<u32>>) {
    let sum: u32 = lines.iter().map(|l| l[..l.len()-1].iter().enumerate().map(|(i, &n)| {
        match l[i+1..].iter().find(|&d| if n >= *d { n % *d == 0 } else { *d % n == 0 }) {
            None => 0,
            Some(&d) => if n >= d { n / d } else { d / n }
        }
    }).filter(|&q| q != 0).next().unwrap()).sum();
    println!("{}", sum);
}