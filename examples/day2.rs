#![feature(conservative_impl_trait)]
extern crate aoc2017;

fn solve(data: &[Vec<u64>]) -> u64 {
    data.iter()
        .map(|row| row.iter().max().unwrap() - row.iter().min().unwrap())
        .sum()
}

/*
fn solve2(data: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for_circular_opposites(data, |a, b| if a == b { sum += (a - b'0') as u64; });
    // We've only counted half
    sum * 2
}
*/

fn main() {
    let input = aoc2017::get_input(2).unwrap();
    let data = aoc2017::parse_rows::<u64>(&input);

    assert_eq!(solve(&aoc2017::parse_rows("5 1 9 5\n7 5 3\n2 4 6 8")), 18);

    println!("Answer part 1: {}", solve(&data));

    /*
    assert_eq!(solve2(b"1212"), 6);
    assert_eq!(solve2(b"1221"), 0);
    assert_eq!(solve2(b"123425"), 4);
    assert_eq!(solve2(b"12131415"), 4);

    println!("Answer part 2: {}", solve2(&input));
    */
}