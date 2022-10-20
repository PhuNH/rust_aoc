struct BufferedSpinlock {
    buffer: Vec<usize>,
    current: usize,
    steps: usize,
}

impl BufferedSpinlock {
    fn new(steps: usize) -> BufferedSpinlock {
        BufferedSpinlock { buffer: vec![0], current: 0, steps }
    }

    fn run_to(&mut self, to: usize) {
        for n in 1..=to {
            let next_index = (self.current + self.steps) % n + 1;
            self.buffer.insert(next_index, n);
            self.current = next_index;
        }
    }

    fn after_current(&self) -> usize {
        if self.current + 1 >= self.buffer.len() { self.buffer[0] }
        else { self.buffer[self.current + 1] }
    }
}

struct Spinlock {
    current: usize,
    steps: usize,
    one: usize,
}

impl Spinlock {
    fn new(steps: usize) -> Spinlock {
        Spinlock { current: 0, steps, one: 0 }
    }

    fn run_to(&mut self, to: usize) {
        for n in 1..=to {
            let next_index = (self.current + self.steps) % n + 1;
            if next_index == 1 {
                self.one = n;
                println!("{}", n);
            }
            self.current = next_index;
        }
    }
}

pub fn main() {
    let input = 354;
    one(input);
    two(input);
}

fn one(steps: usize) {
    let mut lock = BufferedSpinlock::new(steps);
    lock.run_to(2017);
    println!("After 2017: {}", lock.after_current());
}

fn two(steps: usize) {
    let mut lock = Spinlock::new(steps);
    lock.run_to(50000000);
    println!("After 0: {}", lock.one);
}