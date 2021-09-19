use std::collections::HashSet;

const HEIGHT: u32 = 6;
const WIDTH: u32 = 50;

fn main() {
    let input = aoc::read_input();
    let lines: Vec<&str> = input.lines().collect();
    let mut set = HashSet::<(u32, u32)>::new();
    process(&lines, &mut set);
    one(&set);
    two(&set);
}

fn rect(set: &mut HashSet<(u32, u32)>, a: u32, b: u32) {
    for i in 0..b {
        for j in 0..a {
            set.insert((i, j));
        }
    }
}

fn rotate_row(set: &mut HashSet<(u32, u32)>, a: u32, b: u32, width: u32) {
    let row_elms: HashSet<(u32, u32)> = set.iter().filter(|e| e.0 == a).cloned().collect();
    let new_row_elms: HashSet<(u32, u32)> = row_elms.iter().map(|e| (e.0, (e.1+b)%width)).collect();
    row_elms.iter().for_each(|e| { set.remove(e); });
    new_row_elms.iter().for_each(|e| { set.insert(*e); });
}

fn rotate_col(set: &mut HashSet<(u32, u32)>, a: u32, b: u32, height: u32) {
    let col_elms: HashSet<(u32, u32)> = set.iter().filter(|e| e.1 == a).cloned().collect();
    let new_col_elms: HashSet<(u32, u32)> = col_elms.iter().map(|e| ((e.0+b)%height, e.1)).collect();
    col_elms.iter().for_each(|e| { set.remove(e); });
    new_col_elms.iter().for_each(|e| { set.insert(*e); });
}

fn process(lines: &Vec<&str>, set: &mut HashSet<(u32, u32)>) {
    lines.iter().for_each(|l| {
        if l.starts_with("rect") {
            let ab: Vec<u32> = l.split(' ').nth(1).unwrap().split('x').map(|p| p.parse().unwrap()).collect();
            rect(set, ab[0], ab[1]);
        } else {
            let ab: Vec<u32> = l.split('=').nth(1).unwrap().split(" by ").map(|p| p.parse().unwrap()).collect();
            if l.starts_with("rotate row") {
                rotate_row(set, ab[0], ab[1], WIDTH);
            } else {
                rotate_col(set, ab[0], ab[1], HEIGHT);
            }
        }
    })
}

fn one(set: &HashSet<(u32, u32)>) {
    println!("{}", set.len());
}

fn two(set: &HashSet<(u32, u32)>) {
    let mut screen: Vec<Vec<char>> = vec!(vec!('_'; WIDTH as usize); HEIGHT as usize);
    set.iter().for_each(|(i, j)| screen[*i as usize][*j as usize] = '*');
    screen.iter().for_each(|l| {
        l.iter().for_each(|c| print!("{} ", c));
        println!();
        // let s: String = l.iter().collect();
        // println!("{}", s);
    });
}