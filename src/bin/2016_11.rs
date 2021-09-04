use aoc::utils;
use itertools::Itertools;
use std::collections::{HashSet, HashMap, VecDeque};

#[derive(Debug)]
struct FloorGroups {
    pairs: HashSet<i32>,
    single_gens: HashSet<i32>,
    single_chips: HashSet<i32>,
}

impl FloorGroups {
    fn make(objects: &HashSet<i32>) -> FloorGroups {
        let (mut pairs, mut single_gens, mut single_chips) = (HashSet::new(), HashSet::new(), HashSet::new());
        for (key, vals) in &objects.iter().cloned().group_by(|v| v.abs()) {
            let group: Vec<i32> = vals.collect();
            if group.len() == 2 { pairs.insert(key) }
            else if group[0] > 0 { single_gens.insert(key) }
            else { single_chips.insert(key) }
        }
        FloorGroups { pairs, single_gens, single_chips }
    }
}

#[derive(Debug)]
struct FloorView {
    pairs: usize,
    single_gens: usize,
    single_chips: usize,
}

impl FloorView {
    fn is_valid(&self) -> bool {
        self.single_chips == 0 || (self.single_gens == 0 && self.pairs == 0)
    }

    fn make(groups: &FloorGroups) -> FloorView {
        FloorView {
            pairs: groups.pairs.len(),
            single_gens: groups.single_gens.len(),
            single_chips: groups.single_chips.len(),
        }
    }
}

impl PartialEq for FloorView {
    fn eq(&self, other: &Self) -> bool {
        self.pairs == other.pairs && self.single_gens == other.single_gens && self.single_chips == other.single_chips
    }
}

#[derive(Debug)]
struct Floor {
    objects: HashSet<i32>,
    groups: FloorGroups,
    view: FloorView,
}

impl Floor {
    fn make(objects: HashSet<i32>) -> Floor {
        let groups = FloorGroups::make(&objects);
        let view = FloorView::make(&groups);
        Floor { objects, groups, view }
    }
}

enum Direction {
    Up(i32),
    Down(i32)
}

const UP: Direction = Direction::Up(1);
const DOWN: Direction = Direction::Down(-1);

struct Move {
    movees: HashSet<i32>,
    direction: Direction,
}

#[derive(Debug)]
struct State {
    elevator_floor: usize,
    floors: HashMap<usize, Floor>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator_floor == other.elevator_floor &&
            self.floors.iter().all(|(i, f)| other.floors.get(i).unwrap().view == f.view)
    }
}

impl State {
    fn move_to_state(&self, m: Move) -> State {
        let elevator_floor = self.elevator_floor + m.direction;
        let current_floor = self.floors
    }

    fn make_up_states(&self) -> HashSet<State> {
        let current_floor = self.floors.get(&self.elevator_floor).unwrap();
        let next_floor = self.floors.get(&(self.elevator_floor+1)).unwrap();

        if current_floor.view.single_chips == 0 {
            // this floor has no single chip, so there can be some pairs or some single gens or both
            if current_floor.view.pairs == 0 {
                // this floor has no pair, so there are only some single gens
                if next_floor.view.single_chips == 0 {
                    // next floor has no single chip: TODO move 1 or 2 random gens
                } else {
                    // next floor has at least one single chip: TODO move 0 or 1 or 2 gens that have correspondent chips in next_floor
                }
            } else if current_floor.view.single_gens == 0 {
                // this floor has some pairs and no single gen: TODO check next_floor
            } else {
                // this floor has some pairs and some single gens
                // only single gens and chips in pairs can be moved
            }
        } else {
            // this floor has at least one single chip, so there is no pair and no single gen
            if next_floor.view.single_gens == 0 && next_floor.view.pairs != 0 {
                // next floor has no single gen and some pairs: cannot move anything
                HashSet::new()
            } else if next_floor.view.single_gens == 0 {
                // next floor has no single gen and no pair: TODO move 1 or 2 random chips
            } else {
                // next floor has at least one single gen: TODO move 0 or 1 or 2 chips that have correspondent gens in next_floor
            }
        }
    }

    fn make_down_states(&self) -> HashSet<State> {

    }

    fn next_states(&self) -> HashSet<State> {
        let up_states = if self.elevator_floor < 4 { self.make_up_states() } else { HashSet::new() };
        let down_states = if self.elevator_floor > 1 { self.make_down_states() } else { HashSet::new() };
        up_states.union(&down_states).collect()
    }
}

fn read_input(lines: &Vec<&str>) -> State {
    let mut name_map: HashMap<&str, u32> = HashMap::new();
    let floors: HashMap<usize, Floor> = lines.iter().enumerate().map(|(number, l)| {
        let words: Vec<&str> = l.split(' ').collect();
        let mut objects: HashSet<i32> = HashSet::new();
        words.iter().enumerate().for_each(|(j, w)| {
            let map_len = name_map.len();
            if *w == "a" {
                if words[j+1].contains('-') {
                    let name = words[j+1].split('-').next().unwrap();
                    let object_val = name_map.entry(name).or_insert((map_len+1) as u32);
                    objects.insert(-(*object_val as i32));
                } else {
                    let object_val = name_map.entry(words[j+1]).or_insert((map_len+1) as u32);
                    objects.insert(*object_val as i32);
                }
            }
        });
        (number+1, Floor::make(objects))
    }).collect();
    State { elevator_floor: 1, floors }
}

fn main() {
    let input = utils::read_input();
    let lines: Vec<&str> = input.lines().collect();
    let initial_state = read_input(&lines);
    assert_eq!(initial_state.floors.get(&1).unwrap().view.pairs, 1);
    assert_eq!(initial_state.floors.get(&2).unwrap().view.single_gens, 4);
    assert_eq!(initial_state.floors.get(&3).unwrap().view.single_chips, 4);
    assert_eq!(initial_state.floors.get(&4).unwrap().view.single_gens, 0);
    println!("{:?}", initial_state);
    one();
    two();
}

fn one() {
    println!();
}

fn two() {
    println!();
}