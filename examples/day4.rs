#![feature(conservative_impl_trait)]
extern crate aoc2017;

use std::collections::HashSet;
use std::convert::From;

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

fn is_valid2(words: &[&str]) -> bool {
    let mut set = HashSet::new();
    for word in words {
        let mut v: Vec<u8> = From::from(*word);
        v.sort();
        set.insert(String::from_utf8(v).unwrap());
    }
    set.len() == words.len()
}

fn solve2(data: &[Vec<&str>]) -> usize {
    data.iter()
        .filter(|x| is_valid2(x))
        .count()
}

fn main() {
    let input = aoc2017::get_input(4).unwrap();
    let data = aoc2017::parse_rows_str(&input);

    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "ee"]), true);
    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "aa"]), false);
    assert_eq!(is_valid(&["aa", "bb", "cc", "dd", "aaa"]), true);

    println!("Answer part 1: {}", solve(&data));

    assert_eq!(is_valid2(&["abcde", "fghij"]), true);
    assert_eq!(is_valid2(&["abcde", "xyz", "ecdab"]), false);
    assert_eq!(is_valid2(&["iiii", "oiii", "ooii", "oooi", "oooo"]), true);
    assert_eq!(is_valid2(&["oiii", "ioii", "iioi", "iiio"]), false);

    println!("Answer part 2: {}", solve2(&data));
}