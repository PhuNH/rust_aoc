use aoc::grid::{TwoDCell, UP, RIGHT, DOWN, LEFT};
use std::collections::HashMap;

fn turn_right(direction: &mut TwoDCell) {
    *direction = match *direction {
        UP => RIGHT,
        RIGHT => DOWN,
        DOWN => LEFT,
        _ => UP,
    }
}

fn turn_left(direction: &mut TwoDCell) {
    *direction = match *direction {
        UP => LEFT,
        LEFT => DOWN,
        DOWN => RIGHT,
        _ => UP,
    }
}

fn reverse(direction: &mut TwoDCell) {
    *direction = match *direction {
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        _ => LEFT,
    }
}

fn burst(location: &mut TwoDCell, direction: &mut TwoDCell, infected: &mut Vec<TwoDCell>, infected_count: &mut usize) {
    if let Some(index) = infected.iter().position(|node| *node == *location) {
        turn_right(direction);
        infected.remove(index);
    } else {
        turn_left(direction);
        infected.push(*location);
        *infected_count += 1;
    }
    *location = *location + *direction;
}

pub fn main() {
    let input = aoc::read_input(module_path!());
    let height = input.lines().count();
    let width = input.lines().next().unwrap().chars().count();
    let location = TwoDCell::from(width as i32 / 2, height as i32 / 2);
    let direction = UP;
    let infected: Vec<_> = input.lines().enumerate().flat_map(|(y, l)| {
        l.chars().enumerate().filter(|(_, c)| *c == '#')
            .map(move |(x, _)| TwoDCell::from(x as i32, y as i32))
    }).collect();

    one(location, direction, infected.clone());
    two(location, direction, infected);
}

fn one(mut location: TwoDCell, mut direction: TwoDCell, mut infected: Vec<TwoDCell>) {
    let mut infected_count = 0;
    for _ in 0..10000 {
        burst(&mut location, &mut direction, &mut infected, &mut infected_count);
    }
    println!("{}", infected_count);
}

fn evolved_burst(location: &mut TwoDCell, direction: &mut TwoDCell,
                 infected: &mut HashMap<TwoDCell, usize>, infected_count: &mut usize) {
    /*
    0: weakened
    1: infected
    2: flagged
    3: clean
    */
    if infected.contains_key(location) {
        if *infected.get(location).unwrap() == 1 {
            turn_right(direction);
        } else if *infected.get(location).unwrap() == 2 {
            reverse(direction);
        }
        *infected.get_mut(location).unwrap() += 1;
        if *infected.get(location).unwrap() == 3 {
            infected.remove(location);
        } else if *infected.get(location).unwrap() == 1 {
            *infected_count += 1;
        }
    } else {
        turn_left(direction);
        infected.insert(*location, 0);
    }
    *location = *location + *direction;
}

fn two(mut location: TwoDCell, mut direction: TwoDCell, infected_cells: Vec<TwoDCell>) {
    let mut infected= infected_cells.into_iter().map(|c| (c, 1)).collect();
    let mut infected_count = 0;
    for i in 0..10000000 {
        evolved_burst(&mut location, &mut direction, &mut infected, &mut infected_count);
        if i % 10000 == 0 { println!("{}", i); }
    }
    println!("{}", infected_count);
}