use aoc::utils;
use std::collections::{HashSet, HashMap, VecDeque};
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter};
use std::time::Instant;

#[derive(Clone, Eq, PartialEq)]
struct FloorGroups {
    pairs: HashSet<i32>,
    single_gens: HashSet<i32>,
    single_chips: HashSet<i32>,
}

impl FloorGroups {
    fn new() -> FloorGroups {
        FloorGroups {
            pairs: HashSet::new(),
            single_gens: HashSet::new(),
            single_chips: HashSet::new(),
        }
    }
    
    fn make(objects: &HashSet<i32>) -> FloorGroups {
        let mut pairs= HashSet::new();
        let (mut single_gens, mut single_chips): (HashSet<i32>, HashSet<i32>) = objects.iter().partition(|&e| *e > 0);
        let single_gens_clone = single_gens.clone();
        let single_chips_clone = single_chips.clone();
        for gen in single_gens_clone {
            if single_chips_clone.contains(&(-gen)) {
                pairs.insert(gen);
                single_gens.remove(&gen);
                single_chips.remove(&(-gen));
            }
        }
        single_chips = single_chips.iter().map(|c| c.abs()).collect();
        FloorGroups { pairs, single_gens, single_chips }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct FloorView {
    pairs: usize,
    single_gens: usize,
    single_chips: usize,
}

impl Debug for FloorView {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FloorView")
            .field("pairs", &self.pairs)
            .field("gens", &self.single_gens)
            .field("chips", &self.single_chips)
            .finish()
    }
}

impl Hash for FloorView {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pairs.hash(state);
        self.single_gens.hash(state);
        self.single_chips.hash(state);
    }
}

impl FloorView {
    fn new() -> FloorView {
        FloorView {
            pairs: 0,
            single_gens: 0,
            single_chips: 0,
        }
    }

    fn make(groups: &FloorGroups) -> FloorView {
        FloorView {
            pairs: groups.pairs.len(),
            single_gens: groups.single_gens.len(),
            single_chips: groups.single_chips.len(),
        }
    }
}

#[derive(Clone, Eq)]
struct Floor {
    objects: HashSet<i32>,
    groups: FloorGroups,
    view: FloorView,
}

impl Debug for Floor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Floor")
            .field("objects", &self.objects)
            .field("view", &self.view)
            .finish()
    }
}

impl PartialEq for Floor {
    fn eq(&self, other: &Self) -> bool {
        self.objects == other.objects
    }
}

impl Floor {
    fn update(&mut self) {
        self.groups = FloorGroups::make(&self.objects);
        self.view = FloorView::make(&self.groups);
    }

    fn make(objects: HashSet<i32>) -> Floor {
        let mut floor = Floor { objects, groups: FloorGroups::new(), view: FloorView::new() };
        floor.update();
        floor
    }

    fn add(&self, objects: &HashSet<i32>) -> Floor {
        Floor::make(self.objects.union(objects).cloned().collect())
    }
    
    fn subtract(&self, objects: &HashSet<i32>) -> Floor {
        Floor::make(self.objects.difference(objects).cloned().collect())
    }
}

struct Move {
    objects: HashSet<i32>,
    direction: i32,
}

#[derive(Clone, Eq)]
struct State {
    // prev_state: Box<Option<State>>,
    elevator_floor: usize,
    floors: HashMap<usize, Floor>,
}

impl Debug for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("elevator", &self.elevator_floor)
            .field("floors", &self.floors)
            .finish()
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.elevator_floor == other.elevator_floor &&
            self.floors.iter().all(|(i, f)| other.floors.get(i).unwrap().view == f.view)
    }
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elevator_floor.hash(state);
        for k in 1..4 {
            self.floors.get(&k).unwrap().view.hash(state);
        }
    }
}

impl State {
    fn move_to_state(&self, m: Move) -> State {
        let elevator_floor = (self.elevator_floor as i32 + m.direction) as usize;
        let mut floors = self.floors.clone();
        floors.insert(self.elevator_floor, self.floors.get(&self.elevator_floor).unwrap().subtract(&m.objects));
        floors.insert(elevator_floor, self.floors.get(&elevator_floor).unwrap().add(&m.objects));
        State { /*prev_state: Box::new(Some(self.clone())),*/ elevator_floor, floors }
    }

    fn add_state_by_objects(&self, objects: Vec<i32>, direction: i32, states: &mut HashSet<State>) {
        let state = self.move_to_state(Move { objects: objects.into_iter().collect::<HashSet<_>>(), direction });
        states.insert(state);
    }

    fn make_states(&self, direction: i32) -> HashSet<State> {
        let current_floor = self.floors.get(&self.elevator_floor).unwrap();
        let next_floor = self.floors.get(&((self.elevator_floor as i32 + direction) as usize)).unwrap();

        let mut states = HashSet::new();
        if current_floor.view.single_chips == 0 {
            // this floor has no single chip, so there can be some pairs or some single gens or both
            if current_floor.view.pairs == 0 {
                // this floor has no pair, so there are only some single gens
                if next_floor.view.single_chips == 0 {
                    // next floor has no single chip: move 1 or 2 random gens
                    let mut current_floor_objects_iter = current_floor.objects.iter();
                    let first = *current_floor_objects_iter.next().unwrap();
                    self.add_state_by_objects(vec![first], direction, &mut states);
                    if current_floor.view.single_gens > 1 {
                        let second = *current_floor_objects_iter.next().unwrap();
                        self.add_state_by_objects(vec![first, second], direction, &mut states);
                    }
                } else {
                    // next floor has at least one single chip: move 0 or 1 or 2 gens that have correspondent chips in next_floor
                    if next_floor.view.single_chips < 3 {
                        let common: HashSet<i32> = current_floor.groups.single_gens.intersection(&next_floor.groups.single_chips).cloned().collect();
                        if common.len() > 0 {
                            let mut common_iter = common.iter();
                            let first_common = *common_iter.next().unwrap();
                            self.add_state_by_objects(vec![first_common], direction, &mut states);
                            if next_floor.view.single_chips == 1 && current_floor.view.single_gens > 1 {
                                let other_gen = *current_floor.groups.single_gens.iter().find(|&g| *g != first_common).unwrap();
                                self.add_state_by_objects(vec![first_common, other_gen], direction, &mut states);
                            }
                            if common.len() > 1 {
                                let second_common = *common_iter.next().unwrap();
                                self.add_state_by_objects(vec![first_common, second_common], direction, &mut states);
                            }
                        }
                    }
                }
            } else if current_floor.view.single_gens == 0 {
                // this floor has some pairs and no single gen: check next_floor
                let mut current_floor_pairs_iter = current_floor.groups.pairs.iter();
                let first_pair_gen = *current_floor_pairs_iter.next().unwrap();
                let second_pair_gen_option = current_floor_pairs_iter.next().cloned();
                if next_floor.view.single_chips == 0 {
                    self.add_state_by_objects(vec![first_pair_gen, -first_pair_gen], direction, &mut states);
                    if current_floor.view.pairs == 1 {
                        self.add_state_by_objects(vec![first_pair_gen], direction, &mut states);
                    } else if current_floor.view.pairs == 2 {
                        self.add_state_by_objects(vec![first_pair_gen, second_pair_gen_option.unwrap()], direction, &mut states);
                    }
                }
                if next_floor.view.single_gens == 0 && next_floor.view.pairs == 0 {
                    self.add_state_by_objects(vec![-first_pair_gen], direction, &mut states);
                    if current_floor.view.pairs > 1 {
                        self.add_state_by_objects(vec![-first_pair_gen, -second_pair_gen_option.unwrap()], direction, &mut states);
                    }
                }
            } else {
                // this floor has some pairs and some single gens: only single gens or chips in pairs or a pair can be moved
                let mut current_floor_pairs_iter = current_floor.groups.pairs.iter();
                let first_pair_gen = *current_floor_pairs_iter.next().unwrap();
                let mut current_floor_gens_iter = current_floor.groups.single_gens.iter();
                let first_gen = *current_floor_gens_iter.next().unwrap();

                if next_floor.view.single_chips == 0 {
                    self.add_state_by_objects(vec![first_pair_gen, -first_pair_gen], direction, &mut states);
                    self.add_state_by_objects(vec![first_gen], direction, &mut states);
                    if current_floor.view.single_gens > 1 {
                        let second_gen = *current_floor_gens_iter.next().unwrap();
                        self.add_state_by_objects(vec![first_gen, second_gen], direction, &mut states);
                    } else if current_floor.view.pairs == 1 {
                        self.add_state_by_objects(vec![first_pair_gen, first_gen], direction, &mut states);
                    }
                }
                if next_floor.view.single_gens == 0 && next_floor.view.pairs == 0 {
                    self.add_state_by_objects(vec![-first_pair_gen], direction, &mut states);
                    if current_floor.view.pairs > 1 {
                        let second_pair_gen = *current_floor_pairs_iter.next().unwrap();
                        self.add_state_by_objects(vec![-first_pair_gen, -second_pair_gen], direction, &mut states);
                    }
                    if next_floor.view.single_chips < 3 {
                        let common: HashSet<i32> = current_floor.groups.single_gens.intersection(&next_floor.groups.single_chips).cloned().collect();
                        if common.len() > 0 {
                            let mut common_iter = common.iter();
                            let first_common_gen = *common_iter.next().unwrap();
                            self.add_state_by_objects(vec![first_common_gen], direction, &mut states);
                            if next_floor.view.single_chips == 1 && current_floor.view.single_gens > 1 {
                                let other_gen = *current_floor.groups.single_gens.iter().find(|&g| *g != first_common_gen).unwrap();
                                self.add_state_by_objects(vec![first_common_gen, other_gen], direction, &mut states);
                            } else if next_floor.view.single_chips == 1 && current_floor.view.pairs == 1 {
                                let other_gen_in_pair = *current_floor.groups.pairs.iter().next().unwrap();
                                self.add_state_by_objects(vec![first_common_gen, other_gen_in_pair], direction, &mut states);
                            }
                            if common.len() > 1 {
                                let second_common_gen = *common_iter.next().unwrap();
                                self.add_state_by_objects(vec![first_common_gen, second_common_gen], direction, &mut states);
                            }
                        }
                    }
                }
            }
        } else {
            // this floor has at least one single chip, so there is no pair and no single gen
            if next_floor.view.single_gens == 0 && next_floor.view.pairs == 0 {
                // next floor has no single gen and no pair: move 1 or 2 random chips
                let mut current_floor_objects_iter = current_floor.objects.iter();
                let first = *current_floor_objects_iter.next().unwrap();
                self.add_state_by_objects(vec![first], direction, &mut states);
                if current_floor.view.single_chips > 1 {
                    let second = *current_floor_objects_iter.next().unwrap();
                    self.add_state_by_objects(vec![first, second], direction, &mut states);
                }
            } else if next_floor.view.single_gens > 0 {
                // next floor has at least one single gen: move 0 or 1 or 2 chips that have correspondent gens in next_floor
                let common: HashSet<i32> = current_floor.groups.single_chips.intersection(&next_floor.groups.single_gens).cloned().collect();
                if common.len() > 0 {
                    let mut common_iter = common.iter();
                    let first_common = -*common_iter.next().unwrap();
                    self.add_state_by_objects(vec![first_common], direction, &mut states);
                    if common.len() > 1 {
                        let second_common = -*common_iter.next().unwrap();
                        self.add_state_by_objects(vec![first_common, second_common], direction, &mut states);
                    }
                }
            }
        }
        states
    }

    fn next_states(&self) -> HashSet<State> {
        let mut up_states = if self.elevator_floor < 4 { self.make_states(1) } else { HashSet::new() };
        let down_states = if self.elevator_floor > 1 { self.make_states(-1) } else { HashSet::new() };
        up_states.extend(down_states.into_iter());
        up_states
    }

    fn is_final(&self) -> bool {
        [1,2,3].iter().all(|floor| self.floors.get(floor).unwrap().objects.len() == 0)
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
    State { /*prev_state: Box::new(None),*/ elevator_floor: 1, floors }
}

fn go(initial: State) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((initial.clone(), 0usize));
    let mut passed_states = HashSet::new();
    passed_states.insert(initial);
    // let mut last_state: State;
    let step_count: usize = loop {
        let (state, step) = queue.pop_front().unwrap();
        let next_states: HashSet<_> = state.next_states().into_iter().filter(|s| !passed_states.contains(s)).collect();
        // if step < 2 {
        //     println!("next_states: {:?}", next_states);
        //     println!("next states len: {}", next_states.len());
        //     println!("passed_states: {:?}", passed_states);
        //     println!("{}", step+1);
        //     println!("---");
        // }
        if let Some(_) = next_states.iter().find(|&s| s.is_final()) {
            // last_state = (*s).clone();
            break step+1;
        }
        next_states.into_iter().for_each(|s| {
            queue.push_back((s.clone(), step+1));
            passed_states.insert(s);
        });
    };
    // loop {
    //     println!("{:?}", last_state);
    //     match *last_state.prev_state {
    //         Some(s) => last_state = s,
    //         None => break
    //     }
    // }
    step_count
}

fn main() {
    let input = utils::read_input();
    let lines: Vec<&str> = input.lines().collect();
    let input_state = read_input(&lines);
    // assert_eq!(initial_state.floors.get(&1).unwrap().view.pairs, 1);
    // assert_eq!(initial_state.floors.get(&2).unwrap().view.single_gens, 4);
    // assert_eq!(initial_state.floors.get(&3).unwrap().view.single_chips, 4);
    // assert_eq!(initial_state.floors.get(&4).unwrap().view.single_gens, 0);
    // println!("{:?}", input_state);
    let now = Instant::now();
    one(&input_state);
    println!("first part: {}", now.elapsed().as_millis());
    let now = Instant::now();
    two(&input_state);
    println!("second part: {}", now.elapsed().as_millis());
}

fn one(input_state: &State) {
    let step_count = go(input_state.clone());
    println!("{}", step_count);
}

fn two(input_state: &State) {
    let mut updated_state = input_state.clone();
    let first_floor = updated_state.floors.get_mut(&1).unwrap();
    let objects: HashSet<_> = vec![100, -100, 200, -200].into_iter().collect();
    *first_floor = first_floor.add(&objects);
    assert_eq!(updated_state.floors.get(&1).unwrap().view.pairs, 3);
    // println!("{:?}", updated_state.floors.get(&1).unwrap());
    let step_count = go(updated_state);
    println!("{}", step_count);
}