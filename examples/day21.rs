#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::collections::HashMap;

#[derive(Eq,PartialEq,Debug,Copy,Clone,Hash)]
enum Square {
    Empty, Full
}
use Square::*;

struct Rule {
    pat: Vec<Square>,
    output: Vec<Square>,
}

fn parse_rule(input: &str) -> Rule {
    let parts: Vec<&str> = input.split_whitespace()
                                .collect();
    let inpat = parts[0];
    let output = parts[2];
    Rule {
        pat: inpat.chars()
                  .filter_map(|c| match c {
                                     '.' => Some(Empty),
                                     '#' => Some(Full),
                                     '/' => None,
                                     _ => panic!(),
                                })
                  .collect(),
        output: output.chars()
                      .filter_map(|c| match c {
                                         '.' => Some(Empty),
                                         '#' => Some(Full),
                                         '/' => None,
                                         _ => panic!(),
                                    })
                      .collect(),
    }
}

fn print_grid(data: &[Square], size: usize) {
    for y in 0..size {
        println!("{}", (&data[y*size..(y+1)*size])
                           .iter()
                           .map(|x| match *x {
                                 Empty => '.',
                                 Full => '#',
                                })
                           .collect::<String>());
    }
}

fn rotate2<T: Copy>(input: &[T]) -> Vec<T> {
    vec![input[2], input[0], input[3], input[1]]
}
fn flip2<T: Copy>(input: &[T]) -> Vec<T> {
    vec![input[2], input[3], input[0], input[1]]
}
fn rotate3<T: Copy>(input: &[T]) -> Vec<T> {
    vec![input[6], input[3], input[0],
         input[7], input[4], input[1],
         input[8], input[5], input[2]]
}
fn flip3<T: Copy>(input: &[T]) -> Vec<T> {
    vec![input[2], input[1], input[0],
         input[5], input[4], input[3],
         input[8], input[7], input[6]]
}

type RuleMap = HashMap<Vec<Square>, Vec<Square>>;

struct Grid {
    data: Vec<Square>,
    size: usize,
}

impl Default for Grid {
    fn default() -> Grid {
        Grid {
            data: vec![Empty, Full,  Empty,
                       Empty, Empty, Full,
                       Full,  Full,  Full],
            size: 3,
        }
    }
}

impl Grid {
    fn step(&mut self, rules2: &RuleMap, rules3: &RuleMap) {
        //println!("step(size={}) data={}", self.size, self.data.len());
        let (step_size, rules) =  if (self.size % 2) == 0 {
            (2, rules2)
        } else {
            (3, rules3)
        };

        let num_squares = self.size / step_size;
        let mut new_data = Vec::new();
        let mut new_squares = Vec::new();
        for big_y in 0..num_squares {
            let y = big_y * step_size;
            new_squares.clear();
            for big_x in 0..num_squares {
                let x = big_x * step_size;
                let mut key = Vec::new();
                for i in 0..step_size {
                    let start = (y + i)*self.size + x;
                    //println!("size={} y={} x={} step_size={} i={}", self.size, y, x, step_size, i);
                    key.extend(&self.data[start..start+step_size]);
                }
                //println!("key={:?}", key);
                let new_square = rules.get(&key).unwrap().clone();
                //println!("   -> {:?}", new_square);
                new_squares.push(new_square);
            }

            /* Now copy the data to our output */
            let new_step_size = step_size+1;
            for i in 0..new_step_size {
                for v in &new_squares {
                    new_data.extend(&v[(i*new_step_size)..(i+1)*new_step_size]);
                }
            }
        }
        self.size += num_squares;
        self.data = new_data;
    }
    fn count_full(&self) -> usize {
        self.data
            .iter()
            .filter(|s| **s == Full)
            .count()

    }
    fn print(&self) {
        print_grid(&self.data, self.size);
    }
}

fn solve1(input: &str, iterations: usize) -> usize {
    let mut rules2 = HashMap::new();
    let mut rules3 = HashMap::new();
    for rule in input.lines()
                     .map(parse_rule)
                     .collect::<Vec<Rule>>() {
        if rule.pat.len() == 4 {
            //print_grid(&rule.pat, 2);
            //println!("  =>  ");
            //print_grid(&rule.output, 3);
            let mut pat = rule.pat;
            let mut output = rule.output;
            //println!("pat={:?}", pat);
            //print_grid(&pat, 2);
            rules2.insert(pat.clone(), output.clone());
            for _ in 0..3 {
                pat = rotate2(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 2);
                rules2.insert(pat.clone(), output.clone());
            }
            pat = flip2(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 2);
            rules2.insert(pat.clone(), output.clone());
            for _ in 0..3 {
                pat = rotate2(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 2);
                rules2.insert(pat.clone(), output.clone());
            }
        } else if rule.pat.len() == 9 {
            //print_grid(&rule.pat, 3);
            //println!("  =>  ");
            //print_grid(&rule.output, 4);
            let mut pat = rule.pat;
            let mut output = rule.output;
            //println!("pat={:?}", pat);
            //print_grid(&pat, 3);
            rules3.insert(pat.clone(), output.clone());
            for _ in 0..3 {
                pat = rotate3(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 3);
                rules3.insert(pat.clone(), output.clone());
            }
            pat = flip3(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 3);
            rules3.insert(pat.clone(), output.clone());
            for _ in 0..3 {
                pat = rotate3(&pat);
            //println!("pat={:?}", pat);
            //print_grid(&pat, 3);
                rules3.insert(pat.clone(), output.clone());
            }
        } else {
            panic!();
        }
    }

    let mut grid = Grid::default();
    for _ in 0..iterations {
        //println!("Current grid:");
        //grid.print();
        grid.step(&rules2, &rules3);
    }
    //grid.print();
    grid.count_full()
}

fn main() {
    let input = aoc2017::get_input(21).unwrap();

    assert_eq!(solve1(r#"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#"#, 2), 12);

    println!("Answer 1: {:?}", solve1(&input, 5));
    println!("Answer 2: {:?}", solve1(&input, 18));
}
