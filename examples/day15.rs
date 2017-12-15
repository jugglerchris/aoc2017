#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

fn next(start: u64, gen: u64) -> u64 {
    let mut result = start * gen;
    while result > 0x7FFFFFFF {
        result = (result & 0x7FFFFFFF) + (result >> 31);
    }
    if result == 0x7FFFFFFF {
        0
    } else {
        result
    }
}

fn solve(start_a: usize, start_b: usize) -> usize {
    let mut result = 0;
    let mut a: u64 = start_a as u64;
    let mut b: u64 = start_b as u64;
    for _ in 0..40_000_000 {
        a = next(a, 16807);
        b = next(b, 48271);

        if (a & 0xffff) == (b & 0xffff) {
           result += 1;
        }
    }
    result
}

// Calls next until val&mask == 0
fn next_mask(start: u64, gen: u64, mask: u64) -> u64 {
    let mut val = start;
    loop {
        val = next(val, gen);
        if (val & mask) == 0 {
            return val;
        }
    }
}

fn solve2(start_a: usize, start_b: usize) -> usize {
    let mut result = 0;
    let mut a: u64 = start_a as u64;
    let mut b: u64 = start_b as u64;
    for _ in 0..5_000_000 {
        a = next_mask(a, 16807, 0x3);
        b = next_mask(b, 48271, 0x7);

        if (a & 0xffff) == (b & 0xffff) {
           result += 1;
        }
    }
    result
}

fn main() {
    let input = aoc2017::get_input(15).unwrap();
    let mut lines = input.lines();
    let start_a: usize = lines.next().unwrap().split(' ').nth(4).unwrap().parse().unwrap();
    let start_b: usize = lines.next().unwrap().split(' ').nth(4).unwrap().parse().unwrap();

    assert_eq!(solve(65, 8921), 588);

    println!("Answer: {:?}", solve(start_a, start_b));

    assert_eq!(solve2(65, 8921), 309);

    println!("Answer: {:?}", solve2(start_a, start_b));
}
