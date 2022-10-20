fn process_input(lines: Vec<&str>) -> Vec<Vec<u32>> {
    lines.into_iter().map(|l| {
        l.split('-').map(|s| s.parse().unwrap()).collect()
    }).collect()
}

pub fn main() {
    let input = aoc::read_input(module_path!());
    let lines = input.lines().collect::<Vec<_>>();
    let mut ranges = process_input(lines);
    ranges.sort_by_key(|p| p[0]);
    one(&ranges);
    two(&ranges);
}

fn one(ranges: &Vec<Vec<u32>>) {
    let lowest_range = ranges.iter().enumerate().find(|(i, r)| {
        r[1] + 1 < ranges[i+1][0]
    }).unwrap();
    println!("{}", lowest_range.1[1]+1);
}

fn two(ranges: &Vec<Vec<u32>>) {
    let mut count = 0;
    let mut max = 0;
    for i in 1..ranges.len() {
        if ranges[i-1][1] > max {
            max = ranges[i-1][1];
        }
        if ranges[i-1][1] == u32::MAX { break; }
        if ranges[i][0] > max + 1 {
            count += ranges[i][0] - (max + 1);
        }
    }
    if max < u32::MAX {
        count += u32::MAX - max;
    }
    println!("{}", count);
}