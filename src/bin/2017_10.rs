fn main() {
    let input = aoc::read_input();
    let lengths: Vec<usize> = input.trim().split(",").map(|s| s.parse().unwrap()).collect();
    one(lengths);
    let chars: Vec<_> = input.trim().chars().collect();
    two(chars);
}

fn knot_hash(lengths: &Vec<usize>, list: &mut Vec<i32>, current_position: &mut usize, skip_size: &mut usize) {
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

fn one(lengths: Vec<usize>) {
    let mut list = (0..=255).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    knot_hash(&lengths, &mut list, &mut current_position, &mut skip_size);
    println!("{}", list[0] * list[1]);
}

fn two(chars: Vec<char>) {
    let mut lengths: Vec<_> = chars.iter().map(|c| *c as usize).collect();
    lengths.extend(vec![17, 31, 73, 47, 23]);
    let mut list = (0..=255).into_iter().collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        knot_hash(&lengths, &mut list, &mut current_position, &mut skip_size);
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
    for n in dense_hash {
        print!("{:02x}", n);
    }
}