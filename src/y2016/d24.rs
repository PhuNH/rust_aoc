use std::collections::{HashMap, HashSet, VecDeque};
use aoc::grid::TwoDCell;

pub fn main() {
    let input = aoc::read_input(module_path!());
    let mut vertices = HashMap::new();
    let mut movables = HashSet::new();
    input.lines().enumerate().for_each(|(y, l)| {
        l.chars().enumerate().filter(|(_, c)| *c != '#')
            .for_each(|(x, c)| {
                let cell = TwoDCell::from(x as i32, y as i32);
                movables.insert(cell);
                if c != '.' {
                    vertices.insert(cell, c);
                }
            })
    });
    let lengths: HashMap<_, _> = vertices.values().map(|&c|
        (c, find_lengths(c, &vertices, &movables))).collect();
    println!("{:?}", lengths);
    one(&lengths);
    two(&lengths);
}

struct LengthSearchUnit {
    cell: TwoDCell,
    length: usize,
}

fn find_lengths(from: char, vertices: &HashMap<TwoDCell, char>, movables: &HashSet<TwoDCell>) -> HashMap<char, usize> {
    let mut lengths = HashMap::new();
    let mut left_names: HashSet<_> = vertices.values().filter(|c| **c != from).cloned().collect();

    let mut search_queue = VecDeque::new();
    let from_cell = *vertices.iter().find(|(_, c)| **c == from).unwrap().0;
    let mut searched = HashSet::new();
    searched.insert(from_cell);
    let mut current_unit = LengthSearchUnit {
        cell: from_cell,
        length: 0
    };
    search_queue.push_back(current_unit);

    loop {
        if search_queue.is_empty() { break; }

        current_unit = search_queue.pop_front().unwrap();
        if vertices.contains_key(&current_unit.cell) && current_unit.cell != from_cell {
            let vertex_name = *vertices.get(&current_unit.cell).unwrap();
            left_names.remove(&vertex_name);
            lengths.insert(vertex_name, current_unit.length);
        }
        if left_names.is_empty() { break; }

        let next_possible_cells: Vec<_> = current_unit.cell.neighbors_4().iter()
            .filter(|&c| movables.contains(c) && !searched.contains(c))
            .cloned().collect();
        let next_possible_units: Vec<_> = next_possible_cells.iter()
            .map(|&cell| {
                searched.insert(cell);
                LengthSearchUnit { cell, length: current_unit.length + 1 }
            }).collect();
        search_queue.extend(next_possible_units);
    }
    lengths
}

struct ShortestLengthSearchUnit {
    route: Vec<char>,
    left_names: HashSet<char>,
    length: usize,
}

fn shortest_route_throughout(from: char, lengths: &HashMap<char, HashMap<char, usize>>, back: bool) -> usize {
    let mut search_stack = Vec::new();
    let mut shortest = usize::MAX;

    let mut left_names: HashSet<_> = lengths.keys().cloned().collect();
    left_names.remove(&from);
    let mut current_unit = ShortestLengthSearchUnit {
        route: vec![from],
        left_names,
        length: 0,
    };
    search_stack.push(current_unit);

    loop {
        if search_stack.is_empty() { break; }

        current_unit = search_stack.pop().unwrap();
        if current_unit.left_names.is_empty() {
            if !back {
                if current_unit.length < shortest {
                    shortest = current_unit.length;
                }
            } else {
                let tip = current_unit.route.last().unwrap();
                let back_length = lengths.get(tip).unwrap().get(&from).unwrap();
                let total_length = current_unit.length + back_length;
                if total_length < shortest {
                    shortest = total_length;
                }
            }
            continue;
        }

        let units: Vec<_> = current_unit.left_names.iter().map(|c| {
            let length = current_unit.length + lengths.get(current_unit.route.last().unwrap()).unwrap().get(&c).unwrap();
            let mut route = current_unit.route.clone();
            route.push(*c);
            let mut left_names = current_unit.left_names.clone();
            left_names.remove(c);
            ShortestLengthSearchUnit { route, left_names, length }
        }).collect();
        search_stack.extend(units);
    }
    shortest
}

fn one(lengths: &HashMap<char, HashMap<char, usize>>) {
    println!("{}", shortest_route_throughout('0', lengths, false));
}

fn two(lengths: &HashMap<char, HashMap<char, usize>>) {
    println!("{}", shortest_route_throughout('0', lengths, true));
}