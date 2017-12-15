#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

fn solve(start_a: usize, start_b: usize) -> usize {
    let mut result = 0;
    let mut a: u64 = start_a as u64;
    let mut b: u64 = start_b as u64;
    for _ in 0..40_000_000 {
        a *= 16807;
        b *= 48271;
        while a > 0x7FFFFFFF {
            a = (a & 0x7FFFFFFF) + (a >> 31);
        }
        while b > 0x7FFFFFFF {
            b = (b & 0x7FFFFFFF) + (b >> 31);
        }
        if a >= 0x7FFFFFFF {
            println!("a={}", a);
        }
        if b >= 0x7FFFFFFF {
            println!("b={}", b);
        }

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
}
