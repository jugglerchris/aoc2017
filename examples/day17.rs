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

fn main() {
    let input = aoc2017::get_input(17).unwrap();

    assert_eq!(solve(3), 638);

    println!("Answer: {:?}", solve(input.parse().unwrap()));

//    assert_eq!(&solve2("s1,x3/4,pe/b", 5, 2), "ceadb");
//
//    println!("Answer: {:?}", solve2(&input, 16, 1000000000));
}
