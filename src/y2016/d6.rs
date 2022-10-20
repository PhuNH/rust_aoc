use std::collections::HashMap;
use std::cmp::Ordering;

pub fn main() {
    let input = aoc::read_input(module_path!());
    let lines: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    let counts = get_total_counts(&lines);
    one(&counts);
    two(&counts);
}

fn get_total_counts(lines: &Vec<Vec<char>>) -> Vec<HashMap<char, u32>> {
    let length = lines[0].len();
    let mut counts = vec![HashMap::new(); length];
    for l in lines {
        for (i, c) in l.iter().enumerate() {
            let cnt = counts[i].entry(*c).or_insert(0);
            *cnt += 1;
        }
    }
    counts
}

fn compare(counts: &Vec<HashMap<char, u32>>,
           closure: &impl Fn(&(&char, &u32), &(&char, &u32)) -> Ordering) {
    let res: Vec<_> = counts.iter().map(|m| {
        let mut cnt_vec: Vec<_> = m.iter().collect();
        cnt_vec.sort_by(closure);
        *cnt_vec[0].0
    }).collect();
    println!("{}", res.iter().collect::<String>());
}

fn one(counts: &Vec<HashMap<char, u32>>) {
    let closure = |a: &(&char, &u32), b: &(&char, &u32)| a.1.cmp(b.1).reverse();
    compare(counts, &closure);
}

fn two(counts: &Vec<HashMap<char, u32>>) {
    let closure = |a: &(&char, &u32), b: &(&char, &u32)| a.1.cmp(b.1);
    compare(counts, &closure);
}
