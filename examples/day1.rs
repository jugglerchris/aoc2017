#![feature(conservative_impl_trait)]
extern crate aoc2017;

fn for_circular_pairs<F: FnMut(u8, u8)>(bytes: &[u8], mut f: F)
{
    let l = bytes.len();
    for slc in bytes.windows(2) {
        f(slc[0], slc[1]);
    }
    f(bytes[l-1], bytes[0]);
}

fn for_circular_opposites<F: FnMut(u8, u8)>(bytes: &[u8], mut f: F)
{
    let l = bytes.len();
    let half = l/2;
    for i in 0..half {
        f(bytes[i], bytes[i+half]);
    }
}

fn solve(data: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for_circular_pairs(data, |a, b| if a == b { sum += (a - b'0') as u64; });
    sum
}

fn solve2(data: &[u8]) -> u64 {
    let mut sum: u64 = 0;
    for_circular_opposites(data, |a, b| if a == b { sum += (a - b'0') as u64; });
    // We've only counted half
    sum * 2
}

fn main() {
    let input = aoc2017::get_input(1).unwrap();
    let input = input.trim().as_bytes();

    assert_eq!(solve(b"1122"), 3);
    assert_eq!(solve(b"1111"), 4);
    assert_eq!(solve(b"1234"), 0);
    assert_eq!(solve(b"91212129"), 9);

    println!("Answer part 1: {}", solve(&input));

    assert_eq!(solve2(b"1212"), 6);
    assert_eq!(solve2(b"1221"), 0);
    assert_eq!(solve2(b"123425"), 4);
    assert_eq!(solve2(b"12131415"), 4);

    println!("Answer part 2: {}", solve2(&input));
}