use md5;

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
            digest = hash(door_id, index);
            index += 1;
            if digest.starts_with("00000") { break; }
        }
        print!("{}", *digest.into_bytes().get(5).unwrap() as char);
    }
    println!();
}

fn two(door_id: &str) {
    let mut index = 0;
    let mut digest;
    let mut pos;
    let mut result = ['_'; 8];
    while result.contains(&'_') {
        loop {
            digest = hash(door_id, index).into_bytes();
            // println!("{:?}", digest);
            index += 1;
            pos = digest[5];
            if digest.starts_with(&[48; 5]) && pos >= 48 && pos <= 55 { break; }
        }
        // println!("{} {} {}", index, pos - 48, digest[6] as char);
        if result[(pos - 48) as usize] == '_' {
            result[(pos - 48) as usize] = digest[6] as char;
        }
    }
    println!("{}", result.iter().collect::<String>());
}

fn hash(door_id: &str, index: u32) -> String {
    format!("{:x}", md5::compute((door_id.to_owned() + &index.to_string()).as_bytes()))
}
