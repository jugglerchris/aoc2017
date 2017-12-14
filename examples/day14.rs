#![feature(conservative_impl_trait)]
extern crate aoc2017;

use aoc2017::Hasher;

fn solve(input: &str) -> usize {
    let mut bits_set = 0;
    for i in 0..128 {
        let mut hasher = Hasher::new_from_string(&format!("{}-{}", input, i));
        for _ in 0..64 {
            hasher.one_round();
        }
        bits_set += hasher.bits_set();
    }
    bits_set
}

fn main() {
    let input: String = aoc2017::get_input(14).unwrap().trim().into();

    assert_eq!(solve("flqrgnkx"), 8108);

    println!("Answer: {:?}", solve(&input));
}
