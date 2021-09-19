fn main() {
    let line = aoc::read_input();

    let (mut orientation, mut x, mut y, mut new_x, mut new_y) = (0, 0, 0, 0, 0);
    let points = line.split(',').map(|p| {
        let p_trim = p.trim();
        let (turn, dist) = p_trim.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        x = new_x; y = new_y;

        orientation = match turn {
            "L" => if orientation == 0 {3} else {orientation-1},
            _ => if orientation == 3 {0} else {orientation+1},
        };
        let res: Vec<(i32, i32)> = match orientation {
            0 => {
                new_y = y + dist;
                (y+1 ..= new_y).map(|i| (x, i)).collect()
            },
            1 => {
                new_x = x + dist;
                (x+1 ..= new_x).map(|i| (i, y)).collect()
            },
            2 => {
                new_y = y - dist;
                (new_y .. y).map(|i| (x, i)).collect()
            },
            _ => {
                new_x = x - dist;
                (new_x .. x).map(|i| (i, y)).collect()
            },
        };
//         println!("{:?}", res);
        res
    }).flatten().collect();
    one(&points);
    two(&points);
}

fn one(points: &Vec<(i32, i32)>) {
    let (x, y) = points[points.len()-1];
    println!("{}", x.abs() + y.abs());
}

fn two(points: &Vec<(i32, i32)>) {
    for (i, p) in points.iter().enumerate() {
        let prevs = &points[..i];
        if (p.0 == 0 && p.1 == 0) || prevs.contains(p) {
            println!("{}", p.0.abs() + p.1.abs());
            return
        }
    }
}
