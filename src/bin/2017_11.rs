use aoc::grid::TwoDCell;

const HEX_NORTH: TwoDCell = TwoDCell { x: 0, y: -2 };
const HEX_NE: TwoDCell = TwoDCell { x: 3, y: -1 };
const HEX_SE: TwoDCell = TwoDCell { x: 3, y: 1 };
const HEX_SOUTH: TwoDCell = TwoDCell { x: 0, y: 2 };
const HEX_SW: TwoDCell = TwoDCell { x: -3, y: 1 };
const HEX_NW: TwoDCell = TwoDCell { x: -3, y: -1 };

fn main() {
    let input = aoc::read_input();
    let instructions: Vec<_> = input.trim().split(",").collect();
    
    let mut furthest = 0;
    let last_coord = instructions.iter().fold(TwoDCell::from(0, 0), |acc, instr| {
        let coord = acc + match *instr {
            "n" => HEX_NORTH,
            "ne" => HEX_NE,
            "se" => HEX_SE,
            "s" => HEX_SOUTH,
            "sw" => HEX_SW,
            _ => HEX_NW,
        };
        let steps = steps(coord);
        if steps > furthest { furthest = steps; }
        coord
    });
    println!("{}", steps(last_coord));
    println!("{}", furthest);
}

fn steps(coord: TwoDCell) -> i32 {
    let x_steps = (coord.x / 3).abs();
    let mut y_steps = 0;
    if coord.y.abs() > x_steps {
        y_steps = (coord.y.abs() - x_steps) / 2;
    }
    x_steps + y_steps
}