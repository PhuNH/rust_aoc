pub fn main() {
    let input = aoc::read_input(module_path!());
    let input = input.trim();
    one_and_two(input);
}

fn one_and_two(input: &str) {
    let mut scores = Vec::new();
    let mut current_score = 0;
    let mut group_stack = Vec::new();
    let mut in_garbage = false;
    let mut to_skip = false;
    let mut garbage_count = 0;

    for c in input.chars() {
        if !in_garbage {
            if c == '{' {
                current_score += 1;
                group_stack.push(current_score);
            } else if c == '}' {
                current_score -= 1;
                scores.push(group_stack.pop().unwrap());
            } else if c == '<' {
                in_garbage = true;
            }
        } else {
            if to_skip {
                to_skip = false;
            } else {
                if c == '>' {
                    in_garbage = false;
                } else if c == '!' {
                    to_skip = true;
                } else {
                    garbage_count += 1;
                }
            }
        }
    }
    println!("{}, {}", scores.iter().sum::<u32>(), garbage_count);
}