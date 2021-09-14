use aoc::utils;

fn swap_positions(input: &str, x: usize, y: usize) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    let temp = chars[x];
    chars[x] = chars[y];
    chars[y] = temp;
    chars.iter().collect()
}

fn swap_letters(input: &str, x: char, y: char) -> String {
    input.chars().map(|c| {
        if c == x { y }
        else if c == y { x }
        else { c }
    }).collect()
}

fn rotate_by_steps(input: &str, x: usize, direction: bool) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    if direction {
        chars.rotate_left(x);
    } else {
        chars.rotate_right(x);
    }
    chars.iter().collect()
}

fn rotate_by_letter(input: &str, x: char) -> String {
    let mut index = input.find(x).unwrap() + 1;
    if index >= 5 { index += 1; }
    if index > input.len() { index = index % input.len(); }
    let mut chars: Vec<_> = input.chars().collect();
    chars.rotate_right(index);
    chars.iter().collect()
}

fn reverse(input: &str, x: usize, y: usize) -> String {
    let chars: Vec<_> = input.chars().collect();
    let mid = chars[x..=y].iter().cloned().rev().collect::<String>();
    if y == input.len() {
        format!("{}{}", chars[..x].iter().collect::<String>(), mid)
    } else {
        format!("{}{}{}", chars[..x].iter().collect::<String>(), mid, chars[y+1..].iter().collect::<String>())
    }
}

fn move_by_positions(input: &str, x: usize, y: usize) -> String {
    let mut chars: Vec<_> = input.chars().collect();
    let c = chars.remove(x);
    chars.insert(y, c);
    chars.iter().collect()
}

fn scramble(password: &str, lines: &Vec<&str>) -> String {
    let mut result = password.to_owned();
    lines.iter().for_each(|&l| {
        let words = l.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "swap" => {
                if words[1] == "position" {
                    let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[5].parse().unwrap());
                    result = swap_positions(&result, x, y);
                } else {
                    let (x, y) = (words[2].chars().next().unwrap(), words[5].chars().next().unwrap());
                    result = swap_letters(&result, x, y);
                }
            },
            "rotate" => {
                match words[1] {
                    "left" => {
                        let x: usize = words[2].parse().unwrap();
                        result = rotate_by_steps(&result, x, true);
                    },
                    "right" => {
                        let x: usize = words[2].parse().unwrap();
                        result = rotate_by_steps(&result, x, false);
                    },
                    _ => {
                        let x = words[6].chars().next().unwrap();
                        result = rotate_by_letter(&result, x);
                    },
                }
            },
            "reverse" => {
                let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[4].parse().unwrap());
                result = reverse(&result, x, y);
            },
            _ => {
                let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[5].parse().unwrap());
                result = move_by_positions(&result, x, y);
            }
        }
    });
    result
}

fn unrotate_by_letter(scrambled: &str, x: char, map: &Vec<usize>) -> String {
    let mut chars: Vec<_> = scrambled.chars().collect();
    let index = scrambled.find(x).unwrap();
    chars.rotate_left(map[index]);
    chars.iter().collect()
}

fn unscramble(scrambled_password: &str, lines: &Vec<&str>, map: Vec<usize>) -> String {
    let mut result = scrambled_password.to_owned();
    lines.iter().rev().for_each(|&l| {
        let words = l.split_whitespace().collect::<Vec<_>>();
        match words[0] {
            "swap" => {
                if words[1] == "position" {
                    let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[5].parse().unwrap());
                    result = swap_positions(&result, y, x);
                    println!("swap positions {}", result);
                } else {
                    let (x, y) = (words[2].chars().next().unwrap(), words[5].chars().next().unwrap());
                    result = swap_letters(&result, y, x);
                    println!("swap letters {}", result);
                }
            },
            "rotate" => {
                match words[1] {
                    "left" => {
                        let x: usize = words[2].parse().unwrap();
                        result = rotate_by_steps(&result, x, false);
                        println!("rotate left {}", result);
                    },
                    "right" => {
                        let x: usize = words[2].parse().unwrap();
                        result = rotate_by_steps(&result, x, true);
                        println!("rotate right {}", result);
                    },
                    _ => {
                        println!("start rotating by letter {}", result);
                        let x = words[6].chars().next().unwrap();
                        result = unrotate_by_letter(&result, x, &map);
                        println!("rotate by letter {}", result);
                    },
                }
            },
            "reverse" => {
                let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[4].parse().unwrap());
                result = reverse(&result, x, y);
                println!("reverse {}", result);
            },
            _ => {
                let (x, y): (usize, usize) = (words[2].parse().unwrap(), words[5].parse().unwrap());
                result = move_by_positions(&result, y, x);
                println!("move {}", result);
            }
        }
    });
    result
}

fn main() {
    let input = utils::read_input();
    let lines = input.lines().collect::<Vec<_>>();
    let map = vec![1usize,1,6,2,7,3,0,4];
    assert_eq!(swap_positions(&swap_positions("cbeghdaf", 2, 5), 5, 2), "cbeghdaf");
    assert_eq!(swap_letters(&swap_letters("cbeghdaf", 'b', 'd'), 'd', 'b'), "cbeghdaf");
    assert_eq!(rotate_by_steps(&rotate_by_steps("cbeghdaf", 3, true), 3, false), "cbeghdaf");
    println!("done rotate_by_steps");
    assert_eq!(unrotate_by_letter(&rotate_by_letter("cbeghdaf", 'h'), 'h', &map), "cbeghdaf");
    println!("done rotate_by_letter");
    assert_eq!(reverse(&reverse("cbeghdaf", 3, 7), 3, 7), "cbeghdaf");
    assert_eq!(move_by_positions(&move_by_positions("cbeghdaf", 4, 6), 6, 4), "cbeghdaf");
    let password = "abcdefgh";
    one(password, &lines);
    let scrambled_password = "fbgdceah";
    two(scrambled_password, &lines, map);
}

fn one(password: &str, lines: &Vec<&str>) {
    println!("{}", scramble(password, lines));
}

fn two(scrambled_password: &str, lines: &Vec<&str>, map: Vec<usize>) {
    println!("{}", unscramble(scrambled_password, lines, map));
}