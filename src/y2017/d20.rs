use std::ops::Add;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct ThreeDCell {
    x: i32,
    y: i32,
    z: i32,
}

impl Add for ThreeDCell {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl ThreeDCell {
    fn from(slice: &[i32]) -> ThreeDCell {
        ThreeDCell { x: slice[0], y: slice[1], z: slice[2] }
    }

    fn mht_dist(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone, Debug)]
struct Particle {
    p: ThreeDCell,
    v: ThreeDCell,
    a: ThreeDCell,
    dead: bool,
}

impl Particle {
    fn step(&mut self) {
        self.v = self.v + self.a;
        self.p = self.p + self.v;
    }

    fn dist(&self) -> i32 {
        self.p.mht_dist()
    }
}

pub fn main() {
    let input = aoc::read_input(module_path!());
    let mut particles = input.lines().map(|l| {
        let parts: Vec<_> = l.split_whitespace()
            .map(|p|
                p.split(&['<', ',', '>'][..]).skip(1).take(3)
                    .map(|n| n.parse::<i32>().unwrap())
                    .collect::<Vec<_>>())
            .collect();
        Particle { p: ThreeDCell::from(&parts[0]), v: ThreeDCell::from(&parts[1]), a: ThreeDCell::from(&parts[2]), dead: false }
    }).collect();
    one(&particles);
    two(&mut particles);
}

fn run(particles: &mut Vec<Particle>, collidable: bool) {
    let mut prev = 0;
    let mut equal_count = 0;
    let threshold = if collidable { 50 } else { 300 };
    loop {
        let mut min_dist = i32::MAX;
        let mut min_i = 0;
        for (i, ptc) in particles.iter_mut().enumerate() {
            if !ptc.dead {
                ptc.step();
                if min_dist > ptc.dist() {
                    min_dist = ptc.dist();
                    min_i = i;
                }
            }
        }

        if collidable {
            let len = particles.len();
            for i in 0..len-1 {
                let mut collided = false;
                for j in i+1..len {
                    if !particles[i].dead {
                        if particles[i].p == particles[j].p {
                            collided = true;
                            particles[j].dead = true;
                        }
                    }
                }
                if collided {
                    particles[i].dead = true;
                }
            }
            println!("left: {}", particles.iter().filter(|ptc| !ptc.dead).count());
        }

        if min_i == prev {
            equal_count += 1;
        } else {
            equal_count = 0;
            prev = min_i;
        }
        if equal_count > threshold { break; }
        println!("closest: {}", min_i);
    }
}

fn one(original: &Vec<Particle>) {
    let mut particles = original.clone();
    run(&mut particles, false);
}

fn two(particles: &mut Vec<Particle>) {
    run(particles, true);
}