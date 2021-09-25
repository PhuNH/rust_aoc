use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = aoc::read_input();
    let connections: HashMap<_, _> = input.trim().lines().map(|l| {
        let parts: Vec<_> = l.split("<->").collect();
        let program: u32 = parts[0].trim().parse().unwrap();
        let connected: Vec<u32> = parts[1].split(',').map(|p| p.trim().parse().unwrap()).collect();
        (program, connected)
    }).collect();
    let group_0 = one(&connections);
    two(&connections, group_0);
}

fn find_group(connections: &HashMap<u32, Vec<u32>>, group_start: u32) -> HashSet<u32> {
    let mut group = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(group_start);
    loop {
        match queue.pop_front() {
            None => { break; }
            Some(current) => {
                let connected: Vec<_> = connections.get(&current).unwrap().iter().filter(|&p| !group.contains(p)).collect();
                queue.extend(connected);
                group.insert(current);
            }
        }
    }
    group
}

fn one(connections: &HashMap<u32, Vec<u32>>) -> HashSet<u32> {
    let group_0 = find_group(connections, 0);
    println!("{}", group_0.len());
    group_0
}

fn two(connections: &HashMap<u32, Vec<u32>>, group_0: HashSet<u32>) {
    let mut groups = vec![group_0];
    loop {
        match connections.keys().find(|&p| groups.iter().all(|g| !g.contains(p))) {
            None => break,
            Some(p) => {
                let group = find_group(connections, *p);
                groups.push(group);
            }
        }
    }
    println!("{}", groups.len());
}