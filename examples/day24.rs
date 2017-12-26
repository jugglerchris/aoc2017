#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::cell::Cell;
use std::cmp::max;

#[derive(Debug,Eq,PartialEq)]
struct Piece {
    port1: usize,
    port2: usize,
    used: Cell<bool>,
}

fn parse_piece(s: &str) -> Piece {
    let mut ports = s.split('/');
    let port1 = ports.next().unwrap().parse().unwrap();
    let port2 = ports.next().unwrap().parse().unwrap();
    Piece {
        port1: port1,
        port2: port2,
        used: Cell::new(false)
    }
}

fn solve(input: &str) -> (usize, usize) {
    let pieces = input.lines()
                      .map(parse_piece)
                      .collect::<Vec<Piece>>();
    let max_pins = pieces.iter()
                         .map(|p| max(p.port1, p.port2))
                         .max()
                         .unwrap();
    let mut piece_by_pins: Vec<Vec<&Piece>> = Vec::new();
    for _ in 0..(max_pins+1) {
        piece_by_pins.push(Vec::new());
    }
    for piece in &pieces {
        piece_by_pins[piece.port1].push(piece);
        piece_by_pins[piece.port2].push(piece);
    }
    let mut max_strength = 0;
    let mut max_strength_longest = 0;
    let mut longest = 0;
    let mut choices = Vec::new();
    let mut pin_stack = Vec::new();
    let mut piece_stack: Vec<&Piece> = Vec::new();
    let mut strength = 0;
    let mut length = 0;
    pin_stack.push(0);  // need to start with 0 pins
    choices.push(0);  // Next option with 0 pins
    while !pin_stack.is_empty() {
        let pins = pin_stack.pop().unwrap();
        let next_choice = choices.pop().unwrap();
        if next_choice >= piece_by_pins[pins].len() {
            // No more choices, so pop a piece off and continue
            if piece_stack.is_empty() {
                break;
            } else {
                let piece = piece_stack.pop().unwrap();
                assert!(piece.used.get(), true);
                piece.used.set(false);
                length -= 1;
                strength -= piece.port1 + piece.port2;
            }
        } else {
            let piece = piece_by_pins[pins][next_choice];
            choices.push(next_choice + 1);
            pin_stack.push(pins);
            if !piece.used.get() {
                piece_stack.push(piece);
                pin_stack.push(if piece.port1 == pins { piece.port2} else { piece.port1 });
                choices.push(0);
                piece.used.set(true);
                strength += piece.port1 + piece.port2;
                if strength > max_strength {
                    max_strength = strength;
                }

                length += 1;
                if length > longest {
                    longest = length;
                    max_strength_longest = strength;
                } else if (length == longest) && (strength > max_strength_longest) {
                    max_strength_longest = strength;
                }
            }
        }
    }

    (max_strength, max_strength_longest)
}

fn main() {
    let input = aoc2017::get_input(24).unwrap();

    assert_eq!(solve("0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10"), (31,19));
    println!("Answer 1,2: {:?}", solve(&input));
}
