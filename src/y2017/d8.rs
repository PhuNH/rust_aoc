use std::collections::HashMap;

pub fn main() {
    let input = aoc::read_input(module_path!());
    let mut registers = HashMap::new();
    let mut max = 0;
    input.lines().for_each(|l| process_instruction(l, &mut registers, &mut max));
    one(&registers);
    two(max);
}

fn process_instruction(instr: &str, registers: &mut HashMap<String, i32>, max: &mut i32) {
    // b inc 5 if a > 1
    // 0 1   2 3  4 5 6
    let words: Vec<_> = instr.split_whitespace().collect();
    let cond_reg = *registers.entry(words[4].to_string()).or_insert(0);
    let cond_val: i32 = words[6].parse().unwrap();
    if match words[5] {
        ">" => cond_reg > cond_val,
        ">=" => cond_reg >= cond_val,
        "<" => cond_reg < cond_val,
        "<=" => cond_reg <= cond_val,
        "==" => cond_reg == cond_val,
        _ => cond_reg != cond_val,
    } {
        let op_val: i32 = words[2].parse().unwrap();
        match words[1] {
            "inc" => *registers.entry(words[0].to_string()).or_insert(0) += op_val,
            _ => *registers.entry(words[0].to_string()).or_insert(0) -= op_val,
        }
        if *registers.get(words[0]).unwrap() > *max {
            *max = *registers.get(words[0]).unwrap();
        }
    }
}

fn one(registers: &HashMap<String, i32>) {
    println!("{}", registers.values().max().unwrap());
}

fn two(max: i32) {
    println!("{}", max);
}