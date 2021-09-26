use std::cell::Cell;
use std::collections::VecDeque;

fn main() {
    let input = "amgozmfv";
    let disk = make_disk(input);
    one(&disk);
    two(&disk);
}

fn make_disk(input: &str) -> Vec<Vec<Cell<char>>> {
    (0..128).into_iter().map(|row| {
        let hash_input: Vec<_> = format!("{}-{}", input, row).chars().collect();
        aoc::knot_hash(hash_input).into_iter().map(|eight| {
            format!("{:b}", eight).chars().map(|c| Cell::new(c)).collect::<Vec<_>>()
        }).collect::<Vec<_>>().join(&[][..])
    }).collect()
}

fn one(disk: &Vec<Vec<Cell<char>>>) {
    let used_squares: usize = disk.iter().map(|r| r.iter().filter(|&s| s.get() == '1').count()).sum();
    println!("{}", used_squares);
}

fn find_left(disk: &Vec<Vec<Cell<char>>>) -> Option<(usize, usize)> {
    for i in 0..disk.len() {
        for j in 0..disk[0].len() {
            if disk[i][j].get() == '1' {
                return Some((j, i));
            }
        }
    }
    None
}

fn spread(disk: &Vec<Vec<Cell<char>>>, coord: (usize, usize)) {
    let mut queue = VecDeque::new();
    disk[coord.1][coord.0].set('2');
    queue.push_back(coord);
    loop {
        match queue.pop_front() {
            None => break,
            Some(current) => {
                let all_four = vec![(current.0 + 1, current.1), (current.0, current.1 + 1),
                                    (current.0 - 1, current.1), (current.0, current.1 - 1)];
                let neighbors: Vec<_> = all_four.iter()
                    .filter(|&&coord| coord.0 >= 0 && coord.0 < disk[0].len() &&
                        coord.1 >= 0 && coord.1 < disk.len() && disk[coord.1][coord.0].get() == '1').collect();
                neighbors.iter().for_each(|&&c| disk[c.1][c.0].set('2'));
                queue.extend(neighbors);
            }
        }
    }
}

fn two(disk: &Vec<Vec<Cell<char>>>) {
    let mut count = 0;
    loop {
        match find_left(disk) {
            None => break,
            Some(coord) => {
                spread(disk, coord);
                count += 1;
            }
        }
    }
    println!("{}", count);
}