enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

fn main() {
    let input = aoc::read_input();
    let moves: Vec<_> = input.trim().split(',').map(|s| {
        let chars: Vec<_> = s.chars().collect();
        let components = chars[1..].iter().collect::<String>();
        if chars[0] == 's' {
            Move::Spin(components.parse().unwrap())
        } else if chars[0] == 'x' {
            let parts: Vec<_> = components.split('/').map(|p| p.parse().unwrap()).collect();
            Move::Exchange(parts[0], parts[1])
        } else {
            let parts: Vec<_> = components.split('/').map(|p| p.chars().next().unwrap()).collect();
            Move::Partner(parts[0], parts[1])
        }
    }).collect();
    let mut programs: Vec<_> = "abcdefghijklmnop".chars().collect();
    one(&mut programs, &moves);
    two(&mut programs, &moves);
}

fn dance(programs: &mut Vec<char>, moves: &Vec<Move>) {
    moves.iter().for_each(|m| {
        match m {
            Move::Spin(x) => programs.rotate_right(*x),
            Move::Exchange(a, b) => programs.swap(*a, *b),
            Move::Partner(a, b) => {
                let a_pos = programs.iter().position(|p| *p == *a).unwrap();
                let b_pos = programs.iter().position(|p| *p == *b).unwrap();
                programs.swap(a_pos, b_pos);
            }
        }
    });
}

fn one(programs: &mut Vec<char>, moves: &Vec<Move>) {
    dance(programs, moves);
    for p in programs {
        print!("{}", p);
    }
    println!();
}

fn two(programs: &mut Vec<char>, moves: &Vec<Move>) {
    for i in 2..177 {
        dance(programs, moves);
        if programs.iter().collect::<String>() == "abcdefghijklmnop" {
            println!("{}", i);
        }
    }
    // the order is repeated every 44 times
    // 1000000000 % 44 = 32
    for _ in 1..33 {
        dance(programs, moves);
    }
    for p in programs {
        print!("{}", p);
    }
}