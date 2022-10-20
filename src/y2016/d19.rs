fn play(count: u32) -> u32 {
    let mut positions = (1..=count).collect::<Vec<_>>();
    loop {
        let left_number = positions.len();
        positions = positions.into_iter().enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .map(|(_, p)| p)
            .collect();
        if positions.len() == 1 {
            break positions[0];
        }
        if left_number % 2 == 1 {
            positions.remove(0);
        }
    }
}

fn play_across(count: u32) -> u32 {
    let mut positions = (1..=count).collect::<Vec<_>>();
    let mut current = 0usize;

    loop {
        let mut len = positions.len();
        let dist = len / 2;
        let mut across = current + dist as usize;
        if across >= len { across -= len; }

        // println!("{}", positions[across]);
        positions.remove(across);
        len = positions.len();
        if len == 1 {
            break positions[0];
        }
        if across < current { current -= 1; }
        current += 1;
        if current >= len {
            current = 0;
        }
    }
}

pub fn main() {
    let input = 3012210u32;
    one(input);
    two(input);
}

fn one(input: u32) {
    println!("{}", play(input));
}

fn two(input: u32) {
    println!("{}", play_across(input));
}