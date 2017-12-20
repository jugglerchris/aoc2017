#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

use std::ops::{Add,AddAssign};
use std::collections::HashMap;

#[derive(Copy,Clone,Eq,PartialEq,Debug,Hash)]
struct MyVec(isize, isize, isize);

impl MyVec {
    // Return the Manhattan distance from <0,0,0>
    fn mag(&self) -> isize {
        self.0.abs() + self.1.abs() + self.2.abs()
    }
}

impl Add for MyVec {
    type Output = MyVec;
    fn add(self, rhs: Self) -> MyVec {
        MyVec(self.0 + rhs.0,
              self.1 + rhs.1,
              self.2 + rhs.2)
    }
}

impl AddAssign for MyVec {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

#[derive(Debug,Clone,Copy,Eq,PartialEq)]
struct Particle {
    p: MyVec,
    v: MyVec,
    a: MyVec,
}

impl Particle {
    fn update(&mut self) {
        self.v += self.a;
        self.p += self.v;
    }
}

regex_parser!(parse_particle: Particle {
    LINE = r#"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$"# =>
        | px: isize, py: isize, pz: isize,
          vx: isize, vy: isize, vz: isize,
          ax: isize, ay: isize, az: isize |
            Particle {
              p: MyVec(px, py, pz),
              v: MyVec(vx, vy, vz),
              a: MyVec(ax, ay, az),
            }
});

fn solve(input: &str) -> (usize, Particle) {
    let particles = input
            .lines()
            .map(parse_particle)
            .collect::<Vec<Particle>>();
    let (idx, smallest) = particles.iter()
                            .enumerate()
                            .min_by_key(|&(_, pcle)|
                                (pcle.a.mag(), pcle.v.mag(), pcle.p.mag()))
                            .unwrap();
    (idx, *smallest)
}

fn solve2(input: &str) -> usize {
    let mut particles = input
            .lines()
            .map(parse_particle)
            .collect::<Vec<Particle>>();

    let mut round = 0usize;
    loop {
        let mut positions: HashMap<MyVec, usize> = HashMap::new();
        for pcle in &mut particles {
            pcle.update();
            *positions.entry(pcle.p).or_insert(0) += 1;
        }

        let new_particles = particles.drain(0..)
                                     .filter(|pcle| *positions.get(&pcle.p).unwrap() == 1)
                                     .collect();
        particles = new_particles;
        println!("Round {}: {} particles left", round, particles.len());
        round += 1;
        // This is a terrible exit condition, which happens to have worked.
        if round > 100 {
            break;
        }
    }
    particles.len()
}

fn main() {
    let input = aoc2017::get_input(20).unwrap();

    assert_eq!(solve(r#"p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>"#),
        (0, Particle {
                p: MyVec(3,0,0),
                v: MyVec(2,0,0),
                a: MyVec(-1,0,0),
            }));

    println!("Answer 1: {:?}", solve(&input));

    assert_eq!(solve2(r#"p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>"#), 1);

    println!("Answer 2: {:?}", solve2(&input));
}
