pub mod grid;

use std::fs;
use std::str::from_utf8;
use md5;

pub fn read_input(module_path: &str) -> String {
    let parts: Vec<&str> = module_path.rsplit("::").take(2).map(|m|
        from_utf8(&m.as_bytes()[1..]).unwrap()).collect();
    fs::read_to_string(format!("input/{}/{}", parts[1], parts[0])).unwrap()
}

pub fn hex_md5(salt: &str, additional: &String) -> String {
    format!("{:x}", md5::compute(salt.to_owned() + additional))
}

pub fn is_prime(n: u32) -> bool {
    let sqrt = (n as f64).sqrt() as u32;
    if n % 2 == 0 { false }
    else { (3..=sqrt).step_by(2).all(|d| n % d != 0) }
}

pub fn factorial(n: u32) -> u32 {
    if n == 2 { 2 }
    else { n * factorial(n-1) }
}

pub fn knot_hash_round(lengths: &Vec<usize>, list: &mut Vec<i32>, current_position: &mut usize, skip_size: &mut usize) {
    let list_len = list.len();

    for length in lengths {
        if *current_position + length >= list_len {
            list.rotate_left(*current_position + length - list_len);
            let mut temp = list[0..(list.len() - length)].to_vec();
            temp.extend(list[(list.len() - length)..].iter().rev());
            *list = temp;
            list.rotate_right(*current_position + length - list_len);
        } else {
            let mut temp = list[0..*current_position].to_vec();
            temp.extend(list[*current_position..(*current_position + length)].iter().rev());
            temp.extend(list[(*current_position + length)..].iter());
            *list = temp;
        }
        *current_position = (*current_position + length + *skip_size) % list_len;
        *skip_size += 1;
    }
}

pub fn knot_hash(chars: Vec<char>) -> Vec<i32> {
    let mut lengths: Vec<_> = chars.iter().map(|c| *c as usize).collect();
    lengths.extend(vec![17, 31, 73, 47, 23]);
    let mut list = (0..=255).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        knot_hash_round(&lengths, &mut list, &mut current_position, &mut skip_size);
    }
    let mut dense_hash = Vec::new();
    let mut temp = 0;
    for i in 0..256 {
        if i % 16 == 0 {
            temp = list[i];
        } else {
            temp ^= list[i];
            if i % 16 == 15 {
                dense_hash.push(temp);
            }
        }
    }
    dense_hash
}