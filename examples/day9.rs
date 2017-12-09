#![feature(conservative_impl_trait)]
#[macro_use] extern crate aoc2017;

fn solve1(input: &str) -> usize {
    let mut cs = input.chars();
    let mut depth = 0;
    let mut score = 0;
    while let Some(c) = cs.next() {
        match c {
            '{' => { depth += 1; },
            '<' => {
                loop {
                    match cs.next().unwrap() {
                        '>' => { break; },
                        '!' => { cs.next().unwrap(); },
                        _ => {},
                    }
                }
            },
            '}' => {
                score += depth;
                depth -= 1;
            },
            _ => {},
        }
    }
    score
}

fn main() {
    let input = aoc2017::get_input(9).unwrap();

    assert_eq!(solve1(r"{}"), 1);
    assert_eq!(solve1(r"{{{}}}"), 6);
    assert_eq!(solve1(r"{{},{}}"), 5);
    assert_eq!(solve1(r"{{{},{},{{}}}}"), 16);
    assert_eq!(solve1(r"{<a>,<a>,<a>,<a>}"), 1);
    assert_eq!(solve1(r"{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    assert_eq!(solve1(r"{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
    assert_eq!(solve1(r"{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);

    println!("Answer 1: {:?}", solve1(&input));
}
