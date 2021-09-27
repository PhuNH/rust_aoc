use std::collections::HashMap;

struct Program<'a> {
    name: &'a str,
    weight: u32,
    parent: Option<&'a str>,
    children: Vec<&'a str>,
}

fn process_name_weight(name_weight: &str) -> (&str, u32) {
    let mut name_weight_iter = name_weight.split_whitespace();
    let name = name_weight_iter.next().unwrap();
    let weight = name_weight_iter.next().unwrap().split(&['(', ')'][..]).nth(1).unwrap().parse().unwrap();
    (name, weight)
}

fn process_input(lines: Vec<&str>) -> HashMap<&str, Program> {
    let mut programs: Vec<(_, _)> = lines.into_iter().map(|l| {
        if l.contains("->") {
            let parts: Vec<_> = l.split("->").collect();
            let (name, weight) = process_name_weight(parts[0]);
            let children: Vec<_> = parts[1].trim().split(',').map(|p| p.trim()).collect();
            let program = Program { name, weight, parent: None, children };
            (name, program)
        } else {
            let (name, weight) = process_name_weight(l);
            let program = Program { name, weight, parent: None, children: Vec::new() };
            (name, program)
        }
    }).collect();
    for p in 0..programs.len() {
        for i in 0..programs.len() {
            if programs[i].1.children.contains(&programs[p].1.name) {
                programs[p].1.parent = Some(programs[i].0);
            }
        }
    }
    programs.into_iter().collect()
}

fn main() {
    let input = aoc::read_input();
    let lines: Vec<_> = input.lines().collect();
    let programs = process_input(lines);
    let root_name = one(&programs);
    two(&programs, root_name);
}

fn one<'a>(programs: &'a HashMap<&str, Program>) -> &'a str {
    let root_name = programs.values().find(|p| p.parent.is_none()).unwrap().name;
    println!("{}", root_name);
    root_name
}

fn find_tower_weight(programs: &HashMap<&str, Program>, tower_weight: &mut HashMap<String, u32>, name: &str) -> u32 {
    if tower_weight.contains_key(name) { *tower_weight.get(name).unwrap() }
    else {
        let program = programs.get(name).unwrap();
        let children_weights: Vec<_> = program.children.iter()
            .map(|child_name|
                find_tower_weight(programs, tower_weight, child_name)
            ).collect();
        let weight = program.weight + children_weights.iter().sum::<u32>();
        tower_weight.insert(name.to_string(), weight);
        weight
    }
}

fn two(programs: &HashMap<&str, Program>, root_name: &str) {
    let mut tower_weights = HashMap::new();
    let mut current = root_name;
    find_tower_weight(programs, &mut tower_weights, current);
    let mut diff = 0;
    let mut next_diff = 0;

    loop {
        let program = programs.get(current).unwrap();
        let children_weights: Vec<_> = program.children.iter()
            .map(|c|
                (c, find_tower_weight(programs, &mut tower_weights, c))
            ).collect();
        let children_avg = children_weights.iter().map(|(_, w)| *w).sum::<u32>() as f32 / children_weights.len() as f32;
        if let Some((unbalanced_name, unbalanced_weight)) = children_weights.iter().find(|(_, w)| *w as f32 != children_avg) {
            next_diff = (children_weights.iter().map(|(_, w)| *w).sum::<u32>() - *unbalanced_weight) as i32 / (children_weights.len() as i32 - 1) - *unbalanced_weight as i32;
            current = unbalanced_name;
        }
        if next_diff != 0 {
            diff = next_diff;
            next_diff = 0;
        } else {
            break;
        }
    }
    println!("{}", programs.get(current).unwrap().weight as i32 + diff);
}