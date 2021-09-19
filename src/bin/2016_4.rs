use std::collections::HashMap;

struct RoomInfo {
    name: String,
    sector_id: u32,
    checksum: String,
}

fn main() {
    let input = aoc::read_input();
    let rooms: Vec<RoomInfo> = process_input(input);
    one(&rooms);
    two(&rooms);
}

fn process_input(input: String) -> Vec<RoomInfo> {
    input.lines().map(|l| {
        let others_and_checksum: Vec<&str> = l.split(&['[', ']'][..]).collect();
        let sector_and_name = others_and_checksum[0].rsplitn(2, '-').collect::<Vec<&str>>();
        let sector_id: u32 = sector_and_name[0].parse().unwrap();
        let name = sector_and_name[1].to_string();
        RoomInfo { name, sector_id, checksum: others_and_checksum[1].to_string() }
    }).collect()
}

fn one(rooms: &Vec<RoomInfo>) {
    let s: u32 = rooms.iter().filter(|r| is_real(&r)).map(|r| r.sector_id).sum();
    println!("{}", s);
}

fn two(rooms: &Vec<RoomInfo>) {
    rooms.iter().filter(|r| is_real(&r)).for_each(|r| {
        let rotation = r.sector_id % 26;
        let real_name: String = r.name.chars().map(|c| {
            if c == '-' {' '}
            else {
                let mut new_value = c as u8 + rotation as u8;
                if new_value > ('z' as u8) {
                    new_value = new_value - 'z' as u8 + 'a' as u8 - 1;
                }
                new_value as char
            }
        }).collect();
        println!("{} {}", real_name, r.sector_id);
    });
}

fn process_name(name: &String) -> String {
    let mut freq_map = HashMap::new();
    for c in name.chars().filter(|c| *c != '-') {
        let count = freq_map.entry(c).or_insert(0);
        *count += 1;
    }
    let mut freq_map_group_by_freq = HashMap::new();
    for (c, f) in freq_map {
        let group = freq_map_group_by_freq.entry(f).or_insert(Vec::new());
        group.push(c);
    }
    let mut freqs = freq_map_group_by_freq.keys().collect::<Vec<_>>();
    freqs.sort();
    freqs.iter().rev().flat_map(|&f| {
        let mut cv = freq_map_group_by_freq.get(f).unwrap().clone();
        cv.sort();
        cv
    }).collect()
}

fn is_real(room: &RoomInfo) -> bool {
    process_name(&room.name).starts_with(&room.checksum)
}
