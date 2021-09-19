fn main() {
    let input = aoc::read_input();

    one(&input);
    two(&input);
}

fn one(input: &String) {
    let mut count = 0;
    for l in input.lines() {
        let triangle: Vec<i32> = l.split_whitespace().map(|p| {
            p.trim().parse::<i32>().unwrap()
        }).collect();
        if is_possible(&triangle) {count += 1;}
    }
    println!("{}", count);
}

fn two(input: &String) {
    let mut count = 0;
    let mut triangles: Vec<Vec<i32>> = vec!(Vec::new(), Vec::new(), Vec::new());
    for (i, l) in input.lines().enumerate() {
        for (j, p) in l.split_whitespace().enumerate() {
            triangles[j].push(p.trim().parse::<i32>().unwrap());
        }
        if i > 0 && i % 3 == 2 {
            if is_possible(&triangles[0]) {count += 1;}
            triangles[0] = Vec::new();
            if is_possible(&triangles[1]) {count += 1;}
            triangles[1] = Vec::new();
            if is_possible(&triangles[2]) {count += 1;}
            triangles[2] = Vec::new();
        }
    }
    println!("{}", count);
}

fn is_possible(triangle: &Vec<i32>) -> bool {
    triangle[0] + triangle[1] > triangle[2] &&
    triangle[1] + triangle[2] > triangle[0] &&
    triangle[2] + triangle[0] > triangle[1]
}
