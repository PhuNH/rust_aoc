fn main() {
    let input = aoc::read_input();
    let mut layers = Vec::new();
    input.lines().for_each(|l| {
        let depth_range: Vec<usize> = l.split(':').map(|p| p.trim().parse().unwrap()).collect();
        let layers_len = layers.len();
        if depth_range[0] > layers_len {
            for _ in layers_len..depth_range[0] {
                layers.push(0);
            }
        }
        layers.push(depth_range[1]);
    });
    one(&layers);
    two(&layers);
}

fn scanner_pos(range: usize, time: usize) -> usize {
    let turn = time / (range - 1);
    let pos_in_turn = time % (range - 1);
    if turn % 2 == 0 { pos_in_turn }
    else { range - 1 - pos_in_turn }
}

fn meet_scanners(layers: &Vec<usize>, delay: usize) -> Vec<usize> {
    layers.iter().enumerate().filter(|(depth, &range)| {
        range != 0 && scanner_pos(range, depth + delay) == 0
    }).map(|(depth, _)| depth).collect()
}

fn one(layers: &Vec<usize>) {
    println!("{}",
             meet_scanners(layers, 0).iter()
                 .map(|depth| *depth * layers[*depth])
                 .sum::<usize>());
}

fn two(layers: &Vec<usize>) {
    let mut delay = 1;
    loop {
        if meet_scanners(layers, delay).is_empty() { break; }
        delay += 1;
    }
    println!("{}", delay);
}