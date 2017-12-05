#![feature(conservative_impl_trait)]
extern crate aoc2017;

fn solve(input: &str) -> isize {
    let mut data = aoc2017::parse_lines::<isize>(input);
    let mut count = 0;
    let mut pc = 0;
    loop {
        let newpc = pc + data[pc as usize];
        data[pc as usize] += 1;

        count += 1;
        if (newpc as usize) >= data.len() || newpc < 0 {
            return count;
        }
        pc = newpc;
    }
}

fn solve2(input: &str) -> isize {
    let mut data = aoc2017::parse_lines::<isize>(input);
    let mut count = 0;
    let mut pc = 0;
    loop {
        let offset = data[pc as usize];
        let newpc = pc + offset;

        let newoffset = if offset >= 3 { offset-1 } else { offset+1 };

        data[pc as usize] = newoffset;

        count += 1;
        if (newpc as usize) >= data.len() || newpc < 0 {
            return count;
        }
        pc = newpc;
    }
}
fn main() {
    let input = aoc2017::get_input(5).unwrap();

    assert_eq!(solve("0\n3\n0\n1\n-3\n"), 5);

    println!("Answer part 1: {}", solve(&input));

    assert_eq!(solve2("0\n3\n0\n1\n-3\n"), 10);

    println!("Answer part 2: {}", solve2(&input));
}