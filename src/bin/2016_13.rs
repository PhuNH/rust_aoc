use std::collections::{VecDeque, HashSet};
use aoc::grid::TwoDCell;

trait Day13 {
    fn is_space(&self, fav: i32) -> bool;
    fn is_valid(&self) -> bool;
    fn neighbors(&self) -> Vec<TwoDCell>;
}

impl Day13 for TwoDCell {
    fn is_space(&self, fav: i32) -> bool {
        let sum = self.x.pow(2) + 3 * self.x + 2 * self.x * self.y + self.y + self.y.pow(2) + fav;
        let binary = format!("{:b}", sum);
        binary.chars().filter(|c| *c == '1').count() % 2 == 0
    }

    fn is_valid(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    fn neighbors(&self) -> Vec<TwoDCell> {
        vec![TwoDCell { x: self.x-1, ..*self }, TwoDCell { x: self.x+1, ..*self },
             TwoDCell { y: self.y-1, ..*self }, TwoDCell { y: self.y+1, ..*self }]
    }
}

fn main() {
    let input = 1364;
    let initial_coord = TwoDCell { x: 1, y: 1 };
    one(input, initial_coord.clone());
    two(input, initial_coord);
}

fn one(input: i32, initial_coord: TwoDCell) {
    let dest_coord = TwoDCell { x: 31, y: 39 };
    let mut queue = VecDeque::new();
    queue.push_back((initial_coord.clone(), 0usize));
    let mut passed_coords = HashSet::new();
    passed_coords.insert(initial_coord);
    let step_count = loop {
        let (coord, step) = queue.pop_front().unwrap();
        let neighbors: HashSet<_> = coord.neighbors()
            .into_iter()
            .filter(|c| !passed_coords.contains(c) && c.is_valid() && c.is_space(input))
            .collect();
        // println!("neighbors: {:?}", neighbors);
        // println!("{}", step+1);
        if neighbors.contains(&dest_coord) {
            break step+1;
        }
        neighbors.into_iter().for_each(|c| {
            queue.push_back((c.clone(), step+1));
            passed_coords.insert(c);
        });
    };
    println!("{}", step_count);
}

fn two(input: i32, initial_coord: TwoDCell) {
    let mut queue = VecDeque::new();
    queue.push_back((initial_coord.clone(), 0usize));
    let mut passed_coords = HashSet::new();
    passed_coords.insert(initial_coord);
    loop {
        let (coord, step) = queue.pop_front().unwrap();
        if step == 50 { break }
        let neighbors: HashSet<_> = coord.neighbors()
            .into_iter()
            .filter(|c| !passed_coords.contains(c) && c.is_valid() && c.is_space(input))
            .collect();
        neighbors.into_iter().for_each(|c| {
            queue.push_back((c.clone(), step+1));
            passed_coords.insert(c);
        });
    };
    println!("{}", passed_coords.len());
}