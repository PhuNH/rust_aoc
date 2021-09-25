use std::ops;
use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Formatter};

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct TwoDCell {
    pub x: i32,
    pub y: i32,
}

const RIGHT: TwoDCell = TwoDCell { x: 1, y: 0 };
const RIGHT_DOWN: TwoDCell = TwoDCell { x: 1, y: 1 };
const DOWN: TwoDCell = TwoDCell { x: 0, y: 1 };
const LEFT_DOWN: TwoDCell = TwoDCell { x: -1, y: 1 };
const LEFT: TwoDCell = TwoDCell { x: -1, y: 0 };
const LEFT_UP: TwoDCell = TwoDCell { x: -1, y: -1 };
const UP: TwoDCell = TwoDCell { x: 0, y: -1 };
const RIGHT_UP: TwoDCell = TwoDCell { x: 1, y: -1 };

impl ops::Add for TwoDCell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Display for TwoDCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl TwoDCell {
    pub fn from(x: i32, y: i32) -> TwoDCell {
        TwoDCell { x, y }
    }

    pub fn mht_dist(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn mht_dist_to(&self, other: &TwoDCell) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn neighbors_4(&self) -> Vec<TwoDCell> {
        vec![*self + RIGHT, *self + DOWN, *self + LEFT, *self + UP]
    }

    pub fn neighbors_8(&self) -> Vec<TwoDCell> {
        vec![*self + RIGHT, *self + RIGHT_DOWN, *self + DOWN, *self + LEFT_DOWN,
             *self + LEFT, *self + LEFT_UP, *self + UP, *self + RIGHT_UP]
    }
}

pub struct Grid<T> {
    pub max_x: i32,
    pub max_y: i32,
    pub data: Vec<T>,
}

impl<T> Grid<T> {
    pub fn coord_to_index(&self, coord: TwoDCell) -> usize {
        (coord.y * (self.max_x+1) + coord.x) as usize
    }

    pub fn from(mut data_map: HashMap<TwoDCell, T>) -> Grid<T> {
        let max_x = data_map.keys().map(|k| k.x).max().unwrap();
        let max_y = data_map.keys().map(|k| k.y).max().unwrap();
        let mut data = Vec::new();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let cell = data_map.remove(&TwoDCell::from(x, y)).unwrap();
                data.push(cell);
            }
        }
        Grid { max_x, max_y, data }
    }

    pub fn get(&self, index: usize) -> &T {
        self.data.get(index).unwrap()
    }

    pub fn upper_right(&self) -> usize { self.max_x as usize }

    pub fn neighbors(&self, index: usize) -> Vec<usize> {
        let cell = TwoDCell::from(index as i32 % (self.max_x+1), index as i32 / (self.max_x+1));
        let mut all_directions: HashSet<_> = vec![RIGHT, DOWN, LEFT, UP].into_iter().collect();
        if cell.x == 0 { all_directions.remove(&LEFT); }
        else if cell.x == self.max_x as i32 { all_directions.remove(&RIGHT); }
        if cell.y == 0 { all_directions.remove(&UP); }
        else if cell.y == self.max_y as i32 { all_directions.remove(&DOWN); }
        all_directions.into_iter().map(|direction| self.coord_to_index(cell + direction)).collect()
    }
}