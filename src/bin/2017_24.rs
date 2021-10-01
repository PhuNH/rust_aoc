use std::collections::HashSet;

struct Component {
    ports: Vec<u32>,
}

impl Component {
    fn strength(&self) -> u32 {
        self.ports.iter().sum()
    }
}

struct SearchUnit {
    bridge: Vec<usize>,
    available: HashSet<usize>,
    tip_port: u32,
    strength: u32,
}

fn main() {
    let input = aoc::read_input();
    let components: Vec<_> = input.lines().map(|l| {
        let ports: Vec<_> = l.trim().split('/').map(|p| p.parse::<u32>().unwrap()).collect();
        Component { ports }
    }).collect();
    one_and_two(&components);
}

fn one_and_two(components: &Vec<Component>) {
    let mut search_stack = Vec::new();
    let mut strongest = 0;
    let mut longest = 0;
    let mut strength_of_longest = 0;
    let mut current_unit = SearchUnit {
        bridge: Vec::new(),
        available: (0..components.len()).collect(),
        tip_port: 0,
        strength: 0,
    };
    search_stack.push(current_unit);

    loop {
        if search_stack.is_empty() { break; }

        current_unit = search_stack.pop().unwrap();
        if current_unit.strength > strongest {
            strongest = current_unit.strength;
        }
        if current_unit.bridge.len() > longest {
            longest = current_unit.bridge.len();
            strength_of_longest = current_unit.strength;
        } else if current_unit.bridge.len() == longest {
            if current_unit.strength > strength_of_longest {
                strength_of_longest = current_unit.strength;
            }
        }
        let with_port: Vec<_> = current_unit.available.iter()
            .filter(|&&i| components[i].ports.contains(&current_unit.tip_port)).cloned()
            .collect();
        let units: Vec<_> = with_port.into_iter().map(|i| {
            let mut bridge = current_unit.bridge.clone();
            bridge.push(i);
            let mut available = current_unit.available.clone();
            available.remove(&i);
            let tip_port = components[i].strength() - current_unit.tip_port;
            let strength = current_unit.strength + components[i].strength();
            SearchUnit { bridge, available, tip_port, strength }
        }).collect();
        search_stack.extend(units);
    }

    println!("{} {}", strongest, strength_of_longest);
}