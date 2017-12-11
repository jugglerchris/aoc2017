#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::cmp::{min,max};

fn solve(input: &str) -> isize {
    let mut n: isize = 0;
    let mut ne: isize = 0;
    let mut se: isize = 0;

    for step in input.trim().split(',') {
        match step {
            "n" => { n += 1; },
            "s" => { n -= 1; },
            "ne" => { ne += 1; },
            "sw" => { ne -= 1; },
            "se" => { se += 1; },
            "nw" => { se -= 1; },
            _ => panic!(),
        };
    }
    // Let's lose the se/nw co-ordinate.  Each se step
    // is a ne step followed by an s step.
    ne += se;
    n -= se;
    se = 0;

    // Now if they have the same sign, just add them.
    if (ne*n) >= 0 {
        ne.abs() + n.abs()
    } else {
        let larger = max(ne.abs(), n.abs());
        let smaller = min(ne.abs(), n.abs());
        larger
    }
}

fn main() {
    let input = aoc2017::get_input(11).unwrap();

    assert_eq!(solve("ne,ne,ne"), 3);
    assert_eq!(solve("ne,ne,sw,sw"), 0);
    assert_eq!(solve("ne,ne,s,s"), 2);
    assert_eq!(solve("se,sw,se,sw,sw"), 3);

    println!("Answer: {:?}", solve(&input));
}
