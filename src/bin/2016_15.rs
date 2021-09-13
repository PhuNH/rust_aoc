use aoc::utils;

#[derive(Debug)]
struct Disk {
    number: u32,
    positions: u32,
    pos_0: u32,
}

fn process_input(lines: Vec<&str>) -> Vec<Disk> {
    lines.iter().map(|&l| {
        let words = l.split_whitespace().collect::<Vec<_>>();
        let number = words[1].chars().skip(1).collect::<String>().parse().unwrap();
        let positions = words[3].parse().unwrap();
        let pos_0 = words[11].split('.').next().unwrap().parse().unwrap();
        Disk { number, positions, pos_0 }
    }).collect()
}

fn find_t(disks: &Vec<Disk>) -> u32 {
    let max_disk = disks.iter().max_by_key(|&d| d.positions).unwrap();
    let mut i = 1u32;
    loop {
        let t = i * max_disk.positions - (max_disk.number + max_disk.pos_0);
        let check = disks.iter().filter(|&d| d.number != max_disk.number).all(|d| {
            (t + d.number + d.pos_0) % d.positions == 0
        });
        if check {
            break t;
        }
        i += 1;
    }
}

fn main() {
    let input = utils::read_input();
    let lines = input.lines().collect::<Vec<_>>();
    let disks = process_input(lines);
    // println!("{:?}", disks);
    one(&disks);
    two(disks);
}

fn one(disks: &Vec<Disk>) {
    println!("{}", find_t(disks));
}

fn two(mut disks: Vec<Disk>) {
    disks.push(Disk { number: disks.len() as u32 + 1, positions: 11, pos_0: 0 });
    println!("{}", find_t(&disks));
}