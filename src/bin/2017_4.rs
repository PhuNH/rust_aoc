fn main() {
    let input = aoc::read_input();
    let passphrases = input.lines().map(|l| l.trim().split_whitespace().collect::<Vec<_>>()).collect::<Vec<_>>();
    one(&passphrases);
    two(&passphrases);
}

fn one(passphrases: &Vec<Vec<&str>>) {
    let count = passphrases.iter().filter(|&l| {
        l.iter().enumerate().all(|(i, w)| {
            if i < l.len()-1 { !l[i+1..].contains(w) }
            else { true }
        })
    }).count();
    println!("{}", count);
}

fn two(passphrases: &Vec<Vec<&str>>) {
    let count = passphrases.iter().filter(|&l| {
        l.iter().enumerate().all(|(i, w)| {
            let mut sorted_w = w.chars().collect::<Vec<_>>();
            sorted_w.sort();
            let sorted_w = sorted_w.iter().collect::<String>();
            if i < l.len()-1 { l[i+1..].iter().all(|other| {
                let mut sorted_other = other.chars().collect::<Vec<_>>();
                sorted_other.sort();
                let sorted_other = sorted_other.iter().collect::<String>();
                sorted_other != sorted_w
            }) }
            else { true }
        })
    }).count();
    println!("{}", count);
}