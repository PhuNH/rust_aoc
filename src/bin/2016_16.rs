fn gen_data(mut data: Vec<char>) -> Vec<char> {
    let b = data.iter().rev().map(|c| match c {
        '0' => '1',
        _ => '0'
    }).collect::<Vec<_>>();
    data.push('0');
    data.extend(b);
    data
}

fn fill_disk(state: &str, length: usize) -> Vec<char> {
    let mut result = state.chars().collect::<Vec<_>>();
    loop {
        if result.len() >= length {
            result.truncate(length);
            break result;
        }
        result = gen_data(result);
    }
}

fn gen_checksum(data: Vec<char>) -> String {
    let mut checksum = data;
    let mut first= ' ';
    let mut temp = Vec::new();
    loop {
        for (i, c) in checksum.iter().enumerate() {
            if i % 2 == 0 { first = *c; }
            else { temp.push(if *c == first {'1'} else {'0'}); }
        }
        if temp.len() % 2 == 1 {
            break temp.into_iter().collect();
        }
        checksum = temp;
        temp = Vec::new();
    }
}

fn main() {
    let input = "01110110101001000";
    one(input);
    two(input);
}

fn one(input: &str) {
    let length = 272usize;
    let disk = fill_disk(input, length);
    println!("{}", gen_checksum(disk));
}

fn two(input: &str) {
    let length = 35651584usize;
    let disk = fill_disk(input, length);
    println!("{}", gen_checksum(disk));
}