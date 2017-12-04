#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::collections::HashSet;

fn is_valid(words: &[&str]) -> bool {
    let mut set = HashSet::new();
    for word in words {
        set.insert(word);
    }
    set.len() == words.len()
}

fn solve(data: &[Vec<&str>]) -> usize {
    data.iter()
        .filter(|x| is_valid(x))
        .count()
}

/*
fn solve2(data: &[Vec<u64>]) -> u64 {
    data.iter()
        .map(find_divisible)
        .sum()
}
*/

fn main() {
    let input = aoc2017::get_input(4).unwrap();
    let data = aoc2017::parse_rows_str(&input);

    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "ee"]), true);
    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "aa"]), false);
    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "aaa"]), true);

    println!("Answer part 1: {}", solve(&data));

    /*
    assert_eq!(solve2(&aoc2017::parse_rows("5 9 2 8\n9 4 7 3\n3 8 6 5\n")), 9);

    println!("Answer part 2: {}", solve2(&data));
    */
}