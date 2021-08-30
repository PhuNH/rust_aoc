use aoc::utils;
use std::collections::HashMap;
use std::cell::RefCell;

#[derive(Clone, Copy)]
enum Target {
    Bot(u32),
    Output(u32),
}

impl Target {
    fn new(kind: &str, num: u32) -> Target {
        if kind == "bot" { Target::Bot(num) }
        else { Target::Output(num) }
    }
}

struct Bot {
    targets: [Option<Target>; 2],
    values: [Option<u32>; 2],
}

impl Bot {
    fn new() -> Bot {
        Bot { values: [None; 2], targets: [None; 2], }
    }

    fn with_targets(&mut self, low_target: Target, high_target: Target) {
        self.targets = [Some(low_target), Some(high_target)];
    }

    fn is_full(&self) -> bool {
        self.values.iter().all(|v| v.is_some())
    }

    fn take(&mut self, val: u32) {
        if self.values[0].is_none() { self.values[0] = Some(val) }
        else if self.values[1].is_none() { self.values[1] = Some(val) }
    }

    fn give(&mut self) -> (u32, u32) {
        assert!(self.is_full());
        let (first, second) = (self.values[0].unwrap(), self.values[1].unwrap());
        self.values = [None, None];
        (first.min(second), first.max(second))
    }
}

struct BotNet {
    bots: HashMap<u32, RefCell<Bot>>,
}

impl BotNet {
    fn new() -> BotNet {
        let bots = HashMap::new();
        BotNet { bots }
    }

    fn set_bot_targets(&mut self, num: u32, low_target: Target, high_target: Target) {
        let bot_cell = self.bots.entry(num).or_insert(RefCell::new(Bot::new()));
        bot_cell.borrow_mut().with_targets(low_target, high_target);
    }

    fn add_bot_val(&mut self, num: u32, val: u32) {
        let bot_cell = self.bots.entry(num).or_insert(RefCell::new(Bot::new()));
        bot_cell.borrow_mut().take(val);
    }

    fn proceed(&self, outputs: &mut [u32; 3]) {
        self.bots.values()
            .filter(|bot_cell| bot_cell.borrow().is_full())
            .for_each(|bot_cell| {
                let (low, high) = bot_cell.borrow_mut().give();
                match bot_cell.borrow().targets[0].unwrap() {
                    Target::Bot(num) => {
                        let target_cell = self.bots.get(&num).unwrap();
                        target_cell.borrow_mut().take(low);
                    },
                    Target::Output(num) => {
                        if num < 3 {
                            outputs[num as usize] = low;
                        }
                    }
                }
                match bot_cell.borrow().targets[1].unwrap() {
                    Target::Bot(num) => {
                        let target_cell = self.bots.get(&num).unwrap();
                        target_cell.borrow_mut().take(high);
                    },
                    Target::Output(num) => {
                        if num < 3 {
                            outputs[num as usize] = high;
                        }
                    }
                }
            })
    }

    fn find_values(&self, first: u32, second: u32) -> Option<u32> {
        self.bots.iter()
            .filter(|&(_, bot_cell)| bot_cell.borrow().is_full())
            .find(|&(_, bot_cell)| {
                let values: Vec<u32> = bot_cell.borrow().values.iter().map(|v| v.unwrap()).collect();
                values.contains(&first) && values.contains(&second)
            })
            .map(|(num, _)| *num)
    }
}

fn read_input(lines: Vec<&str>) -> BotNet {
    let mut bot_net = BotNet::new();
    lines.iter().cloned().for_each(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        if words[0] == "bot" {
            let v: Vec<u32> = [1, 6, 11].iter().map(|i| words[*i].parse().unwrap()).collect();
            let (num, low_target_num, high_target_num) = (v[0], v[1], v[2]);
            let low_target = Target::new(words[5], low_target_num);
            let high_target = Target::new(words[10], high_target_num);
            bot_net.set_bot_targets(num, low_target, high_target);
        } else {
            let v: Vec<u32> = [1, 5].iter().map(|i| words[*i].parse().unwrap()).collect();
            let (value, num) = (v[0], v[1]);
            bot_net.add_bot_val(num, value);
        }
    });
    bot_net
}

fn main() {
    let input = utils::read_input();
    let lines: Vec<&str> = input.lines().collect();
    let mut bot_net = read_input(lines);
    let mut outputs = [0; 3];
    one(&mut bot_net, &mut outputs);
    two(&mut bot_net, &mut outputs);
}

fn one(bot_net: &mut BotNet, outputs: &mut [u32; 3]) {
    let num = loop {
        bot_net.proceed(outputs);
        if let Some(n) = bot_net.find_values(61, 17) {
            break n;
        }
    };
    println!("{}", num);
}

fn two(bot_net: &mut BotNet, outputs: &mut [u32; 3]) {
    while outputs.iter().any(|o| *o == 0) {
        bot_net.proceed(outputs);
    }
    println!("{}", outputs.iter().product::<u32>());
}