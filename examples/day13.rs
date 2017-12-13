#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;
#[macro_use] extern crate lazy_static;

fn solve(input: &str) -> usize {
    let mut scanners: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(':');
        let depth = parts.next().map(str::parse).unwrap().unwrap();
        let range = parts.next().map(str::trim).map(str::parse).unwrap().unwrap();
        scanners.push((depth, range));
    }

    let mut severity = 0;
    for &(depth, range) in &scanners {
        if depth % ((range - 1) * 2) == 0 {
            severity += depth*range;
        }
    }
    severity
}

fn solve2(input: &str) -> usize {
    let mut scanners: Vec<(usize, usize)> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(':');
        let depth = parts.next().map(str::parse).unwrap().unwrap();
        let range = parts.next().map(str::trim).map(str::parse).unwrap().unwrap();
        scanners.push((depth, range));
    }

    let mut delay = 0;
    loop {
        if scanners.iter()
                   .any(|&(depth, range)|  (depth+delay) % ((range - 1) * 2) == 0)
        {
            delay += 1;
            continue;
        }
        return delay;
    }
}

fn main() {
    let input = aoc2017::get_input(13).unwrap();

    assert_eq!(solve(r#"0: 3
1: 2
4: 4
6: 4"#), 24);

    println!("Answer: {:?}", solve(&input));

    assert_eq!(solve2(r#"0: 3
1: 2
4: 4
6: 4"#), 10);

    println!("Answer: {:?}", solve2(&input));
}
