use std::convert::TryInto;

fn main() {
    let input = aoc::utils::read_input();
    let lines: Vec<&str> = input.lines().collect();
    one(&lines);
    two(&lines);
}

fn one(lines: &Vec<&str>) {
    do_part(&lines, do_char);
}

fn two(lines: &Vec<&str>) {
    do_part(&lines, do_char_2);
}

fn do_part(lines: &Vec<&str>, do_char: fn(i8, &char) -> i8) {
    let mut num = 5;
    for l in lines {
        num = do_line(num, &l, do_char);
        print!("{}", char::from_digit(num.try_into().unwrap(), 16).unwrap());
    }
    println!();
}

fn do_line(mut num: i8, instr: &str, do_char: fn(i8, &char) -> i8) -> i8 {
    for c in instr.chars() {
        num = do_char(num, &c);
    }
    num
}

fn do_char(num: i8, c: &char) -> i8 {
    match c {
        'U' => if num - 3 <= 0 { num } else { num - 3 },
        'D' => if num + 3 >= 10 { num } else { num + 3 },
        'L' => if num % 3 == 1 { num } else { num - 1 },
        _ => if num % 3 == 0 { num } else { num + 1 },
    }
}

fn do_char_2(num: i8, c: &char) -> i8 {
    match num {
        1 => if *c == 'D' {3} else {1},
        5 => if *c == 'R' {6} else {5},
        9 => if *c == 'L' {8} else {9},
        13 => if *c == 'U' {11} else {13},
        _ => match c {
            'U' => if num == 2 || num == 4 {num} else if num == 3 {1} else {num-4},
            'D' => if num == 10 || num == 12 {num} else if num == 11 {13} else {num+4},
            'L' => if num == 2 || num == 10 {num} else {num-1},
            _ => if num == 4 || num == 12 {num} else {num+1},
        }
    }
}
