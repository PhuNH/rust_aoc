use aoc::grid::TwoDCell;

pub fn main() {
    let input: u32 = 368078;
    one(input);
    two(input);
}

fn square_width(n: u32) -> u32 {
    let mut width = 1;
    loop {
        if width * width >= n {
            break;
        }
        width += 2;
    }
    width
}

fn number_to_coord(n: u32) -> TwoDCell {
    let width = square_width(n) as i32;
    let edge = if width > 1 { (width * width - n as i32) / (width-1) } else { 0 };
    let dist_on_edge = if width > 1 { (width * width - n as i32) % (width-1) } else { 0 };
    match edge {
        0 => { TwoDCell::from(width/2 - dist_on_edge, width/2) }
        1 => { TwoDCell::from(-width/2, width/2 - dist_on_edge) }
        2 => { TwoDCell::from(dist_on_edge - width/2, -width/2) }
        _ => { TwoDCell::from(width/2, dist_on_edge - width/2) }
    }
}

fn coord_to_number(coord: TwoDCell) -> u32 {
    let (width, edge, dist_from_mid_edge) =
        if coord.x >= 0 && coord.y < coord.x && coord.y >= -coord.x { (coord.x * 2 + 1, 3, -coord.y) }
        else if coord.y < 0 && coord.x < -coord.y && coord.x >= coord.y { (-coord.y * 2 + 1, 2, -coord.x) }
        else if coord.x < 0 && coord.y <= -coord.x && coord.y > coord.x { (-coord.x * 2 + 1, 1, coord.y) }
        else { (coord.y * 2 + 1, 0, coord.x) };
    (width * width - edge * (width-1) - width/2 + dist_from_mid_edge) as u32
}

fn one(number: u32) {
    println!("{}", number_to_coord(number).mht_dist());
}

fn two(sum_threshold: u32) {
    let mut n = 2u32;
    let mut values = vec![1u32];
    let result = loop {
        let coord = number_to_coord(n);
        println!("{}", coord);
        let value: u32 = coord.neighbors_8().iter()
            .filter(|&neigh| coord_to_number(*neigh) < n)
            .map(|&neigh| values[(coord_to_number(neigh)-1) as usize]).sum();
        if value > sum_threshold {
            break value;
        }
        values.push(value);
        n += 1;
    };
    println!("{}", result);
}