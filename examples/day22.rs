#![feature(conservative_impl_trait)]
extern crate aoc2017;
use std::collections::HashMap;

#[derive(Debug,Eq,PartialEq)]
enum Node {
    Infected,
    Clean,
}
use Node::*;

struct Grid {
    nodes: HashMap<(isize, isize), Node>,
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
    infections: usize,
}

fn parse_map(input: &str) -> Grid {
    let mut map = HashMap::new();
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let node =  match c {
                '.' => Clean,
                '#' => Infected,
                _ => panic!(),
            };
            map.insert((x as isize,y as isize), node);
        }
    }
    Grid {
        nodes: map,
        y: ((rows-1)/2) as isize,
        x: ((cols-1)/2) as isize,
        dx: 0,
        dy: -1,
        infections: 0,
    }
}

impl Grid {
    fn step(&mut self) {
        match *self.nodes.get(&(self.x, self.y)).unwrap_or(&Clean) {
            Infected => {
                let new_dx = -self.dy;
                let new_dy = self.dx;
                self.dx = new_dx;
                self.dy = new_dy;
                self.nodes.insert((self.x, self.y), Clean);
            }
            Clean => {
                let new_dx = self.dy;
                let new_dy = -self.dx;
                self.dx = new_dx;
                self.dy = new_dy;
                self.nodes.insert((self.x, self.y), Infected);
                self.infections += 1;
            }
        }
        self.x += self.dx;
        self.y += self.dy;
    }
}

fn solve1(input: &str) -> usize {
    let mut grid = parse_map(input);

    for _ in 0..10000 {
        grid.step();
    }
    grid.infections
}

fn main() {
    let input = aoc2017::get_input(22).unwrap();

    assert_eq!(solve1("..#\n#..\n..."), 5587);

    println!("Answer 1: {:?}", solve1(&input));
}
