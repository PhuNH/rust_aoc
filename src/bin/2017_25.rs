use std::collections::HashSet;

fn main() {
    let mut cursor = 0;
    let mut tape = HashSet::new();
    let mut step = 0;
    let mut state = 'a';
    run(&mut step, &mut state, &mut cursor, &mut tape);
    println!("{}", tape.len());
}

fn run(step: &mut u32, state: &mut char, cursor: &mut i32, tape: &mut HashSet<i32>) {
    loop {
        *step += 1;
        if *step > 12523873 { break; }
        match *state {
            'a' => {
                if tape.contains(cursor) {
                    *cursor -= 1;
                    *state = 'e';
                } else {
                    tape.insert(*cursor);
                    *cursor += 1;
                    *state = 'b';
                }
            }
            'b' => {
                if tape.contains(cursor) {
                    *cursor += 1;
                    *state = 'f';
                } else {
                    tape.insert(*cursor);
                    *cursor += 1;
                    *state = 'c';
                }
            }
            'c' => {
                if tape.contains(cursor) {
                    tape.remove(cursor);
                    *cursor += 1;
                    *state = 'b';
                } else {
                    tape.insert(*cursor);
                    *cursor -= 1;
                    *state = 'd';
                }
            }
            'd' => {
                if tape.contains(cursor) {
                    tape.remove(cursor);
                    *cursor -= 1;
                    *state = 'c';
                } else {
                    tape.insert(*cursor);
                    *cursor += 1;
                    *state = 'e';
                }
            }
            'e' => {
                if tape.contains(cursor) {
                    tape.remove(cursor);
                    *cursor += 1;
                    *state = 'd';
                } else {
                    tape.insert(*cursor);
                    *cursor -= 1;
                    *state = 'a';
                }
            }
            _ => {
                if tape.contains(cursor) {
                    *cursor += 1;
                    *state = 'c';
                } else {
                    tape.insert(*cursor);
                    *cursor += 1;
                    *state = 'a';
                }
            }
        }
    }
}