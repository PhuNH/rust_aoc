use md5;

fn has_five(hash: &String, char_three: &char) -> bool {
    let hash_chars = hash.chars().collect::<Vec<_>>();
    hash_chars[..hash_chars.len()-4].iter().enumerate().any(|(i, c)| {
        *c == *char_three && (1..5).all(|k| hash_chars[k as usize + i] == *c)
    })
}

fn is_key(index: u32, stream: &mut Vec<String>, salt: &str, hash_func: fn(&str, &String) -> String) -> bool {
    let hash = {
        if stream.len() <= index as usize {
            for i in (stream.len() as u32)..=index {
                stream.push(hash_func(salt, &i.to_string()));
            }
        }
        &stream[index as usize]
    };
    let hash_chars = hash.chars().collect::<Vec<_>>();
    let three = hash_chars[..hash_chars.len()-2].iter().enumerate().find(|(i, &c)| {
        hash_chars[i+1] == c && hash_chars[i+2] == c
    }).map(|(_, c)| c);
    match three {
        None => false,
        Some(c3) => {
            let last_1k_index = index as usize + 1001;
            let last_index = stream.len().min(last_1k_index);
            let with_existing_hashes = stream[(index as usize+1)..last_index].iter().any(|h| {
                has_five(h, c3)
            });
            if with_existing_hashes { with_existing_hashes }
            else if stream.len() >= last_1k_index { false }
            else {
                (stream.len()..last_1k_index).any(|i| {
                    let new_hash = hash_func(salt, &i.to_string());
                    let result = has_five(&new_hash, c3);
                    stream.push(new_hash);
                    result
                })
            }
        }
    }
    // three.is_some()
}

fn find_64th(input: &str, hash_func: fn(&str, &String) -> String) {
    let mut key_indices = Vec::new();
    let mut index = 0;
    let mut stream = Vec::new();
    let the_64th_index = loop {
        if is_key(index, &mut stream, input, hash_func) {
            key_indices.push(index);
        }
        if key_indices.len() == 64 {
            break key_indices[63];
        }
        index += 1;
    };
    println!("{:?}", the_64th_index);
}

fn main() {
    let input = "yjdafjpo";
    one(input);
    two(input);
}

fn one(input: &str) {
    find_64th(input, aoc::hex_md5);
}

fn key_stretching(salt: &str, additional: &String) -> String {
    let mut hash = aoc::hex_md5(salt, additional);
    for _ in 0..2016 {
        hash = format!("{:x}", md5::compute(hash));
    }
    hash
}

fn two(input: &str) {
    find_64th(input, key_stretching);
}