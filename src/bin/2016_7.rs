use aoc::utils;

fn main() {
    let input = utils::read_input();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    one(&lines);
    two(&lines);
}

fn has_abba(s: &Vec<char>) -> bool {
    if s.len() < 4 { false }
    else {
        s[..(s.len()-3)].iter().enumerate().any(|(i, c)|
            s[i+3] == *c && s[i+1] == s[i+2] && s[i+1] != *c)
    }
}

fn supports_tls(l: &Vec<char>) -> bool {
    let s: String = l.iter().collect();
    let parts: Vec<&str> = s.split(&['[', ']'][..]).collect();
    parts.iter().skip(1).step_by(2).all(|s| !has_abba(&s.chars().collect()))
        && parts.iter().step_by(2).any(|s| has_abba(&s.chars().collect()))
}

fn supports_ssl(l: &Vec<char>) -> bool {
    let s: String = l.iter().collect();
    let parts: Vec<&str> = s.split(&['[', ']'][..]).collect();
    parts.iter().step_by(2).any(|s| {
        let v: Vec<char> = s.chars().collect();
        if v.len() < 3 { false }
        else {
            v[..(v.len()-2)].iter().enumerate().any(|(i, c)|
                v[i+2] == *c && v[i+1] != *c &&
                parts.iter().skip(1).step_by(2).any(|s_hypernet| {
                    let v_hypernet: Vec<char> = s_hypernet.chars().collect();
                    if v_hypernet.len() < 3 { false }
                    else {
                        v_hypernet[..(v_hypernet.len()-2)].iter().enumerate().any(|(i_hypernet, c_hypernet)|
                            *c_hypernet == v[i+1] && v_hypernet[i_hypernet+2] == *c_hypernet && v_hypernet[i_hypernet+1] == *c)
                    }
                })
            )
        }
    })
}

fn filter_func(lines: &Vec<Vec<char>>, closure: &impl Fn(&Vec<char>) -> bool) {
    let yes_no: Vec<u32> = lines.iter().map(|l| {
        match closure(l) {
            true => 1,
            _ => 0
        }
    }).collect();
    println!("{}", yes_no.iter().sum::<u32>());
}

fn one(lines: &Vec<Vec<char>>) {
    filter_func(lines, &supports_tls);
}

fn two(lines: &Vec<Vec<char>>) {
   filter_func(lines, &supports_ssl);
}