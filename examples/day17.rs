#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

fn solve(step_size: usize) -> usize {
    let mut buffer = vec![0usize];
    let mut pos = 0;

    for i in 1..2018 {
        pos = (pos + step_size + 1) % buffer.len();
        buffer.insert(pos, i);
    }
    buffer[(pos+1)%buffer.len()]
}

fn solve2(step_size: usize) -> usize {
    let mut pos = 0;
    let mut buffer_len = 1;
    let mut result = 0;
    for i in 1..50_000_001 {
        pos = (pos + step_size) % buffer_len;
        if pos == 0 {
            result = i;
        }
        pos += 1;  // the mod will happen next time
        buffer_len += 1;
    }
    result
}

fn main() {
    let input = aoc2017::get_input(17).unwrap();

    assert_eq!(solve(3), 638);

    println!("Answer: {:?}", solve(input.parse().unwrap()));

    println!("Answer 2: {:?}", solve2(input.parse().unwrap()));
}
