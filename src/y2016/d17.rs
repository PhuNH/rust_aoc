use std::collections::VecDeque;

type Step = char;
type Door = bool;

#[derive(Clone)]
struct State {
    coord: (u32, u32),
    steps: Vec<char>,
    doors: Vec<bool>,
}

impl State {
    fn check_doors(passcode: &str, path: &Vec<Step>) -> Vec<Door> {
        let hash = aoc::hex_md5(passcode, &path.iter().collect::<String>());
        hash.chars().take(4).map(|c| c > 'a' && c <= 'f').collect()
    }

    fn from(passcode: &str) -> State {
        let coord = (0u32, 0u32);
        let steps = Vec::new();
        let doors = State::check_doors(passcode, &steps);
        State { coord, steps, doors }
    }

    fn step(&self, step: Step, passcode: &str) -> State {
        let coord = match step {
            'U' => (self.coord.0, self.coord.1-1),
            'D' => (self.coord.0, self.coord.1+1),
            'L' => (self.coord.0-1, self.coord.1),
            _ => (self.coord.0+1, self.coord.1),
        };
        let mut steps = self.steps.clone();
        steps.push(step);
        let doors = State::check_doors(passcode, &steps);
        State { coord, steps, doors }
    }

    fn next_states(&self, passcode: &str) -> Vec<State> {
        let possible_steps = self.doors.iter().enumerate().map(|(i, d)| {
            if !d { '_' }
            else if i == 0 && self.coord.1 > 0 { 'U' }
            else if i == 1 && self.coord.1 < 3 { 'D' }
            else if i == 2 && self.coord.0 > 0 { 'L' }
            else if i == 3 && self.coord.0 < 3 { 'R' }
            else { '_' }
        }).filter(|s| *s != '_').collect::<Vec<_>>();
        possible_steps.into_iter().map(|s| self.step(s, passcode)).collect()
    }

    fn is_final(&self) -> bool {
        self.coord == (3, 3)
    }

    fn get_path(&self) -> String {
        self.steps.iter().collect::<String>()
    }
}

fn shortest_path(passcode: &str) -> String {
    let mut queue = VecDeque::new();
    queue.push_back(State::from(passcode));
    let shortest_path = loop {
        let current_state = queue.pop_front().unwrap();
        let next_states = current_state.next_states(passcode);
        if let Some(s) = next_states.iter().find(|&s| s.is_final()) {
            break s.get_path();
        }
        queue.extend(next_states.iter().cloned());
    };
    shortest_path
}

fn longest_length(passcode: &str) -> usize {
    let mut longest_length = 0usize;
    let mut stack = Vec::new();
    stack.push(State::from(passcode));
    loop {
        if stack.is_empty() { break; }
        let current_state = stack.pop().unwrap();
        current_state.next_states(passcode).into_iter().for_each(|s| {
            if s.is_final() {
                if s.steps.len() > longest_length { longest_length = s.steps.len(); }
            } else { stack.push(s); }
        });
    }
    longest_length
}

pub fn main() {
    let input = "dmypynyp";
    one(input);
    two(input);
}

fn one(input: &str) {
    println!("{}", shortest_path(input));
}

fn two(input: &str) {
    println!("{}", longest_length(input));
}