#![feature(conservative_impl_trait)]
extern crate aoc2017;

// I solved this in my head originally; this is roughly the algorithm
// I used.
fn solve(input: u64) -> u64 {
    // find the next odd square up, and work out the distance from there.
    let sqrt = (input as f64).sqrt().ceil() as u64 | 1;
    let mut dist = sqrt - 1;  // distance from odd square corner
    let mut short = (sqrt * sqrt) - input;
    // Knock off any complete sides, which don't change the distance.
    if sqrt > 1 {
        short = short % (sqrt - 1);
    }
    let corner_to_middle = (sqrt - 1) / 2;
    if short > corner_to_middle {
        dist -= (sqrt-1) - short;
    } else {
        dist -= short;
    }
    dist
}

fn main() {
    let input:u64 = aoc2017::get_input(3).unwrap().parse().unwrap();

    assert_eq!(solve(1), 0);
    assert_eq!(solve(12), 3);
    assert_eq!(solve(23), 2);
    assert_eq!(solve(1024), 31);

    println!("Answer part 1: {}", solve(input));

    /*
    assert_eq!(solve2(&aoc2017::parse_rows("5 9 2 8\n9 4 7 3\n3 8 6 5\n")), 9);

    println!("Answer part 2: {}", solve2(&data));
    */
}