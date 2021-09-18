use aoc::*;
use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::{Display, Formatter};

struct Node {
    size: u32,
    used: u32,
    avail: u32,
}

struct State {
    grid: Grid<RefCell<Node>>,
    focus: usize,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{: >4}", " ")?;
        for i in 0..=self.grid.max_x {
            write!(f, "{: >3}{: >6}", i, " ")?;
        }
        for (i, node) in self.grid.data.iter().enumerate() {
            if i % (self.grid.max_x as usize +1) == 0 {
                write!(f, "\n{: >2}  ", i / (self.grid.max_x as usize +1))?;
            }
            let data = node.borrow();
            let focus_mark = if i == self.focus { "." } else { " " };
            write!(f, "{: >3}|{: >3}{} ", data.used, data.size, focus_mark)?;
        }
        writeln!(f)
    }
}

impl State {
    fn from(grid: Grid<RefCell<Node>>) -> State {
        let focus = grid.upper_right();
        State { grid, focus }
    }
}

fn process_input(lines: Vec<&str>) -> Grid<RefCell<Node>> {
    let data: HashMap<_, _> = lines.into_iter().map(|l| {
        let words = l.split_whitespace().collect::<Vec<_>>();
        let coords = words[0].split('-').skip(1)
            .map(|c| c.chars().skip(1).collect::<String>().parse().unwrap())
            .collect::<Vec<i32>>();
        let coords = TwoDCell::from(coords[0], coords[1]);
        let props = (1..=4).map(|i| {
            let chars = words[i].chars().collect::<Vec<_>>();
            chars[..chars.len()-1].iter().collect::<String>().parse().unwrap()
        }).collect::<Vec<_>>();
        let node = RefCell::new(Node { size: props[0], used: props[1], avail: props[2] });
        (coords, node)
    }).collect();
    Grid::from(data)
}

fn main() {
    let input = utils::read_input();
    let lines = input.lines().skip(2).collect::<Vec<_>>();
    let grid = process_input(lines);
    one(&grid);
    two(grid);
}

fn one(grid: &Grid<RefCell<Node>>) {
    let pair_count: usize = grid.data.iter().enumerate().map(|(index_b, node_b)| {
        grid.data.iter().enumerate().filter(|(index_a, node_a)| {
            *index_a != index_b && node_a.borrow().used > 0 && node_a.borrow().used <= node_b.borrow().avail
        }).collect::<Vec<_>>().len()
    }).sum();
    println!("{}", pair_count);
}

fn two(grid: Grid<RefCell<Node>>) {
    let state = State::from(grid);
    println!("{}", state);
}