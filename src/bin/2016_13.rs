use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32
}

impl Coord {
    fn is_space(&self, fav: i32) -> bool {
        let sum = self.x.pow(2) + 3 * self.x + 2 * self.x * self.y + self.y + self.y.pow(2) + fav;
        let binary = format!("{:b}", sum);
        binary.chars().filter(|c| *c == '1').count() % 2 == 0
    }

    fn is_valid(&self) -> bool {
        self.x >= 0 && self.y >= 0
    }

    fn neighbors(&self) -> Vec<Coord> {
        vec![Coord { x: self.x-1, ..*self }, Coord { x: self.x+1, ..*self },
             Coord { y: self.y-1, ..*self }, Coord { y: self.y+1, ..*self }]
    }
}

fn main() {
    let input = 1364;
    let initial_coord = Coord { x: 1, y: 1 };
    one(input, &initial_coord);
    two(input, &initial_coord);
}

fn one(input: i32, initial_coord: &Coord) {
    let dest_coord = Coord { x: 31, y: 39 };
    let mut queue = VecDeque::new();
    queue.push_back((initial_coord.clone(), 0usize));
    let mut passed_coords = HashSet::new();
    passed_coords.insert(initial_coord.clone());
    let step_count = loop {
        let (coord, step) = queue.pop_front().unwrap();
        let neighbors: HashSet<_> = coord.neighbors()
            .iter().cloned()
            .filter(|c| !passed_coords.contains(c) && c.is_valid() && c.is_space(input))
            .collect();
        // println!("neighbors: {:?}", neighbors);
        // println!("{}", step+1);
        if neighbors.contains(&dest_coord) {
            break step+1;
        }
        neighbors.iter().cloned().for_each(|c| {
            queue.push_back((c.clone(), step+1));
            passed_coords.insert(c);
        });
    };
    println!("{}", step_count);
}

fn two(input: i32, initial_coord: &Coord) {
    let mut queue = VecDeque::new();
    queue.push_back((initial_coord.clone(), 0usize));
    let mut passed_coords = HashSet::new();
    passed_coords.insert(initial_coord.clone());
    loop {
        let (coord, step) = queue.pop_front().unwrap();
        if step == 50 { break }
        let neighbors: HashSet<_> = coord.neighbors()
            .iter().cloned()
            .filter(|c| !passed_coords.contains(c) && c.is_valid() && c.is_space(input))
            .collect();
        neighbors.iter().cloned().for_each(|c| {
            queue.push_back((c.clone(), step+1));
            passed_coords.insert(c);
        });
    };
    println!("{}", passed_coords.len());
}