fn main() {
    let input = aoc::read_input();
    let banks: Vec<_> = input.lines().next().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
    one_and_two(banks.clone());
}

fn one_and_two(mut banks: Vec<u32>) {
    let mut states = vec![banks.clone()];
    let mut cycles = 0u32;
    let first_encounter = loop {
        cycles += 1;
        let most_value = banks.iter().max().unwrap();
        let most_index = banks.iter().position(|b| b == most_value).unwrap();
        let (rounds, residual) = (most_value / banks.len() as u32, most_value % banks.len() as u32);
        // print!("{} {} {} {} ", most_value, most_index, rounds, residual);
        banks[most_index] = 0;
        (0..banks.len()).for_each(|i| banks[i] += rounds);
        if residual > 0 {
            let mut residual_at_end = banks.len() as u32 - 1 - most_index as u32;
            let residual_at_start;
            if residual < residual_at_end {
                residual_at_end = residual;
                residual_at_start = 0;
            } else {
                residual_at_start = residual - residual_at_end;
            }
            if residual_at_end > 0 {
                (most_index+1..most_index+1+residual_at_end as usize).for_each(|i| banks[i] += 1);
            }
            if residual_at_start > 0 {
                (0..residual_at_start as usize).for_each(|i| banks[i] += 1);
            }
        }
        // println!("{:?}", banks);
        if let Some(first_encounter) = states.iter().position(|s| s.eq(&banks)) {
            break first_encounter;
        }
        states.push(banks.clone());
    };
    println!("{}, {}", cycles, cycles - first_encounter as u32);
}
