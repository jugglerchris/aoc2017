#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::collections::HashMap;

fn to_u64(pots: &[usize]) -> u64 {
    let mut result = 0;
    for v in pots {
        assert!(*v < 16);
        result = (result << 4) | (*v as u64);
    }
    return result;
}

fn solve(input: &str) -> (usize,usize) {
    let mut data = aoc2017::parse_rows::<usize>(input);
    let data = &mut data[0];
//    assert_eq!(data.len(), 16);
    let len = data.len();
    let mut count = 0;
    let mut seen = HashMap::new();
    let mut as_u64 = to_u64(&data);
    loop {
        seen.insert(as_u64, count);
        let mut max = 0;
        let mut max_pos = 0;
        for idx in 0..len {
            if data[idx] > max {
                max = data[idx];
                max_pos = idx;
            }
        }
        data[max_pos] = 0;
        for idx in (max_pos+1)..(max_pos+1+max) {
            data[idx % len] += 1;
        }
        count += 1;
        as_u64 = to_u64(&data);
        if let Some(lastcount) = seen.get(&as_u64) {
            return (count, count-lastcount);
        }
    }
}

fn main() {
    let input = aoc2017::get_input(6).unwrap();

    assert_eq!(solve("0 2 7 0"), (5, 4));

    let (a1, a2) = solve(&input);
    println!("Answer part 1: {}", a1);
    println!("Answer part 2: {}", a2);
}