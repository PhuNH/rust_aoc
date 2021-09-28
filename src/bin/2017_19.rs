use aoc::grid::{TwoDCell, RIGHT, DOWN, LEFT, UP};

fn main() {
    let input = aoc::read_input();
    let map: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();
    one_and_two(&map);
}

fn go(map: &Vec<Vec<char>>, current: Option<(TwoDCell, TwoDCell)>, chars: &mut Vec<char>) -> Option<(TwoDCell, TwoDCell)> {
    match current {
        None => Some((TwoDCell::from(map[0].iter().position(|c| *c != ' ').unwrap() as i32, 1), DOWN)),
        Some((cell, direction)) => {
            if cell.x < 0 || cell.x as usize >= map[0].len() || cell.y < 0 || cell.y as usize >= map.len() ||
                map[cell.y as usize][cell.x as usize] == ' ' { None }
            else {
                let new_direction = match map[cell.y as usize][cell.x as usize] {
                    '+' => if direction == DOWN || direction == UP {
                        if cell.x == 0 { RIGHT }
                        else if cell.x as usize == map[0].len() - 1 { LEFT }
                        else if map[cell.y as usize][(cell.x+1) as usize] != ' ' { RIGHT }
                        else { LEFT }
                    } else {
                        if cell.y == 1 { DOWN }
                        else if cell.y as usize == map.len() - 1 { UP }
                        else if map[(cell.y+1) as usize][cell.x as usize] != ' ' { DOWN }
                        else { UP }
                    },
                    x => {
                        if x != '-' && x != '|' {
                            chars.push(x);
                        }
                        direction
                    }
                };
                let new_cell = cell + new_direction;
                Some((new_cell, new_direction))
            }
        }
    }
}

fn one_and_two(map: &Vec<Vec<char>>) {
    let mut current = None;
    let mut chars = Vec::new();
    let mut steps = 0;
    loop {
        match go(map, current, &mut chars) {
            None => break,
            Some(coord_dir) => {
                current = Some(coord_dir);
                steps += 1;
            }
        }
    }
    for c in chars {
        print!("{}", c);
    }
    println!();
    println!("{}", steps);
}