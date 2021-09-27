fn main() {
    let door_id = "ffykfhsq";
    one(door_id);
    two(door_id);
}

fn one(door_id: &str) {
    let mut index = 0;
    let mut digest;
    for _ in 0..8 {
        loop {
            digest = aoc::hex_md5(door_id, &index.to_string());
            index += 1;
            if digest.starts_with("00000") { break; }
        }
        print!("{}", digest.chars().nth(5).unwrap());
    }
    println!();
}

fn two(door_id: &str) {
    let mut index = 0;
    let mut digest;
    let mut pos: u8;
    let mut value;
    let mut result = ['_'; 8];
    while result.contains(&'_') {
        loop {
            digest = aoc::hex_md5(door_id, &index.to_string());
            index += 1;
            let mut char_iter = digest.chars().skip(5);
            pos = char_iter.next().unwrap() as u8;
            value = char_iter.next().unwrap();
            if digest.starts_with("00000") && pos >= 48 && pos <= 55 { break; }
        }
        if result[(pos - 48) as usize] == '_' {
            result[(pos - 48) as usize] = value;
        }
    }
    println!("{}", result.iter().collect::<String>());
}
